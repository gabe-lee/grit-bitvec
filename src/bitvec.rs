// use std::ops::RangeBounds;

// use crate::{
//     mem,
//     align_of,
//     size_of,
//     needs_drop,
//     ptr,
//     NonNull,
//     PhantomData,
//     alloc,
//     Layout,
//     MemUtil,
//     BitUtil,
//     BVecIter,
//     BVecDrain,
//     BitElem,
//     ElementCount,
//     Grow,
//     Shrink,
//     Resize,
//     ElemAccess, utils::RangeUtil
// };

// pub struct BitVec<ELEM>
// where ELEM: BitElem {
//     pub(crate) ptr: NonNull<usize>,
//     pub(crate) len: usize,
//     pub(crate) cap: usize,
//     pub(crate) sub: PhantomData<ELEM>
// }

// impl<ELEM> BitVec<ELEM>
// where ELEM: BitElem {
//     pub(crate) const DEFAULT_GROW: Grow = Grow::OnePointFive;

//     #[inline]
//     pub fn len(&self) -> usize {
//         self.len
//     }

//     #[inline]
//     pub fn cap(&self) -> usize {
//         self.cap
//     }

//     #[inline]
//     pub fn free(&self) -> usize {
//         self.cap - self.len
//     }

//     #[inline]
//     pub fn new() -> Self {
//         if ELEM::BITS > 0 {
//             Self {
//                 ptr: NonNull::dangling(),
//                 cap: 0,
//                 len: 0,
//                 sub: PhantomData,
//             }
//         } else {
//             Self {
//                 ptr: NonNull::dangling(),
//                 cap: usize::MAX,
//                 len: 0,
//                 sub: PhantomData,
//             }
//         }
//     }

//     #[inline]
//     pub fn with_capacity(cap: usize) -> Result<Self, String> {
//         if ELEM::BITS > 0 {
//             let mut new_vec = Self::new();
//             unsafe{new_vec.handle_resize(ElementCount::Total(cap), Resize::ExactCapacity(cap), true)}?;
//             Ok(new_vec)
//         } else {
//             Ok(Self {
//                 ptr: NonNull::dangling(),
//                 cap,
//                 len: 0,
//                 sub: PhantomData,
//             })
//         }
//     }

//     #[inline]
//     pub fn grow_if_needed(&mut self, elem_count: ElementCount) -> Result<(), String> {
//         self.grow_if_needed_custom(elem_count, Self::DEFAULT_GROW)
//     }

//     #[inline]
//     pub fn grow_if_needed_custom(&mut self, elem_count: ElementCount, grow: Grow) -> Result<(), String> {
//         unsafe {self.handle_resize(elem_count, Resize::Grow(grow), false)}
//     }

//     #[inline]
//     pub fn clear(&mut self) {
//         if needs_drop::<ELEM::Base>() {
//             while self.len > 0 {
//                 let _ = unsafe {self.pop_unchecked()};
//             }
//         } else {
//             self.len = 0
//         }
//     }

//     #[inline]
//     pub fn push(&mut self, val: ELEM::Base) -> Result<(), String> {
//         self.push_custom_grow(val, Self::DEFAULT_GROW)
//     }

//     #[inline]
//     pub fn push_custom_grow(&mut self, val: ELEM::Base, grow: Grow) -> Result<(), String> {
//         self.grow_if_needed_custom(ElementCount::Additional(1), grow)?;
//         unsafe {self.push_unchecked(val)};
//         Ok(())
//     }

//     #[inline]
//     pub unsafe fn push_unchecked(&mut self, val: ELEM::Base) {
//         if ELEM::BITS > 0 {
//             let idx_access = Self::create_elem_accessor(self.len);
//             self.write_val_with_elem_accessor(idx_access, ELEM::val_to_bits(val));
//         }
//         self.len += 1;
//     }

//     #[inline]
//     pub fn pop(&mut self) -> Result<ELEM::Base, String> {
//         if self.len == 0 {
//             Err(format!("no elements in BitVec to pop out!"))
//         } else {
//             Ok(unsafe{self.pop_unchecked()})
//         }
//     }

//     #[inline]
//     pub unsafe fn pop_unchecked(&mut self) -> ELEM::Base {
//         self.len -= 1;
//         if ELEM::BITS > 0 {
//             let idx_access = Self::create_elem_accessor(self.len);
//             let val_bits = self.replace_val_with_elem_accessor(idx_access, 0);
//             ELEM::bits_to_val(val_bits)
//         } else {
//             ELEM::bits_to_val(0)
//         }
//     }

