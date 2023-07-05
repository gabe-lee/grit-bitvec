use crate::{
    mem,
    size_of,
    ptr,
    NonNull,
    PhantomData,
    alloc,
    Layout,
    x_vec::{
        XVecIndex,
        ElementCount,
        Grow,
        Shrink
    },
    utils::{
        MemUtil,
        BitUtil
    }, 
    bx_vec_iter::{
        BXVecIter,
        BXVecDrain
    }
};

pub unsafe trait BXVecElem
where <Self as BXVecElem>::Base: Copy + Clone {
    type Base;
    const BITS: usize;
    const MASK: usize = (1 << Self::BITS) - 1;
    fn bits_to_val(bits: usize) -> Self::Base;
    fn val_to_bits(val: Self::Base) -> usize;
}

macro_rules! impl_bxvec_elem_unsigned {
    ($BASE:ty, $TYPE:ident, $BITS:expr) => {
        #[allow(non_camel_case_types)]
        pub struct $TYPE;
        unsafe impl BXVecElem for $TYPE {
            type Base = $BASE;
            const BITS: usize = $BITS;
            #[inline(always)]
            fn bits_to_val(bits: usize) -> Self::Base {
                bits as Self::Base
            }
            #[inline(always)]
            fn val_to_bits(val: Self::Base) -> usize {
                (val as usize) & Self::MASK
            }
        }
        impl $TYPE {
            pub const MIN: $BASE = 0;
            pub const MAX: $BASE = (1 << Self::BITS) - 1;
        }
    };
}
macro_rules! impl_bxvec_elem_signed {
    ($BASE:ty, $TYPE:ident, $BITS:expr) => {
        #[allow(non_camel_case_types)]
        pub struct $TYPE;
        unsafe impl BXVecElem for $TYPE {
            type Base = $BASE;
            const BITS: usize = $BITS;
            #[inline(always)]
            fn bits_to_val(bits: usize) -> Self::Base {
                BitUtil::smear_neg_bit_left(bits, Self::TOP_BIT) as Self::Base
            }
            #[inline(always)]
            fn val_to_bits(val: Self::Base) -> usize {
                let mut neg_bit = (val & Self::Base::MIN) as usize;
                neg_bit >>= Self::Base::BITS as usize - Self::BITS;
                (neg_bit | (val as usize)) & Self::MASK
            }
        }
        impl $TYPE {
            pub(crate) const TOP_BIT: usize = 1 << (Self::BITS - 1);
            pub const MIN: $BASE = -(Self::TOP_BIT as $BASE);
            pub const MAX: $BASE = (Self::TOP_BIT - 1) as $BASE;
        }
    };
}

unsafe impl BXVecElem for bool {
    type Base = bool;
    const BITS: usize = 1;
    #[inline(always)]
    fn bits_to_val(bits: usize) -> Self::Base {
        (bits & 1) == 1
    }
    #[inline(always)]
    fn val_to_bits(val: Self::Base) -> usize {
        val as usize
    }
}
impl_bxvec_elem_unsigned!(u8, u8_as_u1, 1);
impl_bxvec_elem_unsigned!(u8, u8_as_u2, 2);
impl_bxvec_elem_unsigned!(u8, u8_as_u3, 3);
impl_bxvec_elem_unsigned!(u8, u8_as_u4, 4);
impl_bxvec_elem_unsigned!(u8, u8_as_u5, 5);
impl_bxvec_elem_unsigned!(u8, u8_as_u6, 6);
impl_bxvec_elem_unsigned!(u8, u8_as_u7, 7);
impl_bxvec_elem_signed!(i8, i8_as_i1, 1);
impl_bxvec_elem_signed!(i8, i8_as_i2, 2);
impl_bxvec_elem_signed!(i8, i8_as_i3, 3);
impl_bxvec_elem_signed!(i8, i8_as_i4, 4);
impl_bxvec_elem_signed!(i8, i8_as_i5, 5);
impl_bxvec_elem_signed!(i8, i8_as_i6, 6);
impl_bxvec_elem_signed!(i8, i8_as_i7, 7);
impl_bxvec_elem_unsigned!(u16, u16_as_u9, 9);
impl_bxvec_elem_unsigned!(u16, u16_as_u10, 10);
impl_bxvec_elem_unsigned!(u16, u16_as_u11, 11);
impl_bxvec_elem_unsigned!(u16, u16_as_u12, 12);
impl_bxvec_elem_unsigned!(u16, u16_as_u13, 13);
impl_bxvec_elem_unsigned!(u16, u16_as_u14, 14);
impl_bxvec_elem_unsigned!(u16, u16_as_u15, 15);
impl_bxvec_elem_signed!(i16, i16_as_i9, 9);
impl_bxvec_elem_signed!(i16, i16_as_i10, 10);
impl_bxvec_elem_signed!(i16, i16_as_i11, 11);
impl_bxvec_elem_signed!(i16, i16_as_i12, 12);
impl_bxvec_elem_signed!(i16, i16_as_i13, 13);
impl_bxvec_elem_signed!(i16, i16_as_i14, 14);
impl_bxvec_elem_signed!(i16, i16_as_i15, 15);

