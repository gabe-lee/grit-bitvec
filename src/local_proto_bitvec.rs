use crate::{
    BitProto,
    RawBitVec,
    LProtoBitVecIter, 
    LProtoBitVecDrain,
    Range,
    ManuallyDrop
};

/// ## `LProtoBitVec`: "Local Prototype Bitwise Vector"  
/// A `BitVec` where the bit-width and masking data ([`BitProto`]) is saved directly in the struct
/// 
/// This is a thin wrapper around [`RawBitVec`] that simply calls the underlying raw method and passes the associated
/// [`BitProto`] along with it. Unlike [`RawBitVec`] this is safe because it is impossible to ever use the wrong [`BitProto`]
/// 
/// ### Pros
/// - Simpler, safer API than [`RawBitVec`]
/// - No mono-morphization (smaller binary than [`CProtoBitVec`](crate::const_proto_bitvec::CProtoBitVec))
/// - Can store [`LProtoBitVec`]'s in a homogenous collection (`Array`, [`Vec`], [`HashMap`](std::collections::HashMap), etc.)
/// - *Possible* better cache-locality compared to [`SProtoBitVec`](crate::SProtoBitVec) (no pointer to heap)
/// 
/// ### Cons
/// - A full copy of a [`BitProto`] stored in every [`LProtoBitVec`] (6 usize total)
/// - No constant-propogation optimizations
pub struct LProtoBitVec {
    pub(crate) proto: BitProto,
    pub(crate) vec: RawBitVec
}