//     #[inline]
//     pub fn insert(&mut self, idx: usize, val: ELEM::Base) -> Result<(), String> {
//         self.insert_custom_grow(idx, val, Self::DEFAULT_GROW)
//     }

//     #[inline]
//     pub fn insert_custom_grow(&mut self, idx: usize, val: ELEM::Base, grow: Grow) -> Result<(), String> {
//         if idx > self.len {
//             return Err(format!("index out of bounds for insert:\n\tlen = {}\n\tidx = {}", self.len, idx));
//         }
//         self.grow_if_needed_custom(ElementCount::Additional(1), grow)?;
//         Ok(unsafe {self.insert_unchecked(idx, val)})
//     }

//     #[inline]
//     pub unsafe fn insert_unchecked(&mut self, idx: usize, val: ELEM::Base) {
//         if ELEM::BITS > 0 {
//             let idx_access = Self::create_elem_accessor(idx);
//             self.shift_elements_up_with_with_elem_accessor(idx_access, 1);
//             self.write_val_with_elem_accessor(idx_access, ELEM::val_to_bits(val));
//         }
//         self.len += 1;
//     }

//     #[inline]
//     pub unsafe fn insert_iter<II>(&mut self, insert_idx: usize, source: II) -> Result<(), String>
//     where II: IntoIterator<Item = ELEM::Base> {
//         match insert_idx > self.len {
//             true => Err(format!("index out of bounds for insert_iter:\n\tlen = {}\n\tidx = {}", self.len, insert_idx)),
//             false => {
//                 let iter = source.into_iter();
                
//                 Ok(())
//             },
//         }
//         if insert_idx > self.len {

//         }
//         let iter = source.into_iter();
//         if ELEM::BITS > 0 {
//             for elem in iter {
//                 self.insert_unchecked(insert_idx, elem);
//             }
//         } else {
//             for _ in iter {
//                 self.len += 1;
//             }
//         }
//         let iter = source.into_iter();
//         if ELEM::BITS > 0 {
//             for elem in iter {
//                 self.insert_unchecked(insert_idx, elem);
//             }
//         } else {
//             for _ in iter {
//                 self.len += 1;
//             }
//         }
//     }

//     #[inline]
//     pub unsafe fn insert_iter_unchecked<II>(&mut self, mut insert_idx: usize, source: II)
//     where II: IntoIterator<Item = ELEM::Base> {
//         let iter = source.into_iter();
//         if ELEM::BITS > 0 {
//             for elem in iter {
//                 self.insert_unchecked(insert_idx, elem);
//             }
//         } else {
//             for _ in iter {
//                 self.len += 1;
//             }
//         }
//     }

//     #[inline]
//     pub fn remove(&mut self, idx: usize) -> Result<ELEM::Base, String> {
//         match idx >= self.len {
//             true => Err(format!("index out of bounds for remove:\n\tlen = {}\n\tidx = {}", self.len, idx)),
//             false => Ok(unsafe{self.remove_unchecked(idx)}),
//         }
//     }

//     #[inline]
//     pub unsafe fn remove_unchecked(&mut self, idx: usize) -> ELEM::Base {
//         self.len -= 1;
//         if ELEM::BITS > 0 {
//             let idx_access = Self::create_elem_accessor(idx);
//             let shift_access = Self::create_elem_accessor(idx+1);
//             let val_bits = self.replace_val_with_elem_accessor(idx_access, 0);
//             self.shift_elements_down_with_with_elem_accessor(idx_access, shift_access, 1);
//             ELEM::bits_to_val(val_bits)
//         } else {
//             ELEM::bits_to_val(0)
//         }
//     }

//     #[inline]
//     pub fn remove_range<RB>(&mut self, range: RB) -> Result<Self, String>
//     where RB: RangeBounds<usize> {
//         let true_range = RangeUtil::get_real_bounds_for_veclike(range, self.len);
//         match true_range.end > self.len {
//             true => Err(format!("index out of bounds for remove range:\n\tvec len = {}\n\tend idx = {}", self.len, true_range.end)),
//             false => {
//                 let new_cap = true_range.end - true_range.start;
//                 let mut new_vec = Self::with_capacity(new_cap)?;
//                 unsafe {self.remove_range_unchecked(true_range, &mut new_vec)};
//                 Ok(new_vec)
//             },
//         }
//     }

