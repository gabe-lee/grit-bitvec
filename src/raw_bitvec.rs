use crate::{
    ptr,
    NonNull,
    alloc,
    Layout,
    BitUtil,
    RawBitVecIter,
    RawBitVecDrain,
    IdxProxy,
    BitProto,
    MemUtil,
    Range,
    ManuallyDrop,
    handle_alloc_error,
};

/// ## `RawBitVec`: "Raw Bitwise Vector"  
/// A `BitVec` where the bit-width and masking data ([`BitProto`]) must be manually passed to every function that accesses
/// any element or reallocates the underlying memory. 
/// 
/// ## Safety
/// The [`BitProto`] passed to the methods of any given instance of [`RawBitVec`] ***MUST*** be exactly the same as the very first one
/// pased to ***ANY*** of its methods that require it. For example, if you create a new instance with `RawBitVec::new()`,
/// no assumptions are made about the data within and any [`BitProto`] is valid. However, if you then use `RawBitVec::push(PROTO_3_BITS, 5)`,
/// you ***MUST*** pass `PROTO_3_BITS` for every other method call on this *specific instance* that requires a [`BitProto`], for its entire lifetime.
/// 
/// ### Pros
/// - Same stack-size as [`Vec`] (3 usize)
/// - Allows for constant-propogation optimizations (IF [`BitProto`] supplied to its methods is a constant)
/// - No mono-morphization (smaller binary)
/// - Can store [`RawBitVec`]'s in a homogenous collection (`Array`, [`Vec`], [`HashMap`](std::collections::HashMap), etc.)
/// 
/// ### Cons
/// - UNSAFE if the same [`BitProto`] isnt used for every method call on the same instance of a [`RawBitVec`]
/// - Clunky API (requires manually passing an extra variable and nearly all methods are unsafe)
/// - Cannot truly implement many traits because their signatures don't allow for a [`BitProto`] to be passed
///     - Simple psuedo-iterators are provided that *do* require the same [`BitProto`] to be passed
pub struct RawBitVec {
    pub(crate) ptr: NonNull<usize>,
    pub(crate) len: usize,
    pub(crate) true_cap: usize,
}

impl RawBitVec {
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub unsafe fn cap(&self, proto: BitProto) -> usize {
        BitProto::calc_bitwise_count_from_block_count(proto, self.true_cap)
    }

    #[inline]
    pub unsafe fn free(&self, proto: BitProto) -> usize {
        self.cap(proto) - self.len
    }

