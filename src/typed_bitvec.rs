use crate::{
    RawBitVec,
    TypedBitElem, 
    TypedBitVecDrain, 
    TypedBitVecIter,
    Range,
    ManuallyDrop,
    PhantomData
};

/// ## `TypedBitVec`: "Typed Bitwise Vector"  
/// A `BitVec` where all the data needed to access and translate the generic `<T>` to and from the expected `usize`
/// is stored in a monomorphized constant inside the trait [`TypedBitElem`]
/// 
/// This is a thin wrapper around [`RawBitVec`] that simply calls the underlying raw method and passes the associated
/// [`BitProto`](crate::BitProto) along with it, translating the input type to and from `usize` according to the specific implementation
/// of [`TypedBitElem`]. Unlike [`RawBitVec`] this is safe because it is impossible to ever use the wrong [`BitProto`](crate::BitProto)
/// 
/// ### Pros
/// - Simpler, safer API than [`RawBitVec`]
/// - Same stack-size as [`RawBitVec`] and [`Vec`] (3 usize)
/// - Allows for constant-propogation optimizations
/// - Values are automatically translated to and from the generic type `<T>`
/// 
/// ### Cons
/// - Every separate value of `<T>` creates a distinct type with its own copy of all methods (larger binary)
/// - Cannot store [`TypedBitVec`]'s with diferent `<T>`'s in the same homogenous collection (`Array`, [`Vec`], [`HashMap`](std::collections::HashMap), etc.)
/// - *May* require aditional processing to translate the normal `usize` to and from `<T>`
/// - `<T>` must implement `TypedBitElem` (simple integer implementations are provided via crate features)
pub struct TypedBitVec<T: TypedBitElem>(pub(crate) RawBitVec, PhantomData<T>);