//     #[inline]
//     pub unsafe fn remove_range_unchecked<RB>(&mut self, range: RB, bitvec_with_enough_capacity: &mut Self)
//     where RB: RangeBounds<usize> {
//         let true_range = RangeUtil::get_real_bounds_for_veclike(range, self.len);
//         if ELEM::BITS > 0 {
//             let true_end = true_range.end;
//             let true_start = true_range.start;
//             for idx in true_range {
//                 let access = Self::create_elem_accessor(idx);
//                 let val = self.replace_val_with_elem_accessor(access, 0);
//                 bitvec_with_enough_capacity.push_unchecked(ELEM::bits_to_val(val));
//             }
//             if true_end == self.len {
//                 self.len = true_start;
//             } else {
//                 let count = true_end - true_start;
//                 let begin_access = Self::create_elem_accessor(true_start);
//                 let shift_access = Self::create_elem_accessor(true_end);
//                 self.shift_elements_down_with_with_elem_accessor(begin_access, shift_access, count);
//                 self.len -= count;
//             }
//         } else {
//             bitvec_with_enough_capacity.len += true_range.end-true_range.start;
//         }
//     }

//     #[inline]
//     pub fn swap(&mut self, idx_a: usize, idx_b: usize) -> Result<(), String> {
//         if idx_a >= self.len || idx_b >= self.len {
//             return Err(format!("index out of bounds for swap:\n\tlen   = {}\n\tidx_a = {}, idx_b = {}", self.len, idx_a, idx_b))
//         } else if idx_a != idx_b {
//             unsafe {self.swap_unchecked(idx_a, idx_b)};
//         }
//         Ok(())
//     }

//     #[inline]
//     pub unsafe fn swap_unchecked(&mut self, idx_a: usize, idx_b: usize) {
//         if ELEM::BITS > 0 {
//             let access_a = Self::create_elem_accessor(idx_a);
//             let access_b = Self::create_elem_accessor(idx_b);
//             self.swap_vals_with_elem_accessors(access_a, access_b)
//         }
//     }

//     #[inline]
//     pub fn swap_pop(&mut self, idx: usize) -> Result<ELEM::Base, String> {
//         if idx >= self.len {
//             Err(format!("index out of bounds for swap pop:\n\tlen   = {}\n\tidx = {}", self.len, idx))
//         } else if idx == self.len - 1 {
//             Ok(unsafe{self.pop_unchecked()})
//         } else {
//             Ok(unsafe {self.swap_pop_unchecked(idx)})
//         }
//     }

//     #[inline]
//     pub unsafe fn swap_pop_unchecked(&mut self, idx: usize) -> ELEM::Base {
//         self.len -= 1;
//         if ELEM::BITS > 0 {
//             let access_last = Self::create_elem_accessor(self.len);
//             let access_idx = Self::create_elem_accessor(idx);
//             let val: usize = self.swap_pop_val_with_elem_accessor(access_idx, access_last);
//             ELEM::bits_to_val(val)
//         } else {
//             ELEM::bits_to_val(0)
//         }
//     }

    
//     #[inline]
//     pub fn shrink(&mut self, shrink: Shrink) -> Result<(), String> {
//         unsafe {self.handle_resize(ElementCount::Total(self.len), Resize::Shrink(shrink), true)}
//     }

//     #[inline]
//     pub fn set_exact_capacity(&mut self, new_cap: usize) -> Result<(), String> {
//         unsafe {self.handle_resize(ElementCount::Total(self.len), Resize::ExactCapacity(new_cap), true)}
//     }

//     #[inline]
//     pub fn append_iter<II>(&mut self, source: II) -> Result<(), String>
//     where II: IntoIterator<Item = ELEM::Base> {
//         self.append_iter_custom_grow(source, Self::DEFAULT_GROW)
//     }

//     #[inline]
//     pub fn append_iter_custom_grow<II>(&mut self, source: II, grow: Grow) -> Result<(), String>
//     where II: IntoIterator<Item = ELEM::Base> {
//         let iter = source.into_iter();
//         let projected_additional = match iter.size_hint() {
//             (_, Some(upper_bound)) => upper_bound,
//             (lower_bound, None) => lower_bound
//         };
//         self.grow_if_needed_custom(ElementCount::Additional(projected_additional), grow)?;
//         for elem in iter {
//             self.push_custom_grow(elem, grow)?;
//         }
//         Ok(())
//     }

//     #[inline]
//     pub fn clone_val(&self, idx: usize) -> Result<ELEM::Base, String> 
//     where ELEM::Base: Clone {
//         if idx < self.len {
//             Ok(unsafe{self.clone_val_unchecked(idx)})
//         } else {
//             Err(format!("index out of bounds for clone:\n\tlen = {}\n\tidx = {}", self.len, idx))
//         }
//     }