pub struct BXVec<ELEM, IDX>
where IDX: XVecIndex, ELEM: BXVecElem {
    pub(crate) ptr: NonNull<usize>,
    pub(crate) cap: IDX,
    pub(crate) len: IDX,
    sub: PhantomData<ELEM>,
}

impl<ELEM, IDX> BXVec<ELEM, IDX>
where IDX: XVecIndex, ELEM: BXVecElem {
    const UBITS: usize = usize::BITS as usize;

    #[inline(always)]
    pub(crate) fn calc_sub_idx_to_real_idx_and_bit_offset(sub_idx: IDX) -> (usize, usize) {
        let sub_usize = sub_idx.to_usize();
        let (real_idx, bit_off) = match usize::BITS {
            64 => match ELEM::BITS {
                1 => (sub_usize >> 6, sub_usize & 0b_00111111),
                2 => (sub_usize >> 5, sub_usize & 0b_00011111),
                4 => (sub_usize >> 4, sub_usize & 0b_00001111),
                8 => (sub_usize >> 3, sub_usize & 0b_00000111),
                16 => (sub_usize >> 2, sub_usize & 0b_00000011),
                32 => (sub_usize >> 1, sub_usize & 0b_00000001),
                64 => (sub_usize, 0),
                128 => (sub_usize << 1, 0),
                _ => {
                    let total_bits = sub_usize * ELEM::BITS;
                    (total_bits >> 6, total_bits & 0b_00111111)
                } 
            },
            32 => match ELEM::BITS {
                1 => (sub_usize >> 5, sub_usize & 0b_00011111),
                2 => (sub_usize >> 4, sub_usize & 0b_00001111),
                4 => (sub_usize >> 3, sub_usize & 0b_00000111),
                8 => (sub_usize >> 2, sub_usize & 0b_00000011),
                16 => (sub_usize >> 1, sub_usize & 0b_00000001),
                32 => (sub_usize, 0),
                64 => (sub_usize << 1, 0),
                128 => (sub_usize << 2, 0),
                _ => {
                    let total_bits = sub_usize * ELEM::BITS;
                    (total_bits >> 5, total_bits & 0b_00011111)
                } 
            },
            16 => match ELEM::BITS {
                1 => (sub_usize >> 4, sub_usize & 0b_00001111),
                2 => (sub_usize >> 3, sub_usize & 0b_00000111),
                4 => (sub_usize >> 2, sub_usize & 0b_00000011),
                8 => (sub_usize >> 1, sub_usize & 0b_00000001),
                16 => (sub_usize, 0),
                32 => (sub_usize << 1, 0),
                64 => (sub_usize << 2, 0),
                128 => (sub_usize << 3, 0),
                _ => {
                    let total_bits = sub_usize * ELEM::BITS;
                    (total_bits >> 4, total_bits & 0b_00001111)
                } 
            }
            _ => {
                let total_bits = sub_usize * ELEM::BITS;
                (total_bits / (usize::BITS as usize), total_bits % (usize::BITS as usize))
            }
        };
        (real_idx, bit_off)
    }

    #[inline(always)]
    pub(crate) fn calc_end_real_idx_from_start_real_idx_and_bit_offset(real_idx: usize, bit_off: usize) -> usize {
        let bit_end = bit_off + ELEM::BITS;
        return real_idx + match usize::BITS {
            64 => bit_end >> 6,
            32 => bit_end >> 5,
            16 => bit_end >> 4,
            _ => bit_end / Self::UBITS
        }
    }

    #[inline(always)]
    pub(crate) fn calc_real_cap_from_sub_cap(sub_cap: IDX) -> usize {
        let (real_idx, bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(sub_cap);
        Self::calc_end_real_idx_from_start_real_idx_and_bit_offset(real_idx, bit_off)
    }

    #[inline(always)]
    pub(crate) fn calc_elem_count_from_total_bits(total_bits: usize) -> IDX {
        let elem_count = match ELEM::BITS {
            1 => total_bits,
            2 => total_bits >> 1,
            4 => total_bits >> 2,
            8 => total_bits >> 3,
            16 => total_bits >> 4,
            32 => total_bits >> 5,
            64 => total_bits >> 6,
            128 => total_bits >> 7,
            _ => total_bits / ELEM::BITS
        } as usize;
        IDX::from_usize(elem_count)
    }

    #[inline(always)]
    pub(crate) fn calc_total_bits_in_num_usize(num_usize: usize) -> usize {
        match usize::BITS {
            64 => num_usize << 6,
            32 => num_usize << 5,
            16 => num_usize << 4,
            _ => num_usize * Self::UBITS
        }
    }

    #[inline(always)]
    pub(crate) fn calc_sub_cap_from_real_cap(real_cap: usize) -> IDX {
        let total_bits = Self::calc_total_bits_in_num_usize(real_cap.to_usize());
        Self::calc_elem_count_from_total_bits(total_bits)
    }

    #[inline(always)]
    pub fn len(&self) -> IDX {
        self.len
    }

    #[inline(always)]
    pub fn cap(&self) -> IDX {
        self.cap
    }

    #[inline(always)]
    pub fn free(&self) -> IDX {
        self.cap - self.len
    }

    #[inline(always)]
    pub fn new() -> Self {
        if size_of::<ELEM::Base>() != 0 || ELEM::BITS == 0 {
            Self {
                ptr: NonNull::dangling(),
                cap: IDX::IDX_MAX,
                len: IDX::ZERO,
                sub: PhantomData,
            }
        } else {
            Self {
                ptr: NonNull::dangling(),
                cap: IDX::ZERO,
                len: IDX::ZERO,
                sub: PhantomData,
            }
        }
    }

    #[inline(always)]
    pub fn with_capacity(cap: IDX) -> Self {
        let mut new_vec = Self::new();
        new_vec.resize(cap);
        new_vec
    }

    #[inline]
    pub fn grow_if_needed(&mut self, needed: ElementCount<IDX>, mode: Grow<IDX>) {
        if size_of::<ELEM::Base>() != 0 || ELEM::BITS == 0 {
            let target_cap = match needed {
                ElementCount::Total(cap) => cap,
                ElementCount::Change(count) => self.len + count,
            };
            if target_cap > self.cap {
                self.grow(mode);
            }
        }
    }

    #[inline]
    pub fn grow(&mut self, mode: Grow<IDX>) {
        if size_of::<ELEM::Base>() != 0 || ELEM::BITS == 0 {
            let new_cap = match mode {
                Grow::Exact(val) => {
                    if val < self.cap {
                        return self.shrink(Shrink::Exact(val));
                    } else if val > self.cap {
                        val
                    } else {
                        return;
                    }
                },
                Grow::Add(count) => self.cap + count,
                Grow::OnePointFive => self.cap + (self.cap >> IDX::ONE).min(IDX::ONE),
                Grow::Double => (self.cap << IDX::ONE).min(IDX::ONE),
            };
            self.resize(new_cap)
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = IDX::ZERO
    }

    #[inline]
    fn resize(&mut self, target_capacity: IDX) {
        if size_of::<ELEM::Base>() != 0 {
            let old_u8_ptr = match self.cap != IDX::ZERO {
                true => Some(self.ptr.cast::<u8>().as_ptr()),
                false => None,
            };
            let target_real_capacity = Self::calc_real_cap_from_sub_cap(target_capacity).to_usize();
            let current_real_capacity = Self::calc_real_cap_from_sub_cap(self.cap).to_usize();
            let (new_non_null_u8_ptr, new_real_cap) = MemUtil::resize_memory_region_of_usize(
                old_u8_ptr,
                current_real_capacity,
                target_real_capacity,
            );
            self.ptr = new_non_null_u8_ptr.cast();
            self.cap = Self::calc_sub_cap_from_real_cap(new_real_cap);
        }
    }

    #[inline]
    pub fn push(&mut self, val: ELEM::Base, grow: Grow<IDX>) {
        if self.len == IDX::IDX_MAX {
            panic!("XVec is completely full");
        }
        if size_of::<ELEM::Base>() != 0 && ELEM::BITS != 0 {
            self.grow_if_needed(ElementCount::Change(IDX::ONE), grow);
        }
        unsafe {self.push_unchecked(val)};
    }

    #[inline]
    pub unsafe fn push_unchecked(&mut self, val: ELEM::Base) {
        if size_of::<ELEM::Base>() != 0 || ELEM::BITS == 0 {
            let (real_idx, bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(self.len);
            let val_bits = ELEM::val_to_bits(val);
            let this_block_bits_to_push = val_bits << bit_off;
            let next_block_bits_to_push = val_bits >> (Self::UBITS - bit_off);
            let mut block_ptr = self.ptr.as_ptr().add(real_idx);
            let mut block_bits = ptr::read(block_ptr);
            block_bits |= this_block_bits_to_push;
            ptr::write(block_ptr, block_bits);
            if next_block_bits_to_push > 0 {
                block_ptr = block_ptr.add(1);
                block_bits = next_block_bits_to_push;
                ptr::write(block_ptr, block_bits);
            }
        }
        self.len += IDX::ONE;
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<ELEM::Base> {
        if self.len == IDX::ZERO {
            None
        } else {
            Some(unsafe{self.pop_unchecked()})
        }
    }

    #[inline(always)]
    pub unsafe fn pop_unchecked(&mut self) -> ELEM::Base {
        self.len -= IDX::ONE;
        if size_of::<ELEM::Base>() != 0 || ELEM::BITS == 0 {
            let (real_idx, bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(self.len);
            let bit_diff = Self::UBITS - bit_off;
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
    pub unsafe fn insert_unchecked(&mut self, idx: IDX, val: ELEM::Base) {
        if size_of::<ELEM::Base>() != 0 || ELEM::BITS == 0 {
            let (mut last_idx, last_bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(self.len);
            last_idx = Self::calc_end_real_idx_from_start_real_idx_and_bit_offset(last_idx, last_bit_off);
            let (insert_idx, insert_bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(idx);
            let mut block_ptr = self.ptr.as_ptr().add(insert_idx);
            let mut block_bits = ptr::read(block_ptr);
            let keep_first_mask = BitUtil::all_bits_less_than_bit(insert_bit_off);
            let keep_first_bits = block_bits & keep_first_mask;
            let insert_mask = ELEM::MASK << insert_bit_off;
            block_bits &= !(insert_mask | keep_first_mask);
            ptr::write(block_ptr, block_bits);
            let rollover_shift = Self::UBITS - ELEM::BITS;
            let rollover_mask = ELEM::MASK << rollover_shift;
            let mut start_idx = insert_idx;
            let val_bits = ELEM::val_to_bits(val);
            let mut rollover_bits_last: usize = keep_first_bits | (val_bits << insert_bit_off); 
            let mut rollover_bits_this: usize; 
            while start_idx <= last_idx {
                block_bits = ptr::read(block_ptr);
                rollover_bits_this = (block_bits & rollover_mask) >> rollover_shift;
                block_bits = (block_bits << ELEM::BITS) | rollover_bits_last;
                ptr::write(block_ptr, block_bits);
                block_ptr = block_ptr.add(1);
                start_idx += 1;
                rollover_bits_last = rollover_bits_this;
            }
        }
        self.len += IDX::ONE;
    }

    #[inline]
    pub fn insert(&mut self, idx: IDX, val: ELEM::Base, grow: Grow<IDX>) {
        if self.len == IDX::IDX_MAX {
            panic!("XVec at maximum capcity")
        }
        if idx > self.len {
            panic!("insert index out of bounds");
        }
        if size_of::<ELEM>() != 0 {
            self.grow_if_needed(ElementCount::Change(IDX::ONE), grow);
        }
        unsafe {self.insert_unchecked(idx, val)};
    }

    #[inline(always)]
    pub fn remove(&mut self, idx: IDX) -> Option<ELEM::Base> {
        match idx >= self.len {
            true => None,
            false => Some(unsafe{self.remove_unchecked(idx)}),
        }
    }

    #[inline]
    pub unsafe fn remove_unchecked(&mut self, idx: IDX) -> ELEM::Base {
        self.len -= IDX::ONE;
        if size_of::<ELEM::Base>() != 0 || ELEM::BITS == 0 {
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
            let rollover_shift = Self::UBITS - ELEM::BITS;
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
            block_bits |= keep_first_bits;
            ptr::write(block_ptr, block_bits);
            ELEM::bits_to_val(val_bits)
        } else {
            ELEM::bits_to_val(0)
        }
    }

    
    #[inline(always)]
    pub fn shrink(&mut self, mode: Shrink<IDX>) {
        if size_of::<ELEM>() != 0 {
            let new_cap = match mode {
                Shrink::Exact(val) => {
                    if val > self.cap {
                        return self.grow(Grow::Exact(val));
                    } else if val < self.cap {
                        val
                    } else {
                        return;
                    }
                },
                Shrink::Minimum => self.len,
                Shrink::SubtractOrMinimum(count) => self.len.max(self.cap - count),
                Shrink::SubtractTruncate(count) => self.cap - count,
                Shrink::ThreeQuartersOrMinimum => self.len.max((self.cap >> IDX::ONE) + (self.cap >> IDX::TWO)),
                Shrink::ThreeQuartersTruncate => (self.cap >> IDX::ONE) + (self.cap >> IDX::TWO),
                Shrink::HalfOrMinimum => self.len.max(self.cap >> IDX::ONE),
                Shrink::HalfTruncate => self.cap >> IDX::ONE,
            };
            self.set_exact_capacity(new_cap);
        }
    }

    #[inline(always)]
    pub fn set_exact_capacity(&mut self, new_cap: IDX) {
        if size_of::<ELEM>() != 0 {
            if self.len > new_cap {
                let mut trim_count = self.len - new_cap;
                while trim_count > IDX::ZERO {
                    let _ = unsafe {self.pop_unchecked()};
                    trim_count -= IDX::ONE;
                }
                self.len = new_cap;
            }
            self.resize(new_cap)
        }
    }

    // pub fn combine_append(&mut self, mut other: Self, mode: Grow<IDX>) {
    //     if size_of::<ELEM>() != 0 {
    //         self.grow_if_needed(ElementCount::Change(other.len), mode);
    //         unsafe {ptr::copy_nonoverlapping(other.ptr.as_ptr(), self.ptr.as_ptr().add(self.len.to_usize()), other.len.to_usize())}
    //         unsafe{alloc::dealloc(other.ptr.as_ptr().cast(), Layout::from_size_align_unchecked(other.cap.to_usize()*size_of::<ELEM>(), align_of::<ELEM>()))};
    //     } else {
    //         if IDX::IDX_MAX - other.len > self.len {
    //             panic!("combine would overflow integer length")
    //         }
    //     }
    //     self.len += other.len;
    //     other.len = IDX::ZERO;
    //     other.cap = IDX::ZERO;
    //     other.ptr = NonNull::dangling();
    // }

    // pub fn drain_append(&mut self, other: &mut Self, mode: Grow<IDX>) {
    //     if size_of::<ELEM>() != 0 {
    //         self.grow_if_needed(ElementCount::Change(other.len), mode);
    //         unsafe {ptr::copy_nonoverlapping(other.ptr.as_ptr(), self.ptr.as_ptr().add(self.len.to_usize()), other.len.to_usize())}
    //     }
    //     other.len = IDX::ZERO;
    //     self.len += other.len;
    // }

    // pub fn clone_append(&mut self, other: &Self, mode: Grow<IDX>)
    // where ELEM: Clone {
    //     if size_of::<ELEM>() != 0 {
    //         self.grow_if_needed(ElementCount::Change(other.len), mode);
    //         for elem in other.iter() {
    //             unsafe{self.push_unchecked(elem.clone())};
    //         }
    //     }
    //     self.len += other.len;
    // }

    #[inline(always)]
    pub fn get_val_safe(&self, idx: IDX) -> Option<ELEM::Base> {
        if idx < self.len {
            Some(unsafe{self.get_val_unchecked(idx)})
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_val(&self, idx: IDX) -> ELEM::Base {
        if idx >= self.len {
            panic!("index out of range: idx = {}, len = {}", idx, self.len);
        }
        unsafe{self.get_val_unchecked(idx)}
    }

    #[inline(always)]
    pub unsafe fn get_val_unchecked(&self, idx: IDX) -> ELEM::Base {
        if size_of::<ELEM::Base>() != 0 || ELEM::BITS == 0 {
            let (real_idx, bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(idx);
            let bit_diff = Self::UBITS - bit_off;
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

    #[inline(always)]
    pub fn set_val_safe(&mut self, idx: IDX, val: ELEM::Base) -> Result<(), String> {
        if idx < self.len {
            unsafe{self.set_val_unchecked(idx, val)};
            Ok(())
        } else {
            Err(format!("index out of range: idx = {}, len = {}", idx, self.len))
        }
    }

    #[inline(always)]
    pub fn set_val(&mut self, idx: IDX, val: ELEM::Base) {
        if idx >= self.len {
            panic!("index out of range: idx = {}, len = {}", idx, self.len);
        }
        unsafe{self.set_val_unchecked(idx, val)}
    }

    #[inline(always)]
    pub unsafe fn set_val_unchecked(&mut self, idx: IDX, val: ELEM::Base) {
        if size_of::<ELEM::Base>() != 0 && ELEM::BITS != 0 {
            let (real_idx, bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(idx);
            let val_bits = ELEM::val_to_bits(val);
            let this_block_bits_to_push = val_bits << bit_off;
            let this_block_bits_mask = ELEM::MASK << bit_off;
            let next_block_bits_to_push = val_bits >> (Self::UBITS - bit_off);
            let next_block_bits_mask = ELEM::MASK >> (Self::UBITS - bit_off);
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

    pub fn drain<'vec>(&'vec mut self) -> BXVecDrain<'vec, ELEM, IDX> {
        let drain = BXVecDrain {
            vec: PhantomData,
            ptr: self.ptr,
            start: IDX::ZERO,
            count: self.len
        };
        self.len = IDX::ZERO;
        drain
    }
}

impl<ELEM, IDX> Drop for BXVec<ELEM, IDX>
where IDX: XVecIndex, ELEM: BXVecElem {
    fn drop(&mut self) {
        self.clear();
        if size_of::<ELEM>() != 0 {
            if self.cap > IDX::ZERO {
                let layout = Layout::array::<ELEM>(self.cap.to_usize()).unwrap();
                unsafe {alloc::dealloc(self.ptr.as_ptr().cast(), layout)};
            }
        }
    }
}

unsafe impl<ELEM, IDX> Send for BXVec<ELEM, IDX> where IDX: XVecIndex, ELEM: BXVecElem  {}
unsafe impl<ELEM, IDX> Sync for BXVec<ELEM, IDX> where IDX: XVecIndex, ELEM: BXVecElem  {}

impl<ELEM, IDX> Clone for BXVec<ELEM, IDX>
where IDX: XVecIndex, ELEM: BXVecElem , ELEM::Base: Clone {
    fn clone(&self) -> Self {
        if size_of::<ELEM>() == 0 {
            Self {
                ptr: self.ptr,
                cap: self.cap,
                len: self.len,
                sub: PhantomData
            }
        } else {
            let mut new_short_vec = Self {
                ptr: NonNull::dangling(),
                cap: self.len,
                len: self.len,
                sub: PhantomData
            };
            if self.len > IDX::ZERO {
                let (mut real_len, bit_off) = Self::calc_sub_idx_to_real_idx_and_bit_offset(self.len - IDX::ONE);
                real_len = Self::calc_end_real_idx_from_start_real_idx_and_bit_offset(real_len, bit_off) + 1;
                let (new_ptr, _) = MemUtil::resize_memory_region_of_usize(None, 0, real_len);
                unsafe{ptr::copy_nonoverlapping(self.ptr.as_ptr().cast(), new_ptr.as_ptr(), real_len)};
                new_short_vec.ptr = new_ptr.cast();
            }            
            new_short_vec
        }
    }
}



impl<ELEM, IDX> IntoIterator for BXVec<ELEM, IDX>
where IDX: XVecIndex, ELEM: BXVecElem {
    type Item = ELEM::Base;
    type IntoIter = BXVecIter<ELEM, IDX>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = BXVecIter{
            ptr: self.ptr,
            real_cap: IDX::from_usize(Self::calc_real_cap_from_sub_cap(self.cap)),
            start: IDX::ZERO,
            count: self.len,
            sub: PhantomData
        };
        mem::forget(self);
        iter
    }
}

