use crate::{
    mem,
    align_of,
    size_of,
    needs_drop,
    ptr,
    NonNull,
    PhantomData,
    alloc,
    Layout,
    MemUtil,
    BitUtil,
    BVecIter,
    BVecDrain,
    BitElem,
    ElementCount,
    Grow,
    Shrink,
    Resize
};
pub struct BitVec<ELEM>
where ELEM: BitElem {
    pub(crate) ptr: NonNull<usize>,
    pub(crate) len: usize,
    pub(crate) cap: usize,
    pub(crate) sub: PhantomData<ELEM>
}

impl<ELEM> BitVec<ELEM>
where ELEM: BitElem {
    pub(crate) const DEFAULT_GROW: Grow = Grow::OnePointFive;

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn cap(&self) -> usize {
        self.cap
    }

    #[inline]
    pub fn free(&self) -> usize {
        self.cap - self.len
    }

    #[inline]
    pub fn new() -> Self {
        if ELEM::BITS > 0 {
            Self {
                ptr: NonNull::dangling(),
                cap: 0,
                len: 0,
                sub: PhantomData,
            }
        } else {
            Self {
                ptr: NonNull::dangling(),
                cap: usize::MAX,
                len: 0,
                sub: PhantomData,
            }
        }
    }

    #[inline]
    pub fn with_capacity(cap: usize) -> Result<Self, String> {
        if ELEM::BITS > 0 {
            let mut new_vec = Self::new();
            unsafe{new_vec.handle_resize(ElementCount::Total(cap), Resize::ExactCapacity(cap), true)}?;
            Ok(new_vec)
        } else {
            Ok(Self {
                ptr: NonNull::dangling(),
                cap,
                len: 0,
                sub: PhantomData,
            })
        }
    }

    #[inline]
    pub fn grow_if_needed(&mut self, elem_count: ElementCount) -> Result<(), String> {
        self.grow_if_needed_custom(elem_count, Self::DEFAULT_GROW)
    }

    #[inline]
    pub fn grow_if_needed_custom(&mut self, elem_count: ElementCount, grow: Grow) -> Result<(), String> {
        unsafe {self.handle_resize(elem_count, Resize::Grow(grow), false)}
    }

    #[inline]
    pub fn clear(&mut self) {
        if needs_drop::<ELEM::Base>() {
            while self.len > 0 {
                let _ = unsafe {self.pop_unchecked()};
            }
        } else {
            self.len = 0
        }
    }

    #[inline]
    pub fn push(&mut self, val: ELEM::Base) -> Result<(), String> {
        self.push_custom_grow(val, Self::DEFAULT_GROW)
    }

    #[inline]
    pub fn push_custom_grow(&mut self, val: ELEM::Base, grow: Grow) -> Result<(), String> {
        self.grow_if_needed_custom(ElementCount::Additional(1), grow)?;
        unsafe {self.push_unchecked(val)};
        Ok(())
    }

    #[inline]
    pub unsafe fn push_unchecked(&mut self, val: ELEM::Base) {
        if ELEM::BITS > 0 {
            let (real_idx, bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(self.len);
            let val_bits = ELEM::val_to_bits(val);
            let this_block_bits_to_push = val_bits << bit_off;
            let next_block_bits_to_push = val_bits >> (BitUtil::USIZE_BITS - bit_off);
            let mut block_ptr = self.ptr.as_ptr().add(real_idx);
            let mut block_bits = ptr::read(block_ptr);
            block_bits = (block_bits & BitUtil::zero_mask_if_bit_offset_is_zero(bit_off)) | this_block_bits_to_push;
            ptr::write(block_ptr, block_bits);
            if next_block_bits_to_push > 0 {
                block_ptr = block_ptr.add(1);
                block_bits = next_block_bits_to_push;
                ptr::write(block_ptr, block_bits);
            }
        }
        self.len += 1;
    }

    #[inline]
    pub fn pop(&mut self) -> Result<ELEM::Base, String> {
        if self.len == 0 {
            Err(format!("no elements in BitVec to pop out!"))
        } else {
            Ok(unsafe{self.pop_unchecked()})
        }
    }

    #[inline]
    pub unsafe fn pop_unchecked(&mut self) -> ELEM::Base {
        self.len -= 1;
        if ELEM::BITS > 0 {
            let (real_idx, bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(self.len);
            let bit_diff = BitUtil::USIZE_BITS - bit_off;
            let this_block_mask_to_read = ELEM::MASK << bit_off;
            let next_block_mask_to_read = ELEM::MASK >> bit_diff;
            let mut block_ptr = self.ptr.as_ptr().add(real_idx);
            let mut block_bits = ptr::read(block_ptr);
            let mut val_bits: usize = (block_bits & this_block_mask_to_read) >> bit_off;
            block_bits &= !this_block_mask_to_read;
            ptr::write(block_ptr, block_bits);
            if next_block_mask_to_read > 0 {
                block_ptr = block_ptr.add(1);
                block_bits = ptr::replace(block_ptr, 0);
                val_bits |= (block_bits & next_block_mask_to_read) << bit_diff;
            }
            ELEM::bits_to_val(val_bits)
        } else {
            ELEM::bits_to_val(0)
        }
    }

    #[inline]
    pub fn insert(&mut self, idx: usize, val: ELEM::Base) -> Result<(), String> {
        self.insert_custom_grow(idx, val, Self::DEFAULT_GROW)
    }

    #[inline]
    pub fn insert_custom_grow(&mut self, idx: usize, val: ELEM::Base, grow: Grow) -> Result<(), String> {
        if idx > self.len {
            return Err(format!("index out of bounds for insert:\n\tlen = {}\n\tidx = {}", self.len, idx));
        }
        self.grow_if_needed_custom(ElementCount::Additional(1), grow)?;
        Ok(unsafe {self.insert_unchecked(idx, val)})
    }

    #[inline]
    pub unsafe fn insert_unchecked(&mut self, idx: usize, val: ELEM::Base) {
        if ELEM::BITS > 0 {
            let (mut last_idx, last_bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(self.len);
            last_idx = Self::calc_end_real_idx_from_start_real_idx_and_bit_offset(last_idx, last_bit_off);
            let (insert_idx, insert_bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(idx);
            let mut block_ptr = self.ptr.as_ptr().add(insert_idx);
            let mut block_bits = ptr::read(block_ptr);
            let keep_first_mask = BitUtil::all_bits_less_than_bit(insert_bit_off);
            let keep_first_bits = block_bits & keep_first_mask;
            block_bits &= !keep_first_mask;
            ptr::write(block_ptr, block_bits);
            let rollover_shift = BitUtil::USIZE_BITS - ELEM::BITS;
            let rollover_mask = ELEM::MASK << rollover_shift;
            let mut start_idx = insert_idx;
            let val_bits = ELEM::val_to_bits(val) << insert_bit_off;
            let mut rollover_bits_paste: usize = keep_first_bits | val_bits; 
            let mut rollover_bits_copy: usize; 
            let should_purge_last_index = last_bit_off == 0;
            while start_idx <= last_idx {
                let purge_last_index_mask = !BitUtil::smear_left((start_idx == last_idx && should_purge_last_index) as usize);
                block_bits = ptr::read(block_ptr) & purge_last_index_mask;
                rollover_bits_copy = (block_bits & rollover_mask) >> rollover_shift;
                block_bits = (block_bits << ELEM::BITS) | rollover_bits_paste;
                ptr::write(block_ptr, block_bits);
                block_ptr = block_ptr.add(1);
                start_idx += 1;
                rollover_bits_paste = rollover_bits_copy;
            }
        }
        self.len += 1;
    }

    #[inline]
    pub fn remove(&mut self, idx: usize) -> Result<ELEM::Base, String> {
        match idx >= self.len {
            true => Err(format!("index out of bounds for remove:\n\tlen = {}\n\tidx = {}", self.len, idx)),
            false => Ok(unsafe{self.remove_unchecked(idx)}),
        }
    }

    #[inline]
    pub unsafe fn remove_unchecked(&mut self, idx: usize) -> ELEM::Base {
        self.len -= 1;
        if ELEM::BITS > 0 {
            let (mut last_idx, last_bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(self.len);
            last_idx = Self::calc_end_real_idx_from_start_real_idx_and_bit_offset(last_idx, last_bit_off);
            let (remove_idx, remove_bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(idx);
            let mut block_ptr = self.ptr.as_ptr().add(remove_idx);
            let mut block_bits = ptr::read(block_ptr);
            let keep_first_mask = BitUtil::all_bits_less_than_bit(remove_bit_off);
            let keep_first_bits = block_bits & keep_first_mask;
            let remove_mask = ELEM::MASK << remove_bit_off;
            let val_bits = (block_bits & remove_mask) >> remove_bit_off;
            block_bits &= !(remove_mask | keep_first_mask);
            ptr::write(block_ptr, block_bits);
            let rollover_shift = BitUtil::USIZE_BITS - ELEM::BITS;
            let mut curr_idx = last_idx;
            let mut rollover_bits_last: usize = 0; 
            let mut rollover_bits_this: usize; 
            block_ptr = self.ptr.as_ptr().add(last_idx);
            while curr_idx >= remove_idx {
                block_bits = ptr::read(block_ptr);
                rollover_bits_this = (block_bits & ELEM::MASK) << rollover_shift;
                block_bits = (block_bits >> ELEM::BITS) | rollover_bits_last;
                ptr::write(block_ptr, block_bits);
                block_ptr = block_ptr.sub(1);
                curr_idx -= 1 ;
                rollover_bits_last = rollover_bits_this;
            }
            block_ptr = block_ptr.add(1);
            block_bits |= keep_first_bits;
            ptr::write(block_ptr, block_bits);
            ELEM::bits_to_val(val_bits)
        } else {
            ELEM::bits_to_val(0)
        }
    }

    #[inline]
    pub fn swap(&mut self, idx_a: usize, idx_b: usize) -> Result<(), String> {
        if idx_a >= self.len || idx_b >= self.len {
            return Err(format!("index out of bounds for swap:\n\tlen   = {}\n\tidx_a = {}, idx_b = {}", self.len, idx_a, idx_b))
        } else if idx_a != idx_b {
            unsafe {self.swap_unchecked(idx_a, idx_b)};
        }
        Ok(())
    }

    #[inline]
    pub unsafe fn swap_unchecked(&mut self, idx_a: usize, idx_b: usize) {
        if ELEM::BITS > 0 {
            let (real_a, off_a) = Self::calc_sub_idx_to_real_idx_and_bit_offset(idx_a);
            let (real_b, off_b) = Self::calc_sub_idx_to_real_idx_and_bit_offset(idx_b);
            let mask_a = ELEM::MASK << off_a;
            let mask_b = ELEM::MASK << off_b;
            let ptr_a = self.ptr.as_ptr().add(real_a);
            let ptr_b = self.ptr.as_ptr().add(real_b);
            let val_a = (ptr::read(ptr_a) & mask_a) >> off_a;
            let val_b = (ptr::read(ptr_b) & mask_b) >> off_b;
            let block_a = ptr::read(ptr_a) & !mask_a;
            ptr::write(ptr_a, block_a | (val_b << off_a));
            let block_b = ptr::read(ptr_b) & !mask_b;
            ptr::write(ptr_b, block_b | (val_a << off_b));
        }
    }

    #[inline]
    pub fn swap_pop(&mut self, idx: usize) -> Result<ELEM::Base, String> {
        if idx >= self.len {
            Err(format!("index out of bounds for swap pop:\n\tlen   = {}\n\tidx = {}", self.len, idx))
        } else if idx == self.len - 1 {
            Ok(unsafe{self.pop_unchecked()})
        } else {
            Ok(unsafe {self.swap_pop_unchecked(idx)})
        }
    }

    #[inline]
    pub unsafe fn swap_pop_unchecked(&mut self, idx_a: usize) -> ELEM::Base {
        self.len -= 1;
        if ELEM::BITS > 0 {
            let (real_a, off_a) = Self::calc_sub_idx_to_real_idx_and_bit_offset(idx_a);
            let (real_last, off_last) = Self::calc_sub_idx_to_real_idx_and_bit_offset(self.len);
            let mask_a = ELEM::MASK << off_a;
            let mask_last = ELEM::MASK << off_last;
            let ptr_a = self.ptr.as_ptr().add(real_a);
            let ptr_last = self.ptr.as_ptr().add(real_last);
            let val_a = (ptr::read(ptr_a) & mask_a) >> off_a;
            let block_last = ptr::read(ptr_last);
            let val_last = (block_last & mask_last) >> off_last;
            ptr::write(ptr_last, block_last & !mask_last);
            let block_a = ptr::read(ptr_a) & !mask_a;
            ptr::write(ptr_a, block_a | (val_last << off_a));
            ELEM::bits_to_val(val_a)
        } else {
            ELEM::bits_to_val(0)
        }
    }

    
    #[inline]
    pub fn shrink(&mut self, shrink: Shrink) -> Result<(), String> {
        unsafe {self.handle_resize(ElementCount::Total(self.len), Resize::Shrink(shrink), true)}
    }

    #[inline]
    pub fn set_exact_capacity(&mut self, new_cap: usize) -> Result<(), String> {
        unsafe {self.handle_resize(ElementCount::Total(self.len), Resize::ExactCapacity(new_cap), true)}
    }

    #[inline]
    pub fn append<II>(&mut self, source: II) -> Result<(), String>
    where II: IntoIterator<Item = ELEM::Base> {
        self.append_custom_grow(source, Self::DEFAULT_GROW)
    }

    #[inline]
    pub fn append_custom_grow<II>(&mut self, source: II, grow: Grow) -> Result<(), String>
    where II: IntoIterator<Item = ELEM::Base> {
        let iter = source.into_iter();
        let projected_additional = match iter.size_hint() {
            (_, Some(upper_bound)) => upper_bound,
            (lower_bound, None) => lower_bound
        };
        self.grow_if_needed_custom(ElementCount::Additional(projected_additional), grow)?;
        for elem in iter {
            self.push_custom_grow(elem, grow)?;
        }
        Ok(())
    }

    #[inline]
    pub fn clone_val(&self, idx: usize) -> Result<ELEM::Base, String> 
    where ELEM::Base: Clone {
        if idx < self.len {
            Ok(unsafe{self.clone_val_unchecked(idx)})
        } else {
            Err(format!("index out of bounds for clone:\n\tlen = {}\n\tidx = {}", self.len, idx))
        }
    }

    #[inline]
    pub unsafe fn clone_val_unchecked(&self, idx: usize) -> ELEM::Base
    where ELEM::Base: Clone {
        if ELEM::BITS > 0 {
            let (real_idx, bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(idx);
            let bit_diff = BitUtil::USIZE_BITS - bit_off;
            let this_block_mask_to_read = ELEM::MASK << bit_off;
            let next_block_mask_to_read = ELEM::MASK >> bit_diff;
            let mut block_ptr = self.ptr.as_ptr().add(real_idx);
            let mut block_bits = ptr::read(block_ptr);
            let mut val_bits: usize = (block_bits & this_block_mask_to_read) >> bit_off;
            if next_block_mask_to_read > 0 {
                block_ptr = block_ptr.add(1);
                block_bits = ptr::read(block_ptr);
                val_bits |= (block_bits & next_block_mask_to_read) << bit_diff;
            }
            ELEM::bits_to_val(val_bits)
        } else {
            ELEM::bits_to_val(0)
        }
    }

    #[inline]
    pub fn replace(&self, idx: usize, val: ELEM::Base) -> Result<ELEM::Base, String> {
        if idx < self.len {
            Ok(unsafe{self.replace_unchecked(idx, val)})
        } else {
            Err(format!("index out of bounds for clone:\n\tlen = {}\n\tidx = {}", self.len, idx))
        }
    }

    #[inline]
    pub unsafe fn replace_unchecked(&self, idx: usize, val: ELEM::Base) -> ELEM::Base {
        if ELEM::BITS > 0 {
            let in_bits = ELEM::val_to_bits(val);
            let (real_idx, bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(idx);
            let bit_diff = BitUtil::USIZE_BITS - bit_off;
            let this_block_mask_to_read = ELEM::MASK << bit_off;
            let next_block_mask_to_read = ELEM::MASK >> bit_diff;
            let mut block_ptr = self.ptr.as_ptr().add(real_idx);
            let mut block_bits = ptr::read(block_ptr);
            let mut out_bits: usize = (block_bits & this_block_mask_to_read) >> bit_off;
            block_bits = (block_bits & !this_block_mask_to_read) | (in_bits << bit_off);
            ptr::write(block_ptr, block_bits);
            if next_block_mask_to_read > 0 {
                block_ptr = block_ptr.add(1);
                block_bits = ptr::read(block_ptr);
                out_bits |= (block_bits & next_block_mask_to_read) << bit_diff;
                block_bits = (block_bits & !next_block_mask_to_read) | (in_bits >> bit_diff);
                ptr::write(block_ptr, block_bits);
            }
            ELEM::bits_to_val(out_bits)
        } else {
            ELEM::bits_to_val(0)
        }
    }

    #[inline]
    pub fn set(&mut self, idx: usize, val: ELEM::Base) -> Result<(), String> {
        if idx < self.len {
            unsafe{self.set_unchecked(idx, val)};
            Ok(())
        } else {
            Err(format!("index out of range for set: idx = {}, len = {}", idx, self.len))
        }
    }

    #[inline]
    pub unsafe fn set_unchecked(&mut self, idx: usize, val: ELEM::Base) {
        if ELEM::BITS > 0 {
            let (real_idx, bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(idx);
            let val_bits = ELEM::val_to_bits(val);
            let this_block_bits_to_push = val_bits << bit_off;
            let this_block_bits_mask = ELEM::MASK << bit_off;
            let next_block_bits_to_push = val_bits >> (BitUtil::USIZE_BITS - bit_off);
            let next_block_bits_mask = ELEM::MASK >> (BitUtil::USIZE_BITS - bit_off);
            let mut block_ptr = self.ptr.as_ptr().add(real_idx);
            let mut block_bits = ptr::read(block_ptr);
            block_bits &= !this_block_bits_mask;
            block_bits |= this_block_bits_to_push;
            ptr::write(block_ptr, block_bits);
            if next_block_bits_to_push > 0 {
                block_ptr = block_ptr.add(1);
                block_bits = ptr::read(block_ptr);
                block_bits &= !next_block_bits_mask;
                block_bits |= next_block_bits_to_push;
                ptr::write(block_ptr, block_bits);
            }
        }
    }

    pub fn drain<'vec>(&'vec mut self) -> BVecDrain<'vec, ELEM> {
        let drain = BVecDrain {
            vec: PhantomData,
            ptr: self.ptr,
            start: 0,
            count: self.len
        };
        self.len = 0;
        drain
    }

    #[inline]
    pub(crate) const fn calc_sub_idx_to_real_idx_and_bit_offset(elem_idx: usize) -> (usize, usize) {
        match BitUtil::USIZE_BITS {
            64 => match ELEM::BITS {
                1 => (elem_idx >> 6, elem_idx & 0b_00111111),
                2 => (elem_idx >> 5, (elem_idx & 0b_00011111) << 1),
                4 => (elem_idx >> 4, (elem_idx & 0b_00001111) << 2),
                8 => (elem_idx >> 3, (elem_idx & 0b_00000111) << 3),
                16 => (elem_idx >> 2, (elem_idx & 0b_00000011) << 4),
                32 => (elem_idx >> 1, (elem_idx & 0b_00000001) << 5),
                64 => (elem_idx, 0),
                128 => (elem_idx << 1, 0),
                _ => {
                    let total_bits = elem_idx * ELEM::BITS;
                    (total_bits >> 6, total_bits & 0b_00111111)
                } 
            },
            32 => match ELEM::BITS {
                1 => (elem_idx >> 5, elem_idx & 0b_00011111),
                2 => (elem_idx >> 4, (elem_idx & 0b_00001111) << 1),
                4 => (elem_idx >> 3, (elem_idx & 0b_00000111) << 2),
                8 => (elem_idx >> 2, (elem_idx & 0b_00000011) << 3),
                16 => (elem_idx >> 1, (elem_idx & 0b_00000001) << 4),
                32 => (elem_idx, 0),
                64 => (elem_idx << 1, 0),
                128 => (elem_idx << 2, 0),
                _ => {
                    let total_bits = elem_idx * ELEM::BITS;
                    (total_bits >> 5, total_bits & 0b_00011111)
                } 
            },
            16 => match ELEM::BITS {
                1 => (elem_idx >> 4, elem_idx & 0b_00001111),
                2 => (elem_idx >> 3, (elem_idx & 0b_00000111) << 1),
                4 => (elem_idx >> 2, (elem_idx & 0b_00000011) << 2),
                8 => (elem_idx >> 1, (elem_idx & 0b_00000001) << 3),
                16 => (elem_idx, 0),
                32 => (elem_idx << 1, 0),
                64 => (elem_idx << 2, 0),
                128 => (elem_idx << 3, 0),
                _ => {
                    let total_bits = elem_idx * ELEM::BITS;
                    (total_bits >> 4, total_bits & 0b_00001111)
                } 
            }
            _ => {
                let total_bits = elem_idx * ELEM::BITS;
                (total_bits / BitUtil::USIZE_BITS, total_bits % BitUtil::USIZE_BITS)
            }
        }
    }

    #[inline]
    pub(crate) const fn calc_end_real_idx_from_start_real_idx_and_bit_offset(real_idx: usize, bit_off: usize) -> usize {
        let bit_end = bit_off + ELEM::BITS;
        match BitUtil::USIZE_BITS {
            64 => real_idx + (bit_end >> 6),
            32 => real_idx + (bit_end >> 5),
            16 => real_idx + (bit_end >> 4),
            _ => real_idx + (bit_end / BitUtil::USIZE_BITS)
        }
    }

    #[inline]
    pub(crate) const fn calc_real_count_from_sub_count(count: usize) -> usize {
        let (real_idx, bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(count);
        real_idx + BitUtil::one_if_val_isnt_zero(bit_off)
    }

    #[inline]
    #[cfg(test)]
    pub(crate) const fn calc_real_len_from_sub_len(&self) -> usize {
        Self::calc_real_count_from_sub_count(self.len)
    }

    // #[inline]
    // #[cfg(test)]
    // pub(crate) const fn calc_real_cap_from_sub_cap(&self) -> usize {
    //     Self::calc_real_count_from_sub_count(self.cap)
    // }

    #[inline]
    pub(crate) const fn calc_elem_count_from_total_bits(total_bits: usize) -> usize {
        match ELEM::BITS {
            1 => total_bits,
            2 => total_bits >> 1,
            4 => total_bits >> 2,
            8 => total_bits >> 3,
            16 => total_bits >> 4,
            32 => total_bits >> 5,
            64 => total_bits >> 6,
            128 => total_bits >> 7,
            _ => total_bits / ELEM::BITS
        }
    }

    #[inline]
    pub(crate) const fn calc_sub_cap_from_real_cap(real_cap: usize) -> usize {
        let total_bits = BitUtil::calc_total_bits_in_num_usize(real_cap);
        Self::calc_elem_count_from_total_bits(total_bits)
    }

    #[inline]
    pub(crate) unsafe fn handle_resize(&mut self, element_count: ElementCount, resize: Resize, force_realloc: bool) -> Result<(), String> {
        let target_len = match element_count {
            ElementCount::Total(len) => len,
            ElementCount::Additional(count) if usize::MAX - count > self.len => self.len.saturating_add(count),
            ElementCount::Additional(count) => return Err(format!("{} additional elements would overflow usize::MAX:\n\tusize::MAX = {}\n\trequested  = {}", count, usize::MAX, self.len as u128 + count as u128))
        };
        if ELEM::BITS > 0 {
            if target_len > MemUtil::MAX_CAPACITY_FOR_USIZE {
                return Err(format!("resize would overflow user memory space:\n\tuser memory space = {} bytes\n\trequested memory =  {} x {} bytes\n\trequested memory =  {} bytes", isize::MAX, target_len, BitUtil::USIZE_BYTES, target_len * BitUtil::USIZE_BYTES));
            }
            if (force_realloc && target_len != self.cap) || target_len > self.cap {
                let new_cap = match resize {
                    Resize::ExactCapacity(cap) => cap,
                    Resize::Grow(grow) => match grow {
                        Grow::Exact => target_len,
                        Grow::ExactPlus(count) => target_len.saturating_add(count).min(MemUtil::MAX_CAPACITY_FOR_USIZE),
                        Grow::OnePointFive => target_len.saturating_add(target_len >> 1).min(MemUtil::MAX_CAPACITY_FOR_USIZE),
                        Grow::Double => target_len.saturating_mul(2).min(MemUtil::MAX_CAPACITY_FOR_USIZE),
                    },
                    Resize::Shrink(shrink) => match shrink {
                        Shrink::Minimum => target_len,
                        Shrink::SubtractOrMinimum(count) => target_len.max(self.cap - count),
                        Shrink::SubtractTruncate(count) => self.cap - count,
                        Shrink::ThreeQuartersOrMinimum => target_len.max((self.cap >> 1) + (self.cap >> 2)),
                        Shrink::ThreeQuartersTruncate => (self.cap >> 1) + (self.cap >> 2),
                        Shrink::HalfOrMinimum => target_len.max(self.cap >> 1),
                        Shrink::HalfTruncate => self.cap >> 1,
                    },
                };
                if needs_drop::<ELEM::Base>() {
                    while self.len > new_cap {
                        let _ = self.pop_unchecked();
                    }
                }
                let target_real_capacity = Self::calc_real_count_from_sub_count(new_cap);
                let current_real_capacity = Self::calc_real_count_from_sub_count(self.cap);
                let new_layout: Layout = Layout::from_size_align_unchecked(target_real_capacity*size_of::<usize>(), align_of::<usize>());
                let new_ptr = match self.cap {
                    0 => {
                        alloc::alloc(new_layout)
                    },
                    _ => {
                        let old_layout = Layout::from_size_align_unchecked(current_real_capacity*size_of::<usize>(), align_of::<usize>());
                        alloc::realloc(self.ptr.as_ptr().cast(), old_layout, new_layout.size())
                    },
                };
                let new_sub_capacity = Self::calc_sub_cap_from_real_cap(target_real_capacity);
                match NonNull::new(new_ptr) {
                    Some(non_null) => {
                        self.ptr = non_null.cast();
                        self.cap = new_sub_capacity;
                        Ok(())
                    },
                    None => Err(format!("memory allocation failed:\n\tlayout = {:?}", new_layout)),
                }
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

impl<ELEM> IntoIterator for BitVec<ELEM>
where ELEM: BitElem {
    type Item = ELEM::Base;

    type IntoIter = BVecIter<ELEM>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = BVecIter{
            ptr: self.ptr,
            real_cap: BitVec::<ELEM>::calc_real_count_from_sub_count(self.cap),
            start: 0,
            count: self.len,
            sub: PhantomData
        };
        mem::forget(self);
        iter
    }
}