//     #[inline]
//     pub unsafe fn clone_val_unchecked(&self, idx: usize) -> ELEM::Base
//     where ELEM::Base: Clone {
//         if ELEM::BITS > 0 {
//             let idx_access = Self::create_elem_accessor(idx);
//             let val_bits = self.read_val_with_elem_accessor(idx_access);
//             ELEM::bits_to_val(val_bits)
//         } else {
//             ELEM::bits_to_val(0)
//         }
//     }

//     #[inline]
//     pub fn replace(&self, idx: usize, val: ELEM::Base) -> Result<ELEM::Base, String> {
//         if idx < self.len {
//             Ok(unsafe{self.replace_unchecked(idx, val)})
//         } else {
//             Err(format!("index out of bounds for clone:\n\tlen = {}\n\tidx = {}", self.len, idx))
//         }
//     }

//     #[inline]
//     pub unsafe fn replace_unchecked(&self, idx: usize, val: ELEM::Base) -> ELEM::Base {
//         if ELEM::BITS > 0 {
//             let idx_access = Self::create_elem_accessor(idx);
//             let out_bits = self.replace_val_with_elem_accessor(idx_access, ELEM::val_to_bits(val));
//             ELEM::bits_to_val(out_bits)
//         } else {
//             ELEM::bits_to_val(0)
//         }
//     }

//     #[inline]
//     pub fn set(&mut self, idx: usize, val: ELEM::Base) -> Result<(), String> {
//         if idx < self.len {
//             unsafe{self.set_unchecked(idx, val)};
//             Ok(())
//         } else {
//             Err(format!("index out of range for set: idx = {}, len = {}", idx, self.len))
//         }
//     }

//     #[inline]
//     pub unsafe fn set_unchecked(&mut self, idx: usize, val: ELEM::Base) {
//         if ELEM::BITS > 0 {
//             let idx_access = Self::create_elem_accessor(idx);
//             self.write_val_with_elem_accessor(idx_access, ELEM::val_to_bits(val));
//         }
//     }

//     pub fn drain<'vec>(&'vec mut self) -> BVecDrain<'vec, ELEM> {
//         let drain = BVecDrain {
//             vec: PhantomData,
//             ptr: self.ptr,
//             start: 0,
//             count: self.len
//         };
//         self.len = 0;
//         drain
//     }

//     #[inline]
//     pub(crate) const fn create_elem_accessor(sub_idx: usize) -> ElemAccess {
//         let (start_idx, first_offset) = Self::calc_sub_idx_to_real_idx_and_bit_offset(sub_idx);
//         let second_offset = BitUtil::USIZE_BITS - first_offset;
//         let first_mask = ELEM::MASK << first_offset;
//         let second_mask = BitUtil::right_shift_discard_if_ubits(ELEM::MASK, second_offset);
//         ElemAccess {
//             start_idx,
//             first_offset,
//             second_offset,
//             first_mask,
//             second_mask
//         }
//     }

//     // #[inline]
//     // pub(crate) const unsafe fn clean_new_blocks_if_needed(&mut self, len_access: ElemAccess, additional_elements: usize) {
//     //     let mut total_new_bits = additional_elements * ELEM::BITS;
//     //     let mut already_clean = len_access.first_offset | (len_access.first_offset << 1);
//     //     already_clean |= already_clean << 2;
//     //     already_clean |= already_clean << 4;
//     //     already_clean = already_clean & BitUtil::USIZE_BITS;
//     //     already_clean -= len_access.first_offset;
//     //     total_new_bits = total_new_bits.saturating_sub(already_clean);
//     //     let mut whole_blocks= match usize::BITS {
//     //         64 => total_new_bits >> 6,
//     //         32 => total_new_bits >> 5,
//     //         _ => total_new_bits >> 4,
//     //     };
//     //     let mut rem_block = total_new_bits & BitUtil::USIZE_MAX_SHIFT;
//     //     rem_block |= rem_block >> 1;
//     //     rem_block |= rem_block >> 2;
//     //     rem_block |= rem_block >> 4;
//     //     rem_block &= 1;
//     //     whole_blocks += rem_block;
//     //     let mut block_ptr = self.ptr.as_ptr().add(len_access.start_idx + (already_clean & 1));
//     //     while whole_blocks > 0 {
//     //         ptr::write(block_ptr, 0);
//     //         block_ptr.add(1);
//     //     }
//     // }