impl<T: TypedBitElem> TypedBitVec<T> {

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len
    }

    #[inline(always)]
    pub fn cap(&self) -> usize {
        unsafe {self.0.cap(T::PROTO)}
    }

    #[inline(always)]
    pub fn free(&self) -> usize {
        unsafe{self.0.free(T::PROTO)}
    }

    #[inline(always)]
    pub fn new() -> Self {
        Self(RawBitVec::new(), PhantomData)
    }

    #[inline(always)]
    pub fn with_capacity(cap: usize) -> Self {
        Self(RawBitVec::with_capacity(T::PROTO, cap), PhantomData)
    }

    #[inline(always)]
    pub fn grow_exact_for_additional_elements_if_needed(&mut self, extra_elements: usize) -> Result<(), String> {
        unsafe {self.0.grow_exact_for_additional_elements_if_needed(T::PROTO, extra_elements)}
    }

    #[inline(always)]
    pub fn grow_exact_for_total_elements_if_needed(&mut self, total_elements: usize) -> Result<(), String> {
        unsafe {self.0.grow_exact_for_total_elements_if_needed(T::PROTO, total_elements)}
    }

    #[inline(always)]
    pub fn grow_for_additional_elements_if_needed(&mut self, extra_elements: usize) -> Result<(), String> {
        unsafe {self.0.grow_for_additional_elements_if_needed(T::PROTO, extra_elements)}
    }

    #[inline(always)]
    pub fn grow_for_total_elements_if_needed(&mut self, total_elements: usize) -> Result<(), String> {
        unsafe {self.0.grow_for_total_elements_if_needed(T::PROTO, total_elements)}
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.0.clear()
    }

    #[inline(always)]
    pub fn push(&mut self, val: T::Base) -> Result<(), String> {
        unsafe {self.0.push(T::PROTO, T::val_to_bits(val))}
    }

    #[inline(always)]
    pub unsafe fn push_unchecked(&mut self, val: T::Base) {
        self.0.push_unchecked(T::PROTO, T::val_to_bits(val))
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Result<T::Base, String> {
        match unsafe{self.0.pop(T::PROTO)} {
            Ok(bits) => Ok(T::bits_to_val(bits)),
            Err(e) => Err(e),
        }
    }

    #[inline(always)]
    pub unsafe fn pop_unchecked(&mut self) -> T::Base {
        T::bits_to_val(self.0.pop_unchecked(T::PROTO))
    }

    #[inline(always)]
    pub fn insert(&mut self, idx: usize, val: T::Base) -> Result<(), String> {
        unsafe{self.0.insert(T::PROTO, idx, T::val_to_bits(val))}
    }

    #[inline(always)]
    pub unsafe fn insert_unchecked(&mut self, idx: usize, val: T::Base) {
        self.0.insert_unchecked(T::PROTO, idx, T::val_to_bits(val))
    }

    #[inline(always)]
    pub fn insert_bitvec(&mut self, insert_idx: usize, bitvec: Self) -> Result<(), String> {
        unsafe{self.0.insert_bitvec(T::PROTO, insert_idx, bitvec.into_raw())}
    }

    #[inline(always)]
    pub unsafe fn insert_bitvec_unchecked(&mut self, insert_idx: usize, bitvec: Self) {
        self.0.insert_bitvec_unchecked(T::PROTO, insert_idx, bitvec.into_raw())
    }

    #[inline]
    pub fn insert_iter<II, TO, ESI>(&mut self, insert_idx: usize, source: II) -> Result<(), String>
    where II: IntoIterator<Item = TO, IntoIter = ESI>, TO: ToOwned<Owned = usize>, ESI: ExactSizeIterator + Iterator<Item = TO> {
        unsafe {self.0.insert_iter(T::PROTO, insert_idx, source)}
    }

    #[inline]
    pub unsafe fn insert_iter_unchecked<II, TO, ESI>(&mut self, insert_idx: usize, source: II)
    where II: IntoIterator<Item = TO, IntoIter = ESI>, TO: ToOwned<Owned = usize>, ESI: ExactSizeIterator + Iterator<Item = TO> {
        self.0.insert_iter_unchecked(T::PROTO, insert_idx, source)
    }

    #[inline(always)]
    pub fn remove(&mut self, idx: usize) -> Result<T::Base, String> {
        match unsafe{self.0.remove(T::PROTO, idx)} {
            Ok(bits) => Ok(T::bits_to_val(bits)),
            Err(e) => Err(e),
        }
    }

    #[inline(always)]
    pub unsafe fn remove_unchecked(&mut self, idx: usize) -> T::Base {
        T::bits_to_val(self.0.remove_unchecked(T::PROTO, idx))
    }

    #[inline(always)]
    pub fn remove_range(&mut self, idx_range: Range<usize>) -> Result<Self, String> {
        match unsafe{self.0.remove_range(T::PROTO, idx_range)} {
            Ok(raw) => Ok(Self(raw, PhantomData)),
            Err(e) => Err(e),
        }
    }

    #[inline(always)]
    pub unsafe fn remove_range_unchecked(&mut self, idx_range: Range<usize>) -> Self {
        Self(self.0.remove_range_unchecked(T::PROTO, idx_range), PhantomData)
    }

    #[inline(always)]
    pub fn swap(&mut self, idx_a: usize, idx_b: usize) -> Result<(), String> {
        unsafe{self.0.swap(T::PROTO, idx_a, idx_b)}
    }

    #[inline(always)]
    pub unsafe fn swap_unchecked(&mut self, idx_a: usize, idx_b: usize) {
        self.0.swap_unchecked(T::PROTO, idx_a, idx_b)
}

    #[inline(always)]
    pub fn swap_pop(&mut self, idx: usize) -> Result<T::Base, String> {
        match unsafe{self.0.swap_pop(T::PROTO, idx)} {
            Ok(bits) => Ok(T::bits_to_val(bits)),
            Err(e) => Err(e),
        }
    }

    #[inline(always)]
    pub unsafe fn swap_pop_unchecked(&mut self, idx: usize) -> T::Base {
        T::bits_to_val(self.0.swap_pop_unchecked(T::PROTO, idx))
    }

    #[inline(always)]
    pub fn trim_excess_capacity(&mut self, extra_capacity_to_keep: usize) -> Result<(), String> {
        unsafe{self.0.shrink_excess_capacity(T::PROTO, extra_capacity_to_keep)}
    }

    #[inline(always)]
    pub fn append_bitvec(&mut self, bitvec: Self) -> Result<(), String> {
        unsafe{self.0.append_bitvec(T::PROTO, bitvec.into_raw())}
    }

    #[inline(always)]
    pub unsafe fn append_bitvec_unchecked(&mut self, bitvec: Self) {
        self.0.append_bitvec_unchecked(T::PROTO, bitvec.into_raw())
    }

    #[inline(always)]
    pub fn append_iter<II, TO, ESI>(&mut self, source: II) -> Result<(), String>
    where II: IntoIterator<Item = TO, IntoIter = ESI>, TO: ToOwned<Owned = T::Base>, ESI: ExactSizeIterator + Iterator<Item = TO> {
        unsafe{self.0.append_iter(T::PROTO, source.into_iter().map(|val| T::val_to_bits(val.to_owned())))}
    }

    #[inline(always)]
    pub unsafe fn append_iter_unchecked<I, TO>(&mut self, iter: I)
    where I: Iterator<Item = TO> + ExactSizeIterator, TO: ToOwned<Owned = T::Base> {
        self.0.append_iter_unchecked(T::PROTO, iter.map(|val| T::val_to_bits(val.to_owned())))
    }

    #[inline(always)]
    pub fn get(&self, idx: usize) -> Result<T::Base, String> {
        match unsafe{self.0.get(T::PROTO, idx)} {
            Ok(bits) => Ok(T::bits_to_val(bits)),
            Err(e) => Err(e),
        }
    }

    #[inline(always)]
    pub unsafe fn get_unchecked(&self, idx: usize) -> T::Base {
        T::bits_to_val(self.0.get_unchecked(T::PROTO, idx))
    }

    #[inline(always)]
    pub fn replace(&mut self, idx: usize, val: T::Base) -> Result<T::Base, String> {
        match unsafe{self.0.replace(T::PROTO, idx, T::val_to_bits(val))} {
            Ok(bits) => Ok(T::bits_to_val(bits)),
            Err(e) => Err(e),
        }
    }

    #[inline(always)]
    pub unsafe fn replace_unchecked(&mut self, idx: usize, val: T::Base) -> T::Base {
        T::bits_to_val(self.0.replace_unchecked(T::PROTO, idx, T::val_to_bits(val)))
    }

    #[inline(always)]
    pub fn set(&mut self, idx: usize, val: T::Base) -> Result<(), String> {
        unsafe{self.0.set(T::PROTO, idx, T::val_to_bits(val))}
    }

    #[inline(always)]
    pub unsafe fn set_unchecked(&mut self, idx: usize, val: T::Base) {
        self.0.set_unchecked(T::PROTO, idx, T::val_to_bits(val))
    }

    #[inline(always)]
    pub fn drain<'vec>(&'vec mut self) -> TypedBitVecDrain<'vec, T> {
        TypedBitVecDrain(self.0.drain(), PhantomData)
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

impl<T: TypedBitElem> IntoIterator for TypedBitVec<T> {
    type Item = T::Base;

    type IntoIter = TypedBitVecIter<T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        TypedBitVecIter(unsafe{self.into_raw().into_iter()}, PhantomData)
    }
}

impl<T: TypedBitElem> Drop for TypedBitVec<T> {
    #[inline(always)]
    fn drop(&mut self) {/* RawBitVec::drop() will take care of the allocation */}
}