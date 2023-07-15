use crate::{
    RawBitVecIter,
    RawBitVecDrain,
    TypedBitElem,
    PhantomData
};

pub struct TypedBitVecIter<TYPE: TypedBitElem>(pub(crate) RawBitVecIter, pub(crate) PhantomData<TYPE>);

impl<TYPE: TypedBitElem> Iterator for TypedBitVecIter<TYPE> {
    type Item = TYPE::Base;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match unsafe {self.0.next(TYPE::PROTO)} {
            Some(bits) => Some(TYPE::bits_to_val(bits)),
            None => None,
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len();
        (len, Some(len))
    }
}

impl<TYPE: TypedBitElem> DoubleEndedIterator for TypedBitVecIter<TYPE> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        match unsafe {self.0.next_back(TYPE::PROTO)} {
            Some(bits) => Some(TYPE::bits_to_val(bits)),
            None => None,
        }
    }
}

impl<TYPE: TypedBitElem> ExactSizeIterator for TypedBitVecIter<TYPE> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<TYPE: TypedBitElem> Drop for TypedBitVecIter<TYPE>  {
    #[inline(always)]
    fn drop(&mut self) {/* RawBitVecIter will handle the deallocation */}
}

pub struct TypedBitVecDrain<'vec, TYPE: TypedBitElem>(pub(crate) RawBitVecDrain<'vec>, pub(crate) PhantomData<TYPE>);

impl<'vec, TYPE: TypedBitElem> Iterator for TypedBitVecDrain<'vec, TYPE> {
    type Item = TYPE::Base;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match unsafe {self.0.next(TYPE::PROTO)} {
            Some(bits) => Some(TYPE::bits_to_val(bits)),
            None => None,
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len();
        (len, Some(len))
    }
}

impl<'vec, TYPE: TypedBitElem> DoubleEndedIterator for TypedBitVecDrain<'vec, TYPE> {

    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        match unsafe {self.0.next_back(TYPE::PROTO)} {
            Some(bits) => Some(TYPE::bits_to_val(bits)),
            None => None,
        }
    }
}

impl<'vec, TYPE: TypedBitElem> ExactSizeIterator for TypedBitVecDrain<'vec, TYPE> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'vec, TYPE: TypedBitElem> Drop for TypedBitVecDrain<'vec, TYPE>  {
    #[inline(always)]
    fn drop(&mut self) {/* RawBitVecIter will handle the deallocation */}
}