//     #[inline]
//     pub(crate) unsafe fn read_val_with_elem_accessor(&self, accessor: ElemAccess) -> usize {
//         let mut block_ptr = self.ptr.as_ptr().add(accessor.start_idx);
//         let mut block_bits = ptr::read(block_ptr);
//         let mut val = (block_bits & accessor.first_mask) >> accessor.first_offset;
//         if accessor.second_mask != 0 {
//             block_ptr = block_ptr.add(1);
//             block_bits = ptr::read(block_ptr);
//             val = val | ((block_bits & accessor.second_mask) << accessor.second_offset);
//         }
//         val
//     }

//     #[inline]
//     pub(crate) unsafe fn replace_val_with_elem_accessor(&self, accessor: ElemAccess, new_val: usize) -> usize {
//         let mut block_ptr = self.ptr.as_ptr().add(accessor.start_idx);
//         let mut block_bits = ptr::read(block_ptr);
//         let mut val = (block_bits & accessor.first_mask) >> accessor.first_offset;
//         block_bits = (block_bits & !accessor.first_mask) | (new_val << accessor.first_offset);
//         ptr::write(block_ptr, block_bits);
//         if accessor.second_mask != 0 {
//             block_ptr = block_ptr.add(1);
//             block_bits = ptr::read(block_ptr);
//             val = val | ((block_bits & accessor.second_mask) << accessor.second_offset);
//             block_bits = (block_bits & !accessor.second_mask) | (new_val >> accessor.second_offset);
//             ptr::write(block_ptr, block_bits);
//         }
//         val
//     }

//     #[inline]
//     pub(crate) unsafe fn write_val_with_elem_accessor(&self, accessor: ElemAccess, new_val: usize) {
//         let mut block_ptr = self.ptr.as_ptr().add(accessor.start_idx);
//         let mut block_bits = ptr::read(block_ptr);
//         block_bits = (block_bits & !accessor.first_mask) | (new_val << accessor.first_offset);
//         ptr::write(block_ptr, block_bits);
//         if accessor.second_mask != 0 {
//             block_ptr = block_ptr.add(1);
//             block_bits = ptr::read(block_ptr);
//             block_bits = (block_bits & !accessor.second_mask) | (new_val >> accessor.second_offset);
//             ptr::write(block_ptr, block_bits);
//         }
//     }

//     // #[inline]
//     // pub(crate) unsafe fn clear_val_with_elem_accessor(&self, accessor: ElemAccess) {
//     //     let mut block_ptr = self.ptr.as_ptr().add(accessor.start_idx);
//     //     let mut block_bits = ptr::read(block_ptr);
//     //     block_bits = block_bits & !accessor.first_mask;
//     //     ptr::write(block_ptr, block_bits);
//     //     if accessor.second_mask != 0 {
//     //         block_ptr = block_ptr.add(1);
//     //         block_bits = ptr::read(block_ptr);
//     //         block_bits = block_bits & !accessor.second_mask;
//     //         ptr::write(block_ptr, block_bits);
//     //     }
//     // }

//     #[inline]
//     pub(crate) unsafe fn swap_vals_with_elem_accessors(&self, accessor_a: ElemAccess, accessor_b: ElemAccess) {
//         let val_a = self.replace_val_with_elem_accessor(accessor_a, 0);
//         let val_b = self.replace_val_with_elem_accessor(accessor_b, 0);
//         self.write_val_with_elem_accessor(accessor_a, val_b);
//         self.write_val_with_elem_accessor(accessor_b, val_a);
//     }

//     #[inline]
//     pub(crate) unsafe fn swap_pop_val_with_elem_accessor(&self, accessor: ElemAccess, accessor_last: ElemAccess) -> usize {
//         let val_last = self.replace_val_with_elem_accessor(accessor_last, 0);
//         let val = self.replace_val_with_elem_accessor(accessor, val_last);
//         val
//     }

