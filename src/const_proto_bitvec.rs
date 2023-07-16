use crate::{
    BitProto,
    RawBitVec,
    CProtoBitVecDrain,
    CProtoBitVecIter,
    Range,
    ManuallyDrop
};

/// ## `CProtoBitVec`: "Constant Prototype Bitwise Vector"  
/// A `BitVec` where the bit-width and masking data ([`BitProto`]) is saved in a monomorphized constant for
/// each different value of `BIT_WIDTH`
/// 
/// This is a thin wrapper around [`RawBitVec`] that simply calls the underlying raw method and passes the associated
/// [`BitProto`] along with it. Unlike [`RawBitVec`] this is safe because it is impossible to ever use the wrong [`BitProto`]
/// 
/// ### Pros
/// - Simpler, safer API than [`RawBitVec`]
/// - Same stack-size as [`RawBitVec`] and [`Vec`] (3 usize)
/// - Allows for constant-propogation optimizations 
/// 
/// ### Cons
/// - Every separate value of `BIT_WIDTH` creates a distinct type with its own copy of all methods (larger binary)
/// - Cannot store [`CProtoBitVec`]'s with diferent `BIT_WIDTH`'s in the same homogenous collection (`Array`, [`Vec`], [`HashMap`](std::collections::HashMap), etc.)
pub struct CProtoBitVec<const BIT_WIDTH: usize>(pub(crate) RawBitVec);