    #[inline]
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            true_cap: 0
        }
    }

    #[inline]
    pub fn with_capacity(proto: BitProto, cap: usize) -> Self {
        let mut new_vec = Self::new();
        let block_cap = BitProto::calc_block_count_from_bitwise_count(proto, cap);
        let (new_ptr, new_layout) = unsafe {Self::alloc_new(block_cap)};
        let new_non_null = Self::handle_alloc_result(new_layout, new_ptr);
        // new_vec.cap = BitProto::calc_bitwise_count_from_block_count(proto, block_cap);
        new_vec.ptr = new_non_null;
        new_vec.true_cap = block_cap;
        new_vec
    }

    #[inline]
    pub unsafe fn grow_exact_for_additional_elements_if_needed(&mut self, proto: BitProto, extra_elements: usize) -> Result<(), String> {
        if usize::MAX - extra_elements > self.len {
            return Err(format!("{} extra elements would overflow usize::MAX", extra_elements));
        }
        self.handle_grow_if_needed(proto, self.len + extra_elements, false)
    }

    #[inline]
    pub unsafe fn grow_exact_for_total_elements_if_needed(&mut self, proto: BitProto, total_elements: usize) -> Result<(), String> {
        self.handle_grow_if_needed(proto, total_elements, false)
    }

    #[inline]
    pub unsafe fn grow_for_additional_elements_if_needed(&mut self, proto: BitProto, extra_elements: usize) -> Result<(), String> {
        if usize::MAX - extra_elements > self.len {
            return Err(format!("{} extra elements would overflow usize::MAX", extra_elements));
        }
        self.handle_grow_if_needed(proto, self.len + extra_elements, true)
    }

    #[inline]
    pub unsafe fn grow_for_total_elements_if_needed(&mut self, proto: BitProto, total_elements: usize) -> Result<(), String> {
        self.handle_grow_if_needed(proto, total_elements, true)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0
    }

    #[inline]
    pub unsafe fn push(&mut self, proto: BitProto, val: usize) -> Result<(), String> {
        match self.len == proto.MAX_CAPACITY {
            true => Err(format!("BitVec is at maximum capacity ({})", proto.MAX_CAPACITY)),
            false => {
                self.handle_grow_if_needed(proto, self.len+1, true)?;
                self.push_unchecked(proto, val);
                Ok(())
            }
        }
    }

    #[inline]
    pub unsafe fn push_unchecked(&mut self, proto: BitProto, val: usize) {
        let len_proxy = BitProto::idx_proxy(proto, self.len);
        self.write_val_with_idx_proxy(len_proxy, val);
        self.len += 1;
    }

    #[inline]
    pub unsafe fn pop(&mut self, proto: BitProto) -> Result<usize, String> {
        if self.len == 0 {
            Err(format!("no elements in BitVec to pop out"))
        } else {
            Ok(self.pop_unchecked(proto))
        }
    }

    #[inline]
    pub unsafe fn pop_unchecked(&mut self, proto: BitProto) -> usize {
        self.len -= 1;
        let last_proxy = BitProto::idx_proxy(proto, self.len);
        self.replace_val_with_idx_proxy(last_proxy, 0)
    }

    #[inline]
    pub unsafe fn insert(&mut self, proto: BitProto, idx: usize, val: usize) -> Result<(), String> {
        if idx > self.len {
            return Err(format!("index out of bounds for insert: (idx) {} > {} (len)", idx, self.len));
        }
        if self.len == proto.MAX_CAPACITY {
            return Err(format!("BitVec is at maximum capacity ({})", proto.MAX_CAPACITY));
        }
        self.handle_grow_if_needed(proto, self.len+1, true)?;
        match idx == self.len {
            true => Ok(self.push_unchecked(proto, val)),
            false => Ok(self.insert_unchecked(proto, idx, val))
        }
    }

    #[inline]
    pub unsafe fn insert_unchecked(&mut self, proto: BitProto, idx: usize, val: usize) {
        let idx_proxy = BitProto::idx_proxy(proto, idx);
        self.shift_elements_up_with_with_idx_proxy(proto, idx_proxy, 1);
        self.write_val_with_idx_proxy(idx_proxy, val);
        self.len += 1;
    }

    #[inline]
    pub unsafe fn insert_bitvec(&mut self, proto: BitProto, insert_idx: usize, bitvec: Self) -> Result<(), String> {
        if insert_idx > self.len {
            return Err(format!("index out of bounds for insert_bitvec: (idx) {} > {} (len)", insert_idx, self.len));
        }
        if proto.MAX_CAPACITY - bitvec.len < self.len {
            return Err(format!("BitVec cannot hold {} more elements, {} elements would reach the maximum capacity ({})", bitvec.len, proto.MAX_CAPACITY - self.len, proto.MAX_CAPACITY));
        }
        self.handle_grow_if_needed(proto, self.len + bitvec.len, true)?;
        match insert_idx == self.len {
            true => self.append_bitvec_unchecked(proto, bitvec),
            false => self.insert_bitvec_unchecked(proto, insert_idx, bitvec)
        }
        Ok(())
    }

    #[inline]
    pub unsafe fn insert_bitvec_unchecked(&mut self, proto: BitProto, insert_idx: usize, bitvec: Self) {
        if bitvec.len > 0 {
            let begin_idx = BitProto::idx_proxy(proto, insert_idx);
            self.shift_elements_up_with_with_idx_proxy(proto, begin_idx, bitvec.len);
            self.len += bitvec.len;
            let mut count: usize = 0;
            while count < bitvec.len {
                let write_proxy = BitProto::idx_proxy(proto, insert_idx+count);
                let read_proxy = BitProto::idx_proxy(proto, count);
                let val = bitvec.read_val_with_idx_proxy(read_proxy);
                self.write_val_with_idx_proxy(write_proxy, val);
                count += 1;
            }
        }
    }

    #[inline]
    pub unsafe fn insert_iter<II, TO, ESI>(&mut self, proto: BitProto, insert_idx: usize, source: II) -> Result<(), String>
    where II: IntoIterator<Item = TO, IntoIter = ESI>, TO: ToOwned<Owned = usize>, ESI: ExactSizeIterator + Iterator<Item = TO> {
        if insert_idx > self.len {
            return Err(format!("index out of bounds for insert_iter: (idx) {} > {} (len)", insert_idx, self.len));
        }
        let iter = source.into_iter();
        if proto.MAX_CAPACITY - iter.len() < self.len {
            return Err(format!("BitVec cannot hold {} more elements, {} elements would reach the maximum capacity ({})", iter.len(), proto.MAX_CAPACITY - self.len, proto.MAX_CAPACITY));
        }
        self.handle_grow_if_needed(proto, self.len + iter.len(), true)?;
        if insert_idx == self.len {
            self.append_iter_unchecked(proto, iter);
        } else {
            self.insert_iter_unchecked(proto, insert_idx, iter);
        }
        Ok(())
    }

    #[inline]
    pub unsafe fn insert_iter_unchecked<II, TO, ESI>(&mut self, proto: BitProto, insert_idx: usize, source: II)
    where II: IntoIterator<Item = TO, IntoIter = ESI>, TO: ToOwned<Owned = usize>, ESI: ExactSizeIterator + Iterator<Item = TO> {
        let mut iter = source.into_iter();
        let iter_len = iter.len();
        if iter_len > 0 {
            let begin_idx = BitProto::idx_proxy(proto, insert_idx);
            self.shift_elements_up_with_with_idx_proxy(proto, begin_idx, iter_len);
            self.len += iter_len;
            let mut count = 0usize;
            while count < iter_len {
                let write_proxy = BitProto::idx_proxy(proto, insert_idx+count);
                let val = iter.next().unwrap();
                self.write_val_with_idx_proxy(write_proxy, val.to_owned());
                count += 1;
            }
        }
    }

    #[inline]
    pub unsafe fn remove(&mut self, proto: BitProto, idx: usize) -> Result<usize, String> {
        match idx >= self.len {
            true => Err(format!("index out of bounds for remove: (idx) {} >= {} (len)", idx, self.len)),
            false =>  match idx == self.len - 1 {
                true => Ok(self.pop_unchecked(proto)),
                false => Ok(self.remove_unchecked(proto, idx))
            },
        }
    }

    #[inline]
    pub unsafe fn remove_unchecked(&mut self, proto: BitProto, idx: usize) -> usize {
        let idx_proxy = BitProto::idx_proxy(proto, idx);
        let shift_proxy = BitProto::idx_proxy(proto, idx+1);
        let val = self.replace_val_with_idx_proxy(idx_proxy, 0);
        self.shift_elements_down_with_with_idx_proxy(proto, idx_proxy, shift_proxy, 1);
        self.len -= 1;
        val
    }

    #[inline]
    pub unsafe fn remove_range(&mut self, proto: BitProto, idx_range: Range<usize>) -> Result<Self, String> {
        match idx_range.end > self.len {
            true => Err(format!("index out of bounds for remove range: (idx) {} >= {} (len)", idx_range.end - 1, self.len)),
            false => Ok(self.remove_range_unchecked(proto, idx_range))
        }
    }

    #[inline]
    pub unsafe fn remove_range_unchecked(&mut self, proto: BitProto, idx_range: Range<usize>) -> Self {
        let count = idx_range.len();
        let mut new_vec = Self::with_capacity(proto, count);
        let start_proxy = BitProto::idx_proxy(proto, idx_range.start);
        let end_excluded_proxy = BitProto::idx_proxy(proto, idx_range.end);
        for idx in idx_range {
            let idx_proxy = BitProto::idx_proxy(proto, idx);
            let val = self.replace_val_with_idx_proxy(idx_proxy, 0);
            new_vec.push_unchecked(proto, val);
        }
        if end_excluded_proxy.bitwise_idx < self.len {
            self.shift_elements_down_with_with_idx_proxy(proto, start_proxy, end_excluded_proxy, count);
        }
        self.len -= count;
        new_vec
    }

    #[inline]
    pub unsafe fn swap(&mut self, proto: BitProto, idx_a: usize, idx_b: usize) -> Result<(), String> {
        if idx_a >= self.len || idx_b >= self.len {
            return Err(format!("index out of bounds for swap: (idx_a) {} >= {} (len) OR (idx_b) {} >= {} (len)", idx_a, self.len, idx_b, self.len))
        } else if idx_a != idx_b {
            self.swap_unchecked(proto, idx_a, idx_b);
        }
        Ok(())
    }

    #[inline]
    pub unsafe fn swap_unchecked(&mut self, proto: BitProto, idx_a: usize, idx_b: usize) {
        let proxy_a = BitProto::idx_proxy(proto, idx_a);
        let proxy_b = BitProto::idx_proxy(proto, idx_b);
        self.swap_vals_with_idx_proxy(proxy_a, proxy_b)
}

    #[inline]
    pub unsafe fn swap_pop(&mut self, proto: BitProto, idx: usize) -> Result<usize, String> {
        if idx >= self.len {
            Err(format!("index out of bounds for swap_pop: (idx) {} >= {} (len)", idx, self.len))
        } else if idx == self.len - 1 {
            Ok(self.pop_unchecked(proto))
        } else {
            Ok(self.swap_pop_unchecked(proto, idx))
        }
    }

    #[inline]
    pub unsafe fn swap_pop_unchecked(&mut self, proto: BitProto, idx: usize) -> usize {
        self.len -= 1;
        let last_proxy = BitProto::idx_proxy(proto, self.len);
        let pop_proxy = BitProto::idx_proxy(proto, idx);
        self.swap_pop_val_with_idx_proxy(pop_proxy, last_proxy)
    }

    #[inline]
    pub unsafe fn shrink_excess_capacity(&mut self, proto: BitProto, target_extra_capacity: usize) -> Result<(), String> {
        let target_capacity = self.len.saturating_add(target_extra_capacity);
        if target_capacity < self.cap(proto) {
            let target_block_capacity = BitProto::calc_block_count_from_bitwise_count(proto, target_capacity);
            let new_layout = MemUtil::usize_array_layout(target_block_capacity);
            let old_layout = MemUtil::usize_array_layout(self.true_cap);
            let new_ptr = alloc::realloc(self.ptr.as_ptr().cast(), old_layout, new_layout.size());
            let new_non_null = Self::handle_alloc_result(new_layout, new_ptr);
            self.true_cap = target_block_capacity;
            self.ptr = new_non_null;
        }
        Ok(())
    }

    #[inline]
    pub unsafe fn append_bitvec(&mut self, proto: BitProto, bitvec: Self) -> Result<(), String> {
        if proto.MAX_CAPACITY - bitvec.len < self.len {
            return Err(format!("BitVec cannot hold {} more elements, {} elements would reach the maximum capacity ({})", bitvec.len, proto.MAX_CAPACITY - self.len, proto.MAX_CAPACITY));
        }
        self.handle_grow_if_needed(proto, self.len + bitvec.len, true)?;
        self.append_bitvec_unchecked(proto, bitvec);
        Ok(())
    }

    #[inline]
    pub unsafe fn append_bitvec_unchecked(&mut self, proto: BitProto, bitvec: Self) {
        let mut count = 0;
        while count < bitvec.len {
                let write_proxy = BitProto::idx_proxy(proto, self.len+count);
                let read_proxy = BitProto::idx_proxy(proto, count);
                let val = bitvec.read_val_with_idx_proxy(read_proxy);
                self.write_val_with_idx_proxy(write_proxy, val);
                count += 1;
        }
        self.len += bitvec.len
    }

    #[inline]
    pub unsafe fn append_iter<II, TO, ESI>(&mut self, proto: BitProto, source: II) -> Result<(), String>
    where II: IntoIterator<Item = TO, IntoIter = ESI>, TO: ToOwned<Owned = usize>, ESI: ExactSizeIterator + Iterator<Item = TO> {
        let iter = source.into_iter();
        if proto.MAX_CAPACITY - iter.len() < self.len {
            return Err(format!("BitVec cannot hold {} more elements, {} elements would reach the maximum capacity ({})", iter.len(), proto.MAX_CAPACITY - self.len, proto.MAX_CAPACITY));
        }
        self.handle_grow_if_needed(proto, self.len + iter.len(), true)?;
        self.append_iter_unchecked(proto, iter);
        Ok(())
    }

    #[inline]
    pub unsafe fn append_iter_unchecked<II, TO, ESI>(&mut self, proto: BitProto, source: II)
    where II: IntoIterator<Item = TO, IntoIter = ESI>, TO: ToOwned<Owned = usize>, ESI: ExactSizeIterator + Iterator<Item = TO> {
        let iter = source.into_iter();
        for val in iter {
            self.push_unchecked(proto, val.to_owned())
        }
    }

    #[inline]
    pub unsafe fn get(&self, proto: BitProto, idx: usize) -> Result<usize, String> {
        match idx < self.len {
            true => Ok(self.get_unchecked(proto, idx)),
            false => Err(format!("index out of bounds for get: (idx) {} >= {} (len)", idx, self.len))
        }
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, proto: BitProto, idx: usize) -> usize {
        let idx_proxy = BitProto::idx_proxy(proto, idx);
        self.read_val_with_idx_proxy(idx_proxy)
    }

    #[inline]
    pub unsafe fn replace(&mut self, proto: BitProto, idx: usize, val: usize) -> Result<usize, String> {
        match idx < self.len {
            true => Ok(self.replace_unchecked(proto, idx, val)),
            false => Err(format!("index out of bounds for replace: (idx) {} >= {} (len)", idx, self.len))
        }
    }

    #[inline]
    pub unsafe fn replace_unchecked(&mut self, proto: BitProto, idx: usize, val: usize) -> usize {
        let idx_proxy = BitProto::idx_proxy(proto, idx);
        self.replace_val_with_idx_proxy(idx_proxy, val)
    }

    #[inline]
    pub unsafe fn set(&mut self, proto: BitProto, idx: usize, val: usize) -> Result<(), String> {
        match idx < self.len {
            true => Ok(self.set_unchecked(proto, idx, val)),
            false => Err(format!("index out of bounds for set: (idx) {} >= {} (len)", idx, self.len))
        }
    }

    #[inline]
    pub unsafe fn set_unchecked(&mut self, proto: BitProto, idx: usize, val: usize) {
        let idx_proxy = BitProto::idx_proxy(proto, idx);
        self.write_val_with_idx_proxy(idx_proxy, val);
    }

    #[inline]
    pub fn drain<'vec>(&'vec mut self) -> RawBitVecDrain<'vec> {
        let len = self.len;
        RawBitVecDrain {
            vec: self,
            start: 0,
            end_excluded: len,
        }
    }

    #[inline]
    pub fn into_iter(self) -> RawBitVecIter {
        let nodrop_self = ManuallyDrop::new(self);
        RawBitVecIter{
            ptr: nodrop_self.ptr,
            true_cap: nodrop_self.true_cap,
            start: 0,
            end_excluded: nodrop_self.len,
        }
    }

    #[inline]
    pub(crate) unsafe fn read_val_with_idx_proxy(&self, idx_proxy: IdxProxy) -> usize {
        let mut block_ptr = self.ptr.as_ptr().add(idx_proxy.real_idx);
        let mut block_bits = ptr::read(block_ptr);
        let mut val: usize = (block_bits & idx_proxy.first_mask) >> idx_proxy.first_offset;
        if idx_proxy.second_mask != 0 {
            block_ptr = block_ptr.add(1);
            block_bits = ptr::read(block_ptr);
            val = val | ((block_bits & idx_proxy.second_mask) << idx_proxy.second_offset);
        }
        val
    }

    #[inline]
    pub(crate) unsafe fn replace_val_with_idx_proxy(&mut self, idx_proxy: IdxProxy, new_val: usize) -> usize {
        let mut block_ptr = self.ptr.as_ptr().add(idx_proxy.real_idx);
        let mut block_bits = ptr::read(block_ptr);
        let mut val = (block_bits & idx_proxy.first_mask) >> idx_proxy.first_offset;
        block_bits = (block_bits & !idx_proxy.first_mask) | (new_val << idx_proxy.first_offset);
        ptr::write(block_ptr, block_bits);
        if idx_proxy.second_mask != 0 {
            block_ptr = block_ptr.add(1);
            block_bits = ptr::read(block_ptr);
            val = val | ((block_bits & idx_proxy.second_mask) << idx_proxy.second_offset);
            block_bits = (block_bits & !idx_proxy.second_mask) | (new_val >> idx_proxy.second_offset);
            ptr::write(block_ptr, block_bits);
        }
        val
    }

    #[inline]
    pub(crate) unsafe fn write_val_with_idx_proxy(&mut self, idx_proxy: IdxProxy, new_val: usize) {
        let mut block_ptr = self.ptr.as_ptr().add(idx_proxy.real_idx);
        let mut block_bits = ptr::read(block_ptr);
        block_bits = (block_bits & !idx_proxy.first_mask) | (new_val << idx_proxy.first_offset);
        ptr::write(block_ptr, block_bits);
        if idx_proxy.second_mask != 0 {
            block_ptr = block_ptr.add(1);
            block_bits = ptr::read(block_ptr);
            block_bits = (block_bits & !idx_proxy.second_mask) | (new_val >> idx_proxy.second_offset);
            ptr::write(block_ptr, block_bits);
        }
    }

    #[inline]
    pub(crate) unsafe fn swap_vals_with_idx_proxy(&mut self, proxy_a: IdxProxy, proxy_b: IdxProxy) {
        let val_a = self.replace_val_with_idx_proxy(proxy_a, 0);
        let val_b = self.replace_val_with_idx_proxy(proxy_b, 0);
        self.write_val_with_idx_proxy(proxy_a, val_b);
        self.write_val_with_idx_proxy(proxy_b, val_a);
    }

    #[inline]
    pub(crate) unsafe fn swap_pop_val_with_idx_proxy(&mut self, pop_proxy: IdxProxy, last_proxy: IdxProxy) -> usize {
        let val_last = self.replace_val_with_idx_proxy(last_proxy, 0);
        self.replace_val_with_idx_proxy(pop_proxy, val_last)
    }

    #[inline]
    pub(crate) unsafe fn shift_elements_up_with_with_idx_proxy(&mut self, proto: BitProto, begin_proxy: IdxProxy, shift_count: usize) {
        let real_len = BitProto::calc_block_count_from_bitwise_count(proto, self.len);
        let blocks_until_end = real_len - begin_proxy.real_idx;
        let final_real_len = BitProto::calc_block_count_from_bitwise_count(proto, self.len+shift_count);
        let end_excluded_block_ptr = self.ptr.as_ptr().add(final_real_len);
        let mut block_ptr = self.ptr.as_ptr().add(begin_proxy.real_idx);
        let mut block_bits = ptr::read(block_ptr);
        let keep_first_mask = BitUtil::all_bits_less_than_bit(begin_proxy.first_offset);
        let keep_first_bits = block_bits & keep_first_mask;
        block_bits &= !keep_first_mask;
        ptr::write(block_ptr, block_bits);
        let total_bits_shifted = shift_count * proto.BITS;
        let whole_blocks = total_bits_shifted / BitUtil::USIZE_BITS;
        let rollover_bits = total_bits_shifted - (whole_blocks * BitUtil::USIZE_BITS);
        if whole_blocks > 0 {
            ptr::copy(block_ptr, block_ptr.add(whole_blocks), blocks_until_end);
            block_ptr = block_ptr.add(whole_blocks);
        }
        if rollover_bits > 0 {
            let rollover_shift = BitUtil::USIZE_BITS - rollover_bits;
            let rollover_mask = usize::MAX << rollover_shift;
            let mut rollover_bits_paste: usize = 0; 
            let mut rollover_bits_copy: usize; 
            while block_ptr < end_excluded_block_ptr {
                block_bits = ptr::read(block_ptr);
                rollover_bits_copy = (block_bits & rollover_mask) >> rollover_shift;
                block_bits = (block_bits << rollover_bits) | rollover_bits_paste;
                ptr::write(block_ptr, block_bits);
                block_ptr = block_ptr.add(1);
                rollover_bits_paste = rollover_bits_copy;
            }
        }
        block_ptr = self.ptr.as_ptr().add(begin_proxy.real_idx);
        block_bits = ptr::read(block_ptr);
        ptr::write(block_ptr, block_bits | keep_first_bits);
    }

    #[inline]
    pub(crate) unsafe fn shift_elements_down_with_with_idx_proxy(&mut self, proto: BitProto, begin_proxy: IdxProxy, shift_proxy: IdxProxy, shift_count: usize) {
        let real_len = BitProto::calc_block_count_from_bitwise_count(proto, self.len);
        let block_count = real_len - shift_proxy.real_idx;
        let mut block_ptr = self.ptr.as_ptr().add(begin_proxy.real_idx);
        let mut block_bits = ptr::read(block_ptr);
        let keep_first_mask = BitUtil::all_bits_less_than_bit(begin_proxy.first_offset);
        let keep_first_bits = block_bits & keep_first_mask;
        block_bits &= !keep_first_mask;
        ptr::write(block_ptr, block_bits);
        let total_bits_shifted = shift_count * proto.BITS;
        let whole_blocks = total_bits_shifted / BitUtil::USIZE_BITS;
        if whole_blocks > 0 {
            ptr::copy(block_ptr.add(whole_blocks), block_ptr, block_count);
        }
        let rollover_bits = total_bits_shifted - (whole_blocks * BitUtil::USIZE_BITS);
        if rollover_bits > 0 {
            let rollover_shift = BitUtil::USIZE_BITS - rollover_bits;
            let rollover_mask = usize::MAX >> rollover_shift;
            let mut blocks_shifted = 0;
            let mut rollover_bits_paste: usize = 0; 
            let mut rollover_bits_copy: usize; 
            block_ptr = self.ptr.as_ptr().add(real_len-whole_blocks);
            while blocks_shifted < block_count {
                block_ptr = block_ptr.sub(1);
                block_bits = ptr::read(block_ptr);
                rollover_bits_copy = (block_bits & rollover_mask) << rollover_shift;
                block_bits = (block_bits >> rollover_bits) | rollover_bits_paste;
                ptr::write(block_ptr, block_bits);
                blocks_shifted += 1;
                rollover_bits_paste = rollover_bits_copy;
            }
        }
        block_bits = ptr::read(block_ptr);
        ptr::write(block_ptr, block_bits | keep_first_bits);
    }
    
    #[inline]
    pub(crate) unsafe fn handle_grow_if_needed(&mut self, proto: BitProto, min_capacity: usize, grow_exponential: bool) -> Result<(), String> {
        let true_min_capacity = BitProto::calc_block_count_from_bitwise_count(proto, min_capacity);
        if true_min_capacity > self.true_cap{
            let new_true_cap = match grow_exponential {
                true => true_min_capacity.saturating_add(true_min_capacity >> 1),
                false => true_min_capacity,
            };
            let new_layout: Layout = MemUtil::usize_array_layout(new_true_cap);
            let new_ptr = match self.true_cap {
                0 => {
                    alloc::alloc(new_layout)
                },
                _ => {
                    let old_layout = MemUtil::usize_array_layout(self.true_cap);
                    alloc::realloc(self.ptr.as_ptr().cast(), old_layout, new_layout.size())
                },
            };
            let new_non_null = Self::handle_alloc_result(new_layout, new_ptr);
            self.true_cap = new_true_cap;
            self.ptr = new_non_null;
            Ok(())
        } else {
            Ok(())
        }
    }

    #[inline]
    pub(crate) unsafe fn alloc_new(real_cap: usize) -> (*mut u8, Layout) {
        let new_layout: Layout = MemUtil::usize_array_layout(real_cap);
        let new_ptr = alloc::alloc(new_layout);
        (new_ptr, new_layout)
    }

    #[inline]
    pub(crate) fn handle_alloc_result(alloc_layout: Layout, alloc_result_ptr: *mut u8) -> NonNull<usize> {
        match NonNull::new(alloc_result_ptr) {
            Some(non_null) => non_null.cast::<usize>(),
            None => handle_alloc_error(alloc_layout)
        }
    }
}

impl Drop for RawBitVec {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if self.true_cap > 0 {
                let layout = MemUtil::usize_array_layout(self.true_cap);
                alloc::dealloc(self.ptr.as_ptr().cast(), layout)
            }
        }
    }
}

#[cfg(debug_assertions)]
impl RawBitVec {
    #[allow(dead_code)]
    pub(crate) fn debug_string(&self, proto: BitProto) -> String {
        let true_len = BitProto::calc_block_count_from_bitwise_count(proto, self.len);
        let elem_cap = BitProto::calc_bitwise_count_from_block_count(proto, self.true_cap);
        let mut output = format!("elem len = {}\nelem cap = {}\ntrue len = {}\ntrue cap = {}\ndata: \n", self.len, elem_cap, true_len, self.true_cap);
        let mut i = 0usize;
        while i < true_len {
            let block = unsafe{ptr::read(self.ptr.as_ptr().add(i))};
            output.push_str(format!("{} = {:064b}\n", i, block).as_str());
            i += 1;
        }
        output
    }
}