//     #[inline]
//     pub(crate) unsafe fn shift_elements_up_with_with_elem_accessor(&self, begin_access: ElemAccess, shift_count: usize) {
//         let new_real_len = Self::calc_real_count_from_sub_count(self.len + shift_count);
//         let block_count = new_real_len - begin_access.start_idx;
//         let mut block_ptr = self.ptr.as_ptr().add(begin_access.start_idx);
//         let mut block_bits = ptr::read(block_ptr);
//         let keep_first_mask = BitUtil::all_bits_less_than_bit(begin_access.first_offset);
//         let keep_first_bits = block_bits & keep_first_mask;
//         block_bits &= !keep_first_mask;
//         ptr::write(block_ptr, block_bits);
//         let total_bits_shifted = shift_count * ELEM::BITS;
//         let whole_blocks = total_bits_shifted / BitUtil::USIZE_BITS;
//         if whole_blocks > 0 {
//             ptr::copy(block_ptr, block_ptr.add(whole_blocks), block_count);
//             block_ptr = block_ptr.add(whole_blocks)
//         }
//         let rollover_bits = total_bits_shifted - (whole_blocks * BitUtil::USIZE_BITS);
//         if rollover_bits > 0 {
//             let rollover_shift = BitUtil::USIZE_BITS - rollover_bits;
//             let rollover_mask = usize::MAX << rollover_shift;
//             let mut blocks_shifted = 0;
//             let mut rollover_bits_paste: usize = 0; 
//             let mut rollover_bits_copy: usize; 
//             while blocks_shifted < block_count {
//                 block_bits = ptr::read(block_ptr);
//                 rollover_bits_copy = (block_bits & rollover_mask) >> rollover_shift;
//                 block_bits = (block_bits << rollover_bits) | rollover_bits_paste;
//                 ptr::write(block_ptr, block_bits);
//                 block_ptr = block_ptr.add(1);
//                 blocks_shifted += 1;
//                 rollover_bits_paste = rollover_bits_copy;
//             }
//         }
//         block_ptr = self.ptr.as_ptr().add(begin_access.start_idx);
//         block_bits = ptr::read(block_ptr);
//         ptr::write(block_ptr, block_bits | keep_first_bits);
//     }

//     #[inline]
//     pub(crate) unsafe fn shift_elements_down_with_with_elem_accessor(&self, begin_access: ElemAccess, shift_access: ElemAccess, shift_count: usize) {
//         let real_len = Self::calc_real_count_from_sub_count(self.len);
//         let block_count = real_len - shift_access.start_idx;
//         let mut block_ptr = self.ptr.as_ptr().add(begin_access.start_idx);
//         let mut block_bits = ptr::read(block_ptr);
//         let keep_first_mask = BitUtil::all_bits_less_than_bit(begin_access.first_offset);
//         let keep_first_bits = block_bits & keep_first_mask;
//         block_bits &= !keep_first_mask;
//         ptr::write(block_ptr, block_bits);
//         let total_bits_shifted = shift_count * ELEM::BITS;
//         let whole_blocks = total_bits_shifted / BitUtil::USIZE_BITS;
//         if whole_blocks > 0 {
//             ptr::copy(block_ptr.add(whole_blocks), block_ptr, block_count);
//         }
//         let rollover_bits = total_bits_shifted - (whole_blocks * BitUtil::USIZE_BITS);
//         if rollover_bits > 0 {
//             let rollover_shift = BitUtil::USIZE_BITS - rollover_bits;
//             let rollover_mask = usize::MAX >> rollover_shift;
//             let mut blocks_shifted = 0;
//             let mut rollover_bits_paste: usize = 0; 
//             let mut rollover_bits_copy: usize; 
//             block_ptr = self.ptr.as_ptr().add(real_len-whole_blocks-1);
//             while blocks_shifted < block_count {
//                 block_bits = ptr::read(block_ptr);
//                 rollover_bits_copy = (block_bits & rollover_mask) << rollover_shift;
//                 block_bits = (block_bits >> rollover_bits) | rollover_bits_paste;
//                 ptr::write(block_ptr, block_bits);
//                 block_ptr = block_ptr.sub(1);
//                 blocks_shifted += 1;
//                 rollover_bits_paste = rollover_bits_copy;
//             }
//         }
//         block_ptr = self.ptr.as_ptr().add(begin_access.start_idx);
//         block_bits = ptr::read(block_ptr);
//         ptr::write(block_ptr, block_bits | keep_first_bits);
//     }