impl<const BIT_WIDTH: usize> CProtoBitVec<BIT_WIDTH> {
    pub const PROTO: BitProto = BitProto::create(BIT_WIDTH);

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len
    }

    #[inline(always)]
    pub fn cap(&self) -> usize {
        unsafe {self.0.cap(Self::PROTO)}
    }

    #[inline(always)]
    pub fn free(&self) -> usize {
        unsafe{self.0.free(Self::PROTO)}
    }

    #[inline(always)]
    pub fn new() -> Self {
        Self(RawBitVec::new())
    }

    #[inline(always)]
    pub fn with_capacity(cap: usize) -> Self {
        Self(RawBitVec::with_capacity(Self::PROTO, cap))
    }

    #[inline(always)]
    pub fn grow_exact_for_additional_elements_if_needed(&mut self, extra_elements: usize) -> Result<(), String> {
        unsafe {self.0.grow_exact_for_additional_elements_if_needed(Self::PROTO, extra_elements)}
    }

    #[inline(always)]
    pub fn grow_exact_for_total_elements_if_needed(&mut self, total_elements: usize) -> Result<(), String> {
        unsafe {self.0.grow_exact_for_total_elements_if_needed(Self::PROTO, total_elements)}
    }

    #[inline(always)]
    pub fn grow_for_additional_elements_if_needed(&mut self, extra_elements: usize) -> Result<(), String> {
        unsafe {self.0.grow_for_additional_elements_if_needed(Self::PROTO, extra_elements)}
    }

    #[inline(always)]
    pub fn grow_for_total_elements_if_needed(&mut self, total_elements: usize) -> Result<(), String> {
        unsafe {self.0.grow_for_total_elements_if_needed(Self::PROTO, total_elements)}
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.0.clear()
    }

    #[inline(always)]
    pub fn push(&mut self, val: usize) -> Result<(), String> {
        unsafe {self.0.push(Self::PROTO, val)}
    }

    #[inline(always)]
    pub unsafe fn push_unchecked(&mut self, val: usize) {
        self.0.push_unchecked(Self::PROTO, val)
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Result<usize, String> {
        unsafe{self.0.pop(Self::PROTO)}
    }

    #[inline(always)]
    pub unsafe fn pop_unchecked(&mut self) -> usize {
        self.0.pop_unchecked(Self::PROTO)
    }

    #[inline(always)]
    pub fn insert(&mut self, idx: usize, val: usize) -> Result<(), String> {
        unsafe{self.0.insert(Self::PROTO, idx, val)}
    }

    #[inline(always)]
    pub unsafe fn insert_unchecked(&mut self, idx: usize, val: usize) {
        self.0.insert_unchecked(Self::PROTO, idx, val)
    }

    #[inline(always)]
    pub fn insert_bitvec(&mut self, insert_idx: usize, bitvec: Self) -> Result<(), String> {
        unsafe{self.0.insert_bitvec(Self::PROTO, insert_idx, bitvec.into_raw())}
    }

    #[inline(always)]
    pub unsafe fn insert_bitvec_unchecked(&mut self, insert_idx: usize, bitvec: Self) {
        self.0.insert_bitvec_unchecked(Self::PROTO, insert_idx, bitvec.into_raw())
    }

    #[inline(always)]
    pub fn remove(&mut self, idx: usize) -> Result<usize, String> {
        unsafe{self.0.remove(Self::PROTO, idx)}
    }

    #[inline(always)]
    pub unsafe fn remove_unchecked(&mut self, idx: usize) -> usize {
        self.0.remove_unchecked(Self::PROTO, idx)
    }

    #[inline(always)]
    pub fn remove_range(&mut self, idx_range: Range<usize>) -> Result<Self, String> {
        match unsafe{self.0.remove_range(Self::PROTO, idx_range)} {
            Ok(raw) => Ok(Self(raw)),
            Err(e) => Err(e),
        }
    }

    #[inline(always)]
    pub unsafe fn remove_range_unchecked(&mut self, idx_range: Range<usize>) -> Self {
        Self(self.0.remove_range_unchecked(Self::PROTO, idx_range))
    }

    #[inline(always)]
    pub fn swap(&mut self, idx_a: usize, idx_b: usize) -> Result<(), String> {
        unsafe{self.0.swap(Self::PROTO, idx_a, idx_b)}
    }

    #[inline(always)]
    pub unsafe fn swap_unchecked(&mut self, idx_a: usize, idx_b: usize) {
        self.0.swap_unchecked(Self::PROTO, idx_a, idx_b)
}

    #[inline(always)]
    pub fn swap_pop(&mut self, idx: usize) -> Result<usize, String> {
        unsafe{self.0.swap_pop(Self::PROTO, idx)}
    }

    #[inline(always)]
    pub unsafe fn swap_pop_unchecked(&mut self, idx: usize) -> usize {
        self.0.swap_pop_unchecked(Self::PROTO, idx)
    }

    #[inline(always)]
    pub fn trim_excess_capacity(&mut self, extra_capacity_to_keep: usize) -> Result<(), String> {
        unsafe{self.0.shrink_excess_capacity(Self::PROTO, extra_capacity_to_keep)}
    }
    #[inline(always)]
    pub fn append_bitvec(&mut self, bitvec: Self) -> Result<(), String> {
        unsafe{self.0.append_bitvec(Self::PROTO, bitvec.into_raw())}
    }
    #[inline(always)]
    pub unsafe fn append_bitvec_unchecked(&mut self, bitvec: Self) {
        self.0.append_bitvec_unchecked(Self::PROTO, bitvec.into_raw())
    }
    #[inline(always)]
    pub fn append_iter<II, TO, ESI>(&mut self, source: II) -> Result<(), String>
    where II: IntoIterator<Item = TO, IntoIter = ESI>, TO: ToOwned<Owned = usize>, ESI: ExactSizeIterator + Iterator<Item = TO> {
        unsafe{self.0.append_iter(Self::PROTO, source)}
    }
    #[inline(always)]
    pub unsafe fn append_iter_unchecked<I, TO>(&mut self, iter: I)
    where I: Iterator<Item = TO> + ExactSizeIterator, TO: ToOwned<Owned = usize> {
        self.0.append_iter_unchecked(Self::PROTO, iter)
    }

    #[inline(always)]
    pub fn get(&self, idx: usize) -> Result<usize, String> {
        unsafe{self.0.get(Self::PROTO, idx)}
    }

    #[inline(always)]
    pub unsafe fn get_unchecked(&self, idx: usize) -> usize {
        self.0.get_unchecked(Self::PROTO, idx)
    }

    #[inline(always)]
    pub fn replace(&mut self, idx: usize, val: usize) -> Result<usize, String> {
        unsafe{self.0.replace(Self::PROTO, idx, val)}
    }

    #[inline(always)]
    pub unsafe fn replace_unchecked(&mut self, idx: usize, val: usize) -> usize {
        self.0.replace_unchecked(Self::PROTO, idx, val)
    }

    #[inline(always)]
    pub fn set(&mut self, idx: usize, val: usize) -> Result<(), String> {
        unsafe{self.0.set(Self::PROTO, idx, val)}
    }

    #[inline(always)]
    pub unsafe fn set_unchecked(&mut self, idx: usize, val: usize) {
        self.0.set_unchecked(Self::PROTO, idx, val)
    }

    #[inline(always)]
    pub fn drain<'vec>(&'vec mut self) -> CProtoBitVecDrain<'vec, BIT_WIDTH> {
        CProtoBitVecDrain(self.0.drain())
    }

    #[inline(always)]
    pub unsafe fn into_raw(self) -> RawBitVec {
        let nodrop_self = ManuallyDrop::new(self);
        RawBitVec {
            ptr: nodrop_self.0.ptr,
            len: nodrop_self.0.len, 
            true_cap: nodrop_self.0.true_cap 
        }
    }
}

impl<const BIT_WIDTH: usize> IntoIterator for CProtoBitVec<BIT_WIDTH> {
    type Item = usize;

    type IntoIter = CProtoBitVecIter<BIT_WIDTH>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        CProtoBitVecIter(unsafe{self.into_raw().into_iter()})
    }
}

impl<const BIT_WIDTH: usize> Drop for CProtoBitVec<BIT_WIDTH> {
    #[inline(always)]
    fn drop(&mut self) {/* RawBitVec::drop() will take care of the allocation */}
}