impl LProtoBitVec {
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.vec.len
    }

    #[inline(always)]
    pub fn cap(&self) -> usize {
        unsafe {self.vec.cap(self.proto)}
    }

    #[inline(always)]
    pub fn free(&self) -> usize {
        unsafe{self.vec.free(self.proto)}
    }

    #[inline(always)]
    pub fn new(proto: BitProto) -> Self {
        Self { proto, vec: RawBitVec::new() }
    }

    #[inline(always)]
    pub fn with_capacity(proto: BitProto, cap: usize) -> Self {
        Self { proto, vec: RawBitVec::with_capacity(proto, cap) }
    }

    #[inline(always)]
    pub fn grow_exact_for_additional_elements_if_needed(&mut self, extra_elements: usize) -> Result<(), String> {
        unsafe {self.vec.grow_exact_for_additional_elements_if_needed(self.proto, extra_elements)}
    }

    #[inline(always)]
    pub fn grow_exact_for_total_elements_if_needed(&mut self, total_elements: usize) -> Result<(), String> {
        unsafe {self.vec.grow_exact_for_total_elements_if_needed(self.proto, total_elements)}
    }

    #[inline(always)]
    pub fn grow_for_additional_elements_if_needed(&mut self, extra_elements: usize) -> Result<(), String> {
        unsafe {self.vec.grow_for_additional_elements_if_needed(self.proto, extra_elements)}
    }

    #[inline(always)]
    pub fn grow_for_total_elements_if_needed(&mut self, total_elements: usize) -> Result<(), String> {
        unsafe {self.vec.grow_for_total_elements_if_needed(self.proto, total_elements)}
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.vec.clear()
    }

    #[inline(always)]
    pub fn push(&mut self, val: usize) -> Result<(), String> {
        unsafe {self.vec.push(self.proto, val)}
    }

    #[inline(always)]
    pub unsafe fn push_unchecked(&mut self, val: usize) {
        self.vec.push_unchecked(self.proto, val)
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Result<usize, String> {
        unsafe{self.vec.pop(self.proto)}
    }

    #[inline(always)]
    pub unsafe fn pop_unchecked(&mut self) -> usize {
        self.vec.pop_unchecked(self.proto)
    }

    #[inline(always)]
    pub fn insert(&mut self, idx: usize, val: usize) -> Result<(), String> {
        unsafe{self.vec.insert(self.proto, idx, val)}
    }

    #[inline(always)]
    pub unsafe fn insert_unchecked(&mut self, idx: usize, val: usize) {
        self.vec.insert_unchecked(self.proto, idx, val)
    }

    #[inline(always)]
    pub fn insert_bitvec(&mut self, insert_idx: usize, bitvec: Self) -> Result<(), String> {
        unsafe{self.vec.insert_bitvec(self.proto, insert_idx, bitvec.into_raw())}
    }

    #[inline(always)]
    pub unsafe fn insert_bitvec_unchecked(&mut self, insert_idx: usize, bitvec: Self) {
        self.vec.insert_bitvec_unchecked(self.proto, insert_idx, bitvec.into_raw())
    }

    #[inline(always)]
    pub fn remove(&mut self, idx: usize) -> Result<usize, String> {
        unsafe{self.vec.remove(self.proto, idx)}
    }

    #[inline(always)]
    pub unsafe fn remove_unchecked(&mut self, idx: usize) -> usize {
        self.vec.remove_unchecked(self.proto, idx)
    }

    #[inline(always)]
    pub fn remove_range(&mut self, idx_range: Range<usize>) -> Result<Self, String> {
        match unsafe{self.vec.remove_range(self.proto, idx_range)} {
            Ok(raw) => Ok(Self{
                proto: self.proto,
                vec: raw
            }),
            Err(e) => Err(e),
        }
    }

    #[inline(always)]
    pub unsafe fn remove_range_unchecked(&mut self, idx_range: Range<usize>) -> Self {
        Self {
            proto: self.proto,
            vec: self.vec.remove_range_unchecked(self.proto, idx_range)
        }
    }

    #[inline(always)]
    pub fn swap(&mut self, idx_a: usize, idx_b: usize) -> Result<(), String> {
        unsafe{self.vec.swap(self.proto, idx_a, idx_b)}
    }

    #[inline(always)]
    pub unsafe fn swap_unchecked(&mut self, idx_a: usize, idx_b: usize) {
        self.vec.swap_unchecked(self.proto, idx_a, idx_b)
}

    #[inline(always)]
    pub fn swap_pop(&mut self, idx: usize) -> Result<usize, String> {
        unsafe{self.vec.swap_pop(self.proto, idx)}
    }

    #[inline(always)]
    pub unsafe fn swap_pop_unchecked(&mut self, idx: usize) -> usize {
        self.vec.swap_pop_unchecked(self.proto, idx)
    }

    #[inline(always)]
    pub fn trim_excess_capacity(&mut self, extra_capacity_to_keep: usize) -> Result<(), String> {
        unsafe{self.vec.shrink_excess_capacity(self.proto, extra_capacity_to_keep)}
    }

    #[inline(always)]
    pub fn append_bitvec(&mut self, bitvec: Self) -> Result<(), String> {
        unsafe{self.vec.append_bitvec(self.proto, bitvec.into_raw())}
    }

    #[inline(always)]
    pub unsafe fn append_bitvec_unchecked(&mut self, bitvec: Self) {
        self.vec.append_bitvec_unchecked(self.proto, bitvec.into_raw())
    }

    #[inline(always)]
    pub fn append_iter<II, TO, ESI>(&mut self, source: II) -> Result<(), String>
    where II: IntoIterator<Item = TO, IntoIter = ESI>, TO: ToOwned<Owned = usize>, ESI: ExactSizeIterator + Iterator<Item = TO> {
        unsafe{self.vec.append_iter(self.proto, source)}
    }

    #[inline(always)]
    pub unsafe fn append_iter_unchecked<I, TO>(&mut self, iter: I)
    where I: Iterator<Item = TO> + ExactSizeIterator, TO: ToOwned<Owned = usize> {
        self.vec.append_iter_unchecked(self.proto, iter)
    }

    #[inline(always)]
    pub fn get(&self, idx: usize) -> Result<usize, String> {
        unsafe{self.vec.get(self.proto, idx)}
    }

    #[inline(always)]
    pub unsafe fn get_unchecked(&self, idx: usize) -> usize {
        self.vec.get_unchecked(self.proto, idx)
    }

    #[inline(always)]
    pub fn replace(&mut self, idx: usize, val: usize) -> Result<usize, String> {
        unsafe{self.vec.replace(self.proto, idx, val)}
    }

    #[inline(always)]
    pub unsafe fn replace_unchecked(&mut self, idx: usize, val: usize) -> usize {
        self.vec.replace_unchecked(self.proto, idx, val)
    }

    #[inline(always)]
    pub fn set(&mut self, idx: usize, val: usize) -> Result<(), String> {
        unsafe{self.vec.set(self.proto, idx, val)}
    }

    #[inline(always)]
    pub unsafe fn set_unchecked(&mut self, idx: usize, val: usize) {
        self.vec.set_unchecked(self.proto, idx, val)
    }

    #[inline(always)]
    pub fn drain<'vec>(&'vec mut self) -> LProtoBitVecDrain<'vec> {
        LProtoBitVecDrain{
            proto: self.proto,
            drain: self.vec.drain()
        }
    }

    #[inline(always)]
    pub unsafe fn into_raw(self) -> RawBitVec {
        let nodrop_self = ManuallyDrop::new(self);
        RawBitVec {
            ptr: nodrop_self.vec.ptr,
            len: nodrop_self.vec.len, 
            true_cap: nodrop_self.vec.true_cap 
        }
    }
}

impl IntoIterator for LProtoBitVec {
    type Item = usize;

    type IntoIter = LProtoBitVecIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        LProtoBitVecIter{
            proto: self.proto,
            iter: unsafe{self.into_raw().into_iter()}
        }
    }
}

impl Drop for LProtoBitVec {
    #[inline(always)]
    fn drop(&mut self) {/* RawBitVec::drop() will take care of the deallocation */}
}