//     #[inline]
//     pub(crate) const fn calc_sub_idx_to_real_idx_and_bit_offset(elem_idx: usize) -> (usize, usize) {
//         match BitUtil::USIZE_BITS {
//             64 => match ELEM::BITS {
//                 1 => (elem_idx >> 6, elem_idx & 0b_00111111),
//                 2 => (elem_idx >> 5, (elem_idx & 0b_00011111) << 1),
//                 4 => (elem_idx >> 4, (elem_idx & 0b_00001111) << 2),
//                 8 => (elem_idx >> 3, (elem_idx & 0b_00000111) << 3),
//                 16 => (elem_idx >> 2, (elem_idx & 0b_00000011) << 4),
//                 32 => (elem_idx >> 1, (elem_idx & 0b_00000001) << 5),
//                 64 => (elem_idx, 0),
//                 128 => (elem_idx << 1, 0),
//                 _ => {
//                     let total_bits = elem_idx * ELEM::BITS;
//                     (total_bits >> 6, total_bits & 0b_00111111)
//                 } 
//             },
//             32 => match ELEM::BITS {
//                 1 => (elem_idx >> 5, elem_idx & 0b_00011111),
//                 2 => (elem_idx >> 4, (elem_idx & 0b_00001111) << 1),
//                 4 => (elem_idx >> 3, (elem_idx & 0b_00000111) << 2),
//                 8 => (elem_idx >> 2, (elem_idx & 0b_00000011) << 3),
//                 16 => (elem_idx >> 1, (elem_idx & 0b_00000001) << 4),
//                 32 => (elem_idx, 0),
//                 64 => (elem_idx << 1, 0),
//                 128 => (elem_idx << 2, 0),
//                 _ => {
//                     let total_bits = elem_idx * ELEM::BITS;
//                     (total_bits >> 5, total_bits & 0b_00011111)
//                 } 
//             },
//             16 => match ELEM::BITS {
//                 1 => (elem_idx >> 4, elem_idx & 0b_00001111),
//                 2 => (elem_idx >> 3, (elem_idx & 0b_00000111) << 1),
//                 4 => (elem_idx >> 2, (elem_idx & 0b_00000011) << 2),
//                 8 => (elem_idx >> 1, (elem_idx & 0b_00000001) << 3),
//                 16 => (elem_idx, 0),
//                 32 => (elem_idx << 1, 0),
//                 64 => (elem_idx << 2, 0),
//                 128 => (elem_idx << 3, 0),
//                 _ => {
//                     let total_bits = elem_idx * ELEM::BITS;
//                     (total_bits >> 4, total_bits & 0b_00001111)
//                 } 
//             }
//             _ => {
//                 let total_bits = elem_idx * ELEM::BITS;
//                 (total_bits / BitUtil::USIZE_BITS, total_bits % BitUtil::USIZE_BITS)
//             }
//         }
//     }

//     // #[inline]
//     // pub(crate) const fn calc_end_real_idx_from_start_real_idx_and_bit_offset(real_idx: usize, bit_off: usize) -> usize {
//     //     let bit_end = bit_off + ELEM::BITS;
//     //     match BitUtil::USIZE_BITS {
//     //         64 => real_idx + (bit_end >> 6),
//     //         32 => real_idx + (bit_end >> 5),
//     //         16 => real_idx + (bit_end >> 4),
//     //         _ => real_idx + (bit_end / BitUtil::USIZE_BITS)
//     //     }
//     // }

//     #[inline]
//     pub(crate) const fn calc_real_count_from_sub_count(count: usize) -> usize {
//         let (real_idx, bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(count);
//         real_idx + BitUtil::one_if_val_isnt_zero(bit_off)
//     }

//     #[inline]
//     #[cfg(test)]
//     pub(crate) fn calc_real_len_from_sub_len(&self) -> usize {
//         Self::calc_real_count_from_sub_count(self.len)
//     }

//     // #[inline]
//     // #[cfg(test)]
//     // pub(crate) const fn calc_real_cap_from_sub_cap(&self) -> usize {
//     //     Self::calc_real_count_from_sub_count(self.cap)
//     // }

//     #[inline]
//     pub(crate) const fn calc_elem_count_from_total_bits(total_bits: usize) -> usize {
//         match ELEM::BITS {
//             1 => total_bits,
//             2 => total_bits >> 1,
//             4 => total_bits >> 2,
//             8 => total_bits >> 3,
//             16 => total_bits >> 4,
//             32 => total_bits >> 5,
//             64 => total_bits >> 6,
//             128 => total_bits >> 7,
//             _ => total_bits / ELEM::BITS
//         }
//     }

//     #[inline]
//     pub(crate) const fn calc_sub_cap_from_real_cap(real_cap: usize) -> usize {
//         let total_bits = BitUtil::calc_total_bits_in_num_usize(real_cap);
//         Self::calc_elem_count_from_total_bits(total_bits)
//     }

//     #[inline]
//     pub(crate) unsafe fn handle_resize(&mut self, element_count: ElementCount, resize: Resize, force_realloc: bool) -> Result<(), String> {
//         let target_len = match element_count {
//             ElementCount::Total(len) => len,
//             ElementCount::Additional(count) if usize::MAX - count > self.len => self.len + count,
//             ElementCount::Additional(count) => return Err(format!("{} additional elements would overflow usize::MAX:\n\tusize::MAX = {}\n\trequested  = {}", count, usize::MAX, self.len as u128 + count as u128))
//         };
//         if ELEM::BITS > 0 {
//             if target_len > MemUtil::MAX_CAPACITY_FOR_USIZE {
//                 return Err(format!("resize would overflow user memory space:\n\tuser memory space = {} bytes\n\trequested memory =  {} x {} bytes\n\trequested memory =  {} bytes", isize::MAX, target_len, BitUtil::USIZE_BYTES, target_len * BitUtil::USIZE_BYTES));
//             }
//             if (force_realloc && target_len != self.cap) || target_len > self.cap {
//                 let new_cap = match resize {
//                     Resize::ExactCapacity(cap) => cap,
//                     Resize::Grow(grow) => match grow {
//                         Grow::Exact => target_len,
//                         Grow::ExactPlus(count) => target_len.saturating_add(count).min(MemUtil::MAX_CAPACITY_FOR_USIZE),
//                         Grow::OnePointFive => target_len.saturating_add(target_len >> 1).min(MemUtil::MAX_CAPACITY_FOR_USIZE),
//                         Grow::Double => target_len.saturating_mul(2).min(MemUtil::MAX_CAPACITY_FOR_USIZE),
//                     },
//                     Resize::Shrink(shrink) => match shrink {
//                         Shrink::Minimum => target_len,
//                         Shrink::SubtractOrMinimum(count) => target_len.max(self.cap - count),
//                         Shrink::SubtractTruncate(count) => self.cap - count,
//                         Shrink::ThreeQuartersOrMinimum => target_len.max((self.cap >> 1) + (self.cap >> 2)),
//                         Shrink::ThreeQuartersTruncate => (self.cap >> 1) + (self.cap >> 2),
//                         Shrink::HalfOrMinimum => target_len.max(self.cap >> 1),
//                         Shrink::HalfTruncate => self.cap >> 1,
//                     },
//                 };
//                 if needs_drop::<ELEM::Base>() {
//                     while self.len > new_cap {
//                         let _ = self.pop_unchecked();
//                     }
//                 }
//                 let target_real_capacity = Self::calc_real_count_from_sub_count(new_cap);
//                 let current_real_capacity = Self::calc_real_count_from_sub_count(self.cap);
//                 let new_layout: Layout = Layout::from_size_align_unchecked(target_real_capacity*size_of::<usize>(), align_of::<usize>());
//                 let new_ptr = match self.cap {
//                     0 => {
//                         alloc::alloc(new_layout)
//                     },
//                     _ => {
//                         let old_layout = Layout::from_size_align_unchecked(current_real_capacity*size_of::<usize>(), align_of::<usize>());
//                         alloc::realloc(self.ptr.as_ptr().cast(), old_layout, new_layout.size())
//                     },
//                 };
//                 let new_sub_capacity = Self::calc_sub_cap_from_real_cap(target_real_capacity);
//                 match NonNull::new(new_ptr) {
//                     Some(non_null) => {
                        
//                         self.ptr = non_null.cast();
//                         self.cap = new_sub_capacity;
//                         if target_len > self.len {
//                             let real_target_len = Self::calc_real_count_from_sub_count(target_len);
//                             let real_current_len = Self::calc_real_count_from_sub_count(self.len);
//                             let mut len_diff = real_target_len - real_current_len;
//                             let mut block_ptr = self.ptr.as_ptr().add(real_current_len);
//                             while len_diff > 0 {
//                                 ptr::write(block_ptr, 0);
//                                 len_diff -= 1;
//                                 block_ptr = block_ptr.add(1);
//                             }
//                         }
//                         Ok(())
//                     },
//                     None => Err(format!("memory allocation failed:\n\tlayout = {:?}", new_layout)),
//                 }
//             } else {
//                 if target_len > self.len {
//                     let real_target_len = Self::calc_real_count_from_sub_count(target_len);
//                     let real_current_len = Self::calc_real_count_from_sub_count(self.len);
//                     let mut len_diff = real_target_len - real_current_len;
//                     let mut block_ptr = self.ptr.as_ptr().add(real_current_len);
//                     while len_diff > 0 {
//                         ptr::write(block_ptr, 0);
//                         len_diff -= 1;
//                         block_ptr = block_ptr.add(1);
//                     }
//                 }
//                 Ok(())
//             }
//         } else {
//             Ok(())
//         }
//     }
// }

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