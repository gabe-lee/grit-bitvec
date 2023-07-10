use std::{ops::RangeBounds, alloc::handle_alloc_error};

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
    Resize,
    ElemAccess, utils::RangeUtil, IdxProxy, IdxProxyRange
};

pub struct RawBitVec<const BIT_WIDTH: usize> {
    pub(crate) ptr: NonNull<usize>,
    pub(crate) len: usize,
    pub(crate) cap: usize,
}

impl<const BIT_WIDTH: usize> RawBitVec<BIT_WIDTH> {

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
        if BIT_WIDTH > 0 {
            Self {
                ptr: NonNull::dangling(),
                cap: 0,
                len: 0,
            }
        } else {
            Self {
                ptr: NonNull::dangling(),
                cap: usize::MAX,
                len: 0,
            }
        }
    }

    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        if BIT_WIDTH > 0 {
            let mut new_vec = Self::new();
            let real_cap = IdxProxy::<BIT_WIDTH>::calc_real_count_from_bitwise_count(cap);
            let (new_ptr, new_layout) = unsafe {Self::alloc_new(real_cap)};
            let new_non_null = Self::handle_alloc_result(new_layout, new_ptr);
            new_vec.cap = IdxProxy::<BIT_WIDTH>::calc_bitwise_count_from_real_count(real_cap);
            new_vec.ptr = new_non_null;
            new_vec
        } else {
            Self {
                ptr: NonNull::dangling(),
                cap: usize::MAX,
                len: 0,
            }
        }
    }

    // #[inline]
    // pub fn grow_if_needed(&mut self, elem_count: ElementCount) -> Result<(), String> {
    //     self.grow_if_needed_custom(elem_count, Self::DEFAULT_GROW)
    // }

    // #[inline]
    // pub fn grow_if_needed_custom(&mut self, elem_count: ElementCount, grow: Grow) -> Result<(), String> {
    //     unsafe {self.handle_resize(elem_count, Resize::Grow(grow), false)}
    // }

    #[inline]
    pub fn grow_exact_for_additional_elements_if_needed(&mut self, extra_elements: usize) -> Result<(), String> {
        unsafe {self.handle_grow_if_needed(self.len + extra_elements, false)}
    }

    #[inline]
    pub fn grow_exact_for_total_elements_if_needed(&mut self, total_elements: usize) -> Result<(), String> {
        unsafe {self.handle_grow_if_needed(total_elements, false)}
    }

    #[inline]
    pub fn grow_for_additional_elements_if_needed(&mut self, extra_elements: usize) -> Result<(), String> {
        unsafe {self.handle_grow_if_needed(self.len + extra_elements, true)}
    }

    #[inline]
    pub fn grow_for_total_elements_if_needed(&mut self, total_elements: usize) -> Result<(), String> {
        unsafe {self.handle_grow_if_needed(total_elements, true)}
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0
    }

    // #[inline]
    // pub fn clear(&mut self) {
    //     if needs_drop::<ELEM::Base>() {
    //         while self.len > 0 {
    //             let _ = unsafe {self.pop_unchecked()};
    //         }
    //     } else {
    //         self.len = 0
    //     }
    // }

    #[inline]
    pub fn push(&mut self, val: usize) -> Result<(), String> {
        match self.len == IdxProxy::<BIT_WIDTH>::MAX_CAPACITY {
            true => Err(format!("BitVec is at maximum capacity ({})", IdxProxy::<BIT_WIDTH>::MAX_CAPACITY)),
            false => unsafe {
                self.handle_grow_if_needed(self.len+1, true)?;
                self.push_unchecked(val);
                Ok(())
            }
        }
    }

    #[inline]
    pub unsafe fn push_unchecked(&mut self, val: usize) {
        if BIT_WIDTH > 0 {
            let len_proxy = IdxProxy::<BIT_WIDTH>::from(self.len);
            self.clean_new_blocks_if_needed(len_proxy, 1);
            self.write_val_with_idx_proxy(len_proxy, val);
        }
        self.len += 1;
    }

    #[inline]
    pub fn pop(&mut self) -> Result<usize, String> {
        if self.len == 0 {
            Err(format!("no elements in BitVec to pop out"))
        } else {
            Ok(unsafe{self.pop_unchecked()})
        }
    }

    #[inline]
    pub unsafe fn pop_unchecked(&mut self) -> usize {
        self.len -= 1;
        if BIT_WIDTH > 0 {
            let last_proxy = IdxProxy::<BIT_WIDTH>::from(self.len);
            self.replace_val_with_idx_proxy(last_proxy, 0)
        } else {
            0
        }
    }

    #[inline]
    pub fn insert(&mut self, idx: usize, val: usize) -> Result<(), String> {
        if idx > self.len {
            return Err(format!("index out of bounds for insert: (idx) {} > {} (len)", idx, self.len));
        }
        if self.len == IdxProxy::<BIT_WIDTH>::MAX_CAPACITY {
            return Err(format!("BitVec is at maximum capacity ({})", IdxProxy::<BIT_WIDTH>::MAX_CAPACITY));
        }
        unsafe {
            self.handle_grow_if_needed(self.len+1, true)?;
            match idx == self.len {
                true => Ok(self.push_unchecked(val)),
                false => Ok(self.insert_unchecked(idx, val))
            }
        }
    }

    #[inline]
    pub unsafe fn insert_unchecked(&mut self, idx: usize, val: usize) {
        if BIT_WIDTH > 0 {
            let idx_proxy = IdxProxy::<BIT_WIDTH>::from(idx);
            let len_proxy = IdxProxy::<BIT_WIDTH>::from(self.len);
            self.clean_new_blocks_if_needed(len_proxy, 1);
            self.shift_elements_up_with_with_idx_proxy(idx_proxy, 1);
            self.write_val_with_idx_proxy(idx_proxy, val);
        }
        self.len += 1;
    }

    pub fn insert_bitvec(&mut self, insert_idx: usize, bitvec: Self) -> Result<(), String> {
        if insert_idx > self.len {
            return Err(format!("index out of bounds for insert_bitvec: (idx) {} > {} (len)", insert_idx, self.len));
        }
        if IdxProxy::<BIT_WIDTH>::MAX_CAPACITY - bitvec.len < self.len {
            return Err(format!("BitVec cannot hold {} more elements, {} elements would reach the maximum capacity ({})", bitvec.len, IdxProxy::<BIT_WIDTH>::MAX_CAPACITY - self.len, IdxProxy::<BIT_WIDTH>::MAX_CAPACITY));
        }
        unsafe {
            self.handle_grow_if_needed(self.len + bitvec.len, true)?;
            match insert_idx == self.len {
                true => todo!(),
                false => self.insert_bitvec_unchecked(insert_idx, bitvec)
            }
        }
        Ok(())
    }

    pub fn insert_bitvec_unchecked(&mut self, insert_idx: usize, bitvec: Self) {
        let mut write_iter = IdxProxyRange::<BIT_WIDTH>::from(insert_idx..insert_idx+bitvec.len);
        let mut read_iter = IdxProxyRange::<BIT_WIDTH>::from(0..bitvec.len);
        let mut count = bitvec.len;
        let begin_idx = IdxProxy::<BIT_WIDTH>::from(insert_idx);
        let len_proxy = IdxProxy::<BIT_WIDTH>::from(self.len);
        unsafe {
            self.clean_new_blocks_if_needed(len_proxy, count);
            self.shift_elements_up_with_with_idx_proxy(begin_idx, count);
            while count > 0 {
                    let write_idx = write_iter.next().unwrap();
                    let read_idx = read_iter.next().unwrap();
                    let val = bitvec.read_val_with_idx_proxy(read_idx);
                    self.write_val_with_idx_proxy(write_idx, val);
            }
        }
    }

    // #[inline]
    // pub unsafe fn insert_iter<II>(&mut self, insert_idx: usize, source: II) -> Result<(), String>
    // where II: IntoIterator<Item = ELEM::Base> {
    //     match insert_idx > self.len {
    //         true => Err(format!("index out of bounds for insert_iter:\n\tlen = {}\n\tidx = {}", self.len, insert_idx)),
    //         false => {
    //             let iter = source.into_iter();
                
    //             Ok(())
    //         },
    //     }
    //     if insert_idx > self.len {

    //     }
    //     let iter = source.into_iter();
    //     if ELEM::BITS > 0 {
    //         for elem in iter {
    //             self.insert_unchecked(insert_idx, elem);
    //         }
    //     } else {
    //         for _ in iter {
    //             self.len += 1;
    //         }
    //     }
    //     let iter = source.into_iter();
    //     if ELEM::BITS > 0 {
    //         for elem in iter {
    //             self.insert_unchecked(insert_idx, elem);
    //         }
    //     } else {
    //         for _ in iter {
    //             self.len += 1;
    //         }
    //     }
    // }

    // #[inline]
    // pub unsafe fn insert_iter_unchecked<II>(&mut self, mut insert_idx: usize, source: II)
    // where II: IntoIterator<Item = ELEM::Base> {
    //     let iter = source.into_iter();
    //     if ELEM::BITS > 0 {
    //         for elem in iter {
    //             self.insert_unchecked(insert_idx, elem);
    //         }
    //     } else {
    //         for _ in iter {
    //             self.len += 1;
    //         }
    //     }
    // }

    #[inline]
    pub fn remove(&mut self, idx: usize) -> Result<usize, String> {
        match idx >= self.len {
            true => Err(format!("index out of bounds for remove: (idx) {} >= {} (len)", idx, self.len)),
            false => {
                if idx == self.len - 1 {
                    Ok(unsafe{self.pop_unchecked()})
                } else {
                    Ok(unsafe{self.remove_unchecked(idx)})
                }
            },
        }
    }

    #[inline]
    pub unsafe fn remove_unchecked(&mut self, idx: usize) -> usize {
        self.len -= 1;
        if BIT_WIDTH > 0 {
            let idx_proxy = IdxProxy::<BIT_WIDTH>::from(idx);
            let shift_proxy = IdxProxy::<BIT_WIDTH>::from(idx+1);
            let val = self.replace_val_with_idx_proxy(idx_proxy, 0);
            self.shift_elements_down_with_with_idx_proxy(idx_proxy, shift_proxy, 1);
            val
        } else {
            0
        }
    }

    #[inline]
    pub fn remove_range(&mut self, idx_range: IdxProxyRange<BIT_WIDTH>) -> Result<Self, String> {
        match idx_range.end_excluded > self.len {
            true => Err(format!("index out of bounds for remove range: (idx) {} >= {} (len)", idx_range.end_excluded-1, self.len)),
            false => {
                Ok(unsafe {self.remove_range_unchecked(idx_range)})
            },
        }
    }

    #[inline]
    pub unsafe fn remove_range_unchecked(&mut self, idx_range: IdxProxyRange<BIT_WIDTH>) -> Self {
        let new_cap = idx_range.len();
        let mut new_vec = Self::with_capacity(new_cap);
        if BIT_WIDTH > 0 {
            let start_proxy = IdxProxy::<BIT_WIDTH>::from(idx_range.start);
            let end_excluded_proxy = IdxProxy::<BIT_WIDTH>::from(idx_range.end_excluded);
            let count = idx_range.len();            
            for idx_proxy in idx_range {
                let val = self.replace_val_with_idx_proxy(idx_proxy, 0);
                new_vec.push_unchecked(val);
            }
            if end_excluded_proxy.bitwise_idx < self.len {
                self.shift_elements_down_with_with_idx_proxy(start_proxy, end_excluded_proxy, count);
            }
            self.len -= count;
        } else {
            let count =idx_range.len();
            new_vec.len = count;
            self.len -= count;
        }
        new_vec
    }

    #[inline]
    pub fn swap(&mut self, idx_a: usize, idx_b: usize) -> Result<(), String> {
        if idx_a >= self.len || idx_b >= self.len {
            return Err(format!("index out of bounds for swap: (idx_a) {} >= {} (len) OR (idx_b) {} >= {} (len)", idx_a, self.len, idx_b, self.len))
        } else if idx_a != idx_b {
            unsafe {self.swap_unchecked(idx_a, idx_b)};
        }
        Ok(())
    }

    #[inline]
    pub unsafe fn swap_unchecked(&mut self, idx_a: usize, idx_b: usize) {
        if BIT_WIDTH > 0 {
            let proxy_a = IdxProxy::<BIT_WIDTH>::from(idx_a);
            let proxy_b = IdxProxy::<BIT_WIDTH>::from(idx_b);
            self.swap_vals_with_idx_proxy(proxy_a, proxy_b)
        }
    }

    #[inline]
    pub fn swap_pop(&mut self, idx: usize) -> Result<usize, String> {
        if idx >= self.len {
            Err(format!("index out of bounds for swap pop: (idx) {} >= {} (len)", idx, self.len))
        } else if idx == self.len - 1 {
            Ok(unsafe{self.pop_unchecked()})
        } else {
            Ok(unsafe {self.swap_pop_unchecked(idx)})
        }
    }

    #[inline]
    pub unsafe fn swap_pop_unchecked(&mut self, idx: usize) -> usize {
        self.len -= 1;
        if BIT_WIDTH > 0 {
            let last_proxy = IdxProxy::<BIT_WIDTH>::from(self.len);
            let pop_proxy = IdxProxy::<BIT_WIDTH>::from(idx);
            self.swap_pop_val_with_idx_proxy(pop_proxy, last_proxy)
        } else {
            0
        }
    }

    #[inline]
    pub fn trim_excess_capacity(&mut self, extra_capacity_to_keep: usize) -> Result<(), String> {
        if BIT_WIDTH > 0 {
            let target_capacity = self.len.saturating_add(extra_capacity_to_keep);
            if target_capacity < self.cap {
                unsafe {
                    let target_real_capacity = IdxProxy::<BIT_WIDTH>::calc_real_count_from_bitwise_count(target_capacity);
                    let current_real_capacity = IdxProxy::<BIT_WIDTH>::calc_real_count_from_bitwise_count(self.cap);
                    let new_layout: Layout = Layout::from_size_align_unchecked(target_real_capacity*size_of::<usize>(), align_of::<usize>());
                    let old_layout = Layout::from_size_align_unchecked(current_real_capacity*size_of::<usize>(), align_of::<usize>());
                    let new_ptr = alloc::realloc(self.ptr.as_ptr().cast(), old_layout, new_layout.size());
                    let new_bitwise_capacity = IdxProxy::<BIT_WIDTH>::calc_bitwise_count_from_real_count(target_real_capacity);
                    let new_non_null = Self::handle_alloc_result(new_layout, new_ptr);
                    self.cap = new_bitwise_capacity;
                    self.ptr = new_non_null;
                }
            }
        }
        Ok(())
    }

    pub fn append_bitvec(&mut self, bitvec: Self) -> Result<(), String> {
        if IdxProxy::<BIT_WIDTH>::MAX_CAPACITY - bitvec.len < self.len {
            return Err(format!("BitVec cannot hold {} more elements, {} elements would reach the maximum capacity ({})", bitvec.len, IdxProxy::<BIT_WIDTH>::MAX_CAPACITY - self.len, IdxProxy::<BIT_WIDTH>::MAX_CAPACITY));
        }
        unsafe {self.append_bitvec_unchecked(bitvec)};
        Ok(())
    }

    pub fn append_bitvec_unchecked(&mut self, bitvec: Self) {
        let mut write_iter = IdxProxyRange::<BIT_WIDTH>::from(self.len..self.len+bitvec.len);
        let mut read_iter = IdxProxyRange::<BIT_WIDTH>::from(0..bitvec.len);
        let mut count = bitvec.len;
        let len_proxy = IdxProxy::<BIT_WIDTH>::from(self.len);
        unsafe {
            self.clean_new_blocks_if_needed(len_proxy, count);
            while count > 0 {
                    let write_idx = write_iter.next().unwrap();
                    let read_idx = read_iter.next().unwrap();
                    let val = bitvec.read_val_with_idx_proxy(read_idx);
                    self.write_val_with_idx_proxy(write_idx, val);
            }
        }
    }

    // #[inline]
    // pub fn append_iter<II>(&mut self, source: II) -> Result<(), String>
    // where II: IntoIterator<Item = ELEM::Base> {
    //     self.append_iter_custom_grow(source, Self::DEFAULT_GROW)
    // }

    // #[inline]
    // pub fn append_iter_custom_grow<II>(&mut self, source: II, grow: Grow) -> Result<(), String>
    // where II: IntoIterator<Item = ELEM::Base> {
    //     let iter = source.into_iter();
    //     let projected_additional = match iter.size_hint() {
    //         (_, Some(upper_bound)) => upper_bound,
    //         (lower_bound, None) => lower_bound
    //     };
    //     self.grow_if_needed_custom(ElementCount::Additional(projected_additional), grow)?;
    //     for elem in iter {
    //         self.push_custom_grow(elem, grow)?;
    //     }
    //     Ok(())
    // }

    #[inline]
    pub fn get(&self, idx: usize) -> Result<usize, String> {
        if idx < self.len {
            Ok(unsafe{self.get_unchecked(idx)})
        } else {
            Err(format!("index out of bounds for get_val: (idx) {} >= {} (len)", idx, self.len))
        }
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, idx: usize) -> usize {
        if BIT_WIDTH > 0 {
            let idx_proxy = IdxProxy::<BIT_WIDTH>::from(idx);
            self.read_val_with_idx_proxy(idx_proxy)
        } else {
            0
        }
    }

    #[inline]
    pub fn replace(&mut self, idx: usize, val: usize) -> Result<usize, String> {
        if idx < self.len {
            Ok(unsafe{self.replace_unchecked(idx, val)})
        } else {
            Err(format!("index out of bounds for replace: (idx) {} >= {} (len)", idx, self.len))
        }
    }

    #[inline]
    pub unsafe fn replace_unchecked(&mut self, idx: usize, val: usize) -> usize {
        if BIT_WIDTH > 0 {
            let idx_proxy = IdxProxy::<BIT_WIDTH>::from(idx);
            self.replace_val_with_idx_proxy(idx_proxy, val)
        } else {
            0
        }
    }

    #[inline]
    pub fn set(&mut self, idx: usize, val: usize) -> Result<(), String> {
        if idx < self.len {
            unsafe{self.set_unchecked(idx, val)};
            Ok(())
        } else {
            Err(format!("index out of bounds for set: (idx) {} >= {} (len)", idx, self.len))
        }
    }

    #[inline]
    pub unsafe fn set_unchecked(&mut self, idx: usize, val: usize) {
        if BIT_WIDTH > 0 {
            let idx_proxy = IdxProxy::<BIT_WIDTH>::from(idx);
            self.write_val_with_idx_proxy(idx_proxy, val);
        }
    }

    // pub fn drain<'vec>(&'vec mut self) -> BVecDrain<'vec, ELEM> {
    //     let drain = BVecDrain {
    //         vec: PhantomData,
    //         ptr: self.ptr,
    //         start: 0,
    //         count: self.len
    //     };
    //     self.len = 0;
    //     drain
    // }

    #[inline]
    pub(crate) unsafe fn read_val_with_idx_proxy(&self, idx_proxy: IdxProxy<BIT_WIDTH>) -> usize {
        let mut block_ptr = self.ptr.as_ptr().add(idx_proxy.real_idx);
        let mut block_bits = ptr::read(block_ptr);
        let mut val = (block_bits & idx_proxy.first_mask) >> idx_proxy.first_offset;
        if idx_proxy.second_mask != 0 {
            block_ptr = block_ptr.add(1);
            block_bits = ptr::read(block_ptr);
            val = val | ((block_bits & idx_proxy.second_mask) << idx_proxy.second_offset);
        }
        val
    }

    #[inline]
    pub(crate) unsafe fn replace_val_with_idx_proxy(&mut self, idx_proxy: IdxProxy<BIT_WIDTH>, new_val: usize) -> usize {
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

    pub(crate) unsafe fn clean_new_blocks_if_needed(&mut self, len_proxy: IdxProxy<BIT_WIDTH>, added_elements: usize) {
        let final_proxy = IdxProxy::<BIT_WIDTH>::from(len_proxy.bitwise_idx + added_elements);
        let len_block_needs_clean = (len_proxy.first_offset == 0) as usize;
        let final_block_does_not_need_clean = (final_proxy.first_offset == 0) as usize;
        let clean_block_count = len_block_needs_clean + (final_proxy.real_idx - len_proxy.real_idx - final_block_does_not_need_clean);
        ptr::write_bytes(self.ptr.as_ptr().add(len_proxy.real_idx + 1 - len_block_needs_clean), 0, clean_block_count);
    }

    #[inline]
    pub(crate) unsafe fn write_val_with_idx_proxy(&mut self, idx_proxy: IdxProxy<BIT_WIDTH>, new_val: usize) {
        let mut block_ptr = self.ptr.as_ptr().add(idx_proxy.first_offset);
        let mut block_bits = ptr::read(block_ptr);
        block_bits = (block_bits & !idx_proxy.first_mask) | (new_val << idx_proxy.first_offset);
        ptr::write(block_ptr, block_bits);
        if idx_proxy.second_mask != 0 {
            block_ptr = block_ptr.add(1);
            block_bits = if idx_proxy.second_offset < BIT_WIDTH {
                0
            } else {
                ptr::read(block_ptr)
            };
            block_bits = (block_bits & !idx_proxy.second_mask) | (new_val >> idx_proxy.second_offset);
            ptr::write(block_ptr, block_bits);
        }
    }

    #[inline]
    pub(crate) unsafe fn swap_vals_with_idx_proxy(&mut self, proxy_a: IdxProxy<BIT_WIDTH>, proxy_b: IdxProxy<BIT_WIDTH>) {
        let val_a = self.replace_val_with_idx_proxy(proxy_a, 0);
        let val_b = self.replace_val_with_idx_proxy(proxy_b, 0);
        self.write_val_with_idx_proxy(proxy_a, val_b);
        self.write_val_with_idx_proxy(proxy_b, val_a);
    }

    #[inline]
    pub(crate) unsafe fn swap_pop_val_with_idx_proxy(&mut self, pop_proxy: IdxProxy<BIT_WIDTH>, last_proxy: IdxProxy<BIT_WIDTH>) -> usize {
        let val_last = self.replace_val_with_idx_proxy(last_proxy, 0);
        self.replace_val_with_idx_proxy(pop_proxy, val_last)
    }

    #[inline]
    pub(crate) unsafe fn shift_elements_up_with_with_idx_proxy(&mut self, begin_proxy: IdxProxy<BIT_WIDTH>, shift_count: usize) {
        let new_real_len = IdxProxy::<BIT_WIDTH>::calc_real_count_from_bitwise_count(self.len+shift_count);
        let block_count = new_real_len - begin_proxy.real_idx;
        let mut block_ptr = self.ptr.as_ptr().add(begin_proxy.real_idx);
        let mut block_bits = ptr::read(block_ptr);
        let keep_first_mask = BitUtil::all_bits_less_than_bit(begin_proxy.first_offset);
        let keep_first_bits = block_bits & keep_first_mask;
        block_bits &= !keep_first_mask;
        ptr::write(block_ptr, block_bits);
        let total_bits_shifted = shift_count * BIT_WIDTH;
        let whole_blocks = total_bits_shifted / BitUtil::USIZE_BITS;
        if whole_blocks > 0 {
            ptr::copy(block_ptr, block_ptr.add(whole_blocks), block_count);
            block_ptr = block_ptr.add(whole_blocks)
        }
        let rollover_bits = total_bits_shifted - (whole_blocks * BitUtil::USIZE_BITS);
        if rollover_bits > 0 {
            let rollover_shift = BitUtil::USIZE_BITS - rollover_bits;
            let rollover_mask = usize::MAX << rollover_shift;
            let mut blocks_shifted = 0;
            let mut rollover_bits_paste: usize = 0; 
            let mut rollover_bits_copy: usize; 
            while blocks_shifted < block_count {
                block_bits = ptr::read(block_ptr);
                rollover_bits_copy = (block_bits & rollover_mask) >> rollover_shift;
                block_bits = (block_bits << rollover_bits) | rollover_bits_paste;
                ptr::write(block_ptr, block_bits);
                block_ptr = block_ptr.add(1);
                blocks_shifted += 1;
                rollover_bits_paste = rollover_bits_copy;
            }
        }
        block_ptr = self.ptr.as_ptr().add(begin_proxy.real_idx);
        block_bits = ptr::read(block_ptr);
        ptr::write(block_ptr, block_bits | keep_first_bits);
    }

    #[inline]
    pub(crate) unsafe fn shift_elements_down_with_with_idx_proxy(&mut self, begin_proxy: IdxProxy<BIT_WIDTH>, shift_proxy: IdxProxy<BIT_WIDTH>, shift_count: usize) {
        let real_len = IdxProxy::<BIT_WIDTH>::calc_real_count_from_bitwise_count(self.len);
        let block_count = real_len - shift_proxy.real_idx;
        let mut block_ptr = self.ptr.as_ptr().add(begin_proxy.real_idx);
        let mut block_bits = ptr::read(block_ptr);
        let keep_first_mask = BitUtil::all_bits_less_than_bit(begin_proxy.first_offset);
        let keep_first_bits = block_bits & keep_first_mask;
        block_bits &= !keep_first_mask;
        ptr::write(block_ptr, block_bits);
        let total_bits_shifted = shift_count * BIT_WIDTH;
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
            block_ptr = self.ptr.as_ptr().add(real_len-whole_blocks-1);
            while blocks_shifted < block_count {
                block_bits = ptr::read(block_ptr);
                rollover_bits_copy = (block_bits & rollover_mask) << rollover_shift;
                block_bits = (block_bits >> rollover_bits) | rollover_bits_paste;
                ptr::write(block_ptr, block_bits);
                block_ptr = block_ptr.sub(1);
                blocks_shifted += 1;
                rollover_bits_paste = rollover_bits_copy;
            }
        }
        block_ptr = self.ptr.as_ptr().add(begin_proxy.real_idx);
        block_bits = ptr::read(block_ptr);
        ptr::write(block_ptr, block_bits | keep_first_bits);
    }
    
    #[inline]
    pub(crate) unsafe fn handle_grow_if_needed(&mut self, min_capacity: usize, grow_exponential: bool) -> Result<(), String> {
        if BIT_WIDTH > 0 {
            if min_capacity > self.cap {
                let new_cap = match grow_exponential {
                    true => min_capacity.saturating_add(min_capacity >> 1),
                    false => min_capacity,
                };
                let target_real_capacity = IdxProxy::<BIT_WIDTH>::calc_real_count_from_bitwise_count(new_cap);
                let current_real_capacity = IdxProxy::<BIT_WIDTH>::calc_real_count_from_bitwise_count(self.cap);
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
                let new_bitwise_capacity = IdxProxy::<BIT_WIDTH>::calc_bitwise_count_from_real_count(target_real_capacity);
                let new_non_null = Self::handle_alloc_result(new_layout, new_ptr);
                self.cap = new_bitwise_capacity;
                self.ptr = new_non_null;
                Ok(())
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    pub(crate) unsafe fn alloc_new(real_cap: usize) -> (*mut u8, Layout) {
        let new_layout: Layout = Layout::from_size_align_unchecked(real_cap*size_of::<usize>(), align_of::<usize>());
        let new_ptr = alloc::alloc(new_layout);
        (new_ptr, new_layout)
    }

    pub(crate) fn handle_alloc_result(alloc_layout: Layout, alloc_result_ptr: *mut u8) -> NonNull<usize> {
        match NonNull::new(alloc_result_ptr) {
            Some(non_null) => non_null.cast::<usize>(),
            None => handle_alloc_error(alloc_layout)
        }
    }

    pub(crate) unsafe fn make_layout(real_cap: usize) -> Layout {
        Layout::from_size_align_unchecked(real_cap*size_of::<usize>(), align_of::<usize>())
    }
}

impl<const BIT_WIDTH: usize> Drop for RawBitVec<BIT_WIDTH> {
    fn drop(&mut self) {
        if BIT_WIDTH > 0 {
            unsafe {
                let real_cap = IdxProxy::<BIT_WIDTH>::calc_real_count_from_bitwise_count(self.cap);
                let layout = Self::make_layout(real_cap);
                alloc::dealloc(self.ptr.as_ptr().cast(), layout)
            }
        }
    }
}

// impl<ELEM> IntoIterator for BitVec<ELEM>
// where ELEM: BitElem {
//     type Item = ELEM::Base;

//     type IntoIter = BVecIter<ELEM>;

//     fn into_iter(self) -> Self::IntoIter {
//         let iter = BVecIter{
//             ptr: self.ptr,
//             real_cap: BitVec::<ELEM>::calc_real_count_from_sub_count(self.cap),
//             start: 0,
//             count: self.len,
//             sub: PhantomData
//         };
//         mem::forget(self);
//         iter
//     }
// }