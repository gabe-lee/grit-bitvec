use crate::{
    BitProto,
    RawBitVecIter, 
    RawBitVecDrain,
};

pub struct LProtoBitVecIter {
    pub(crate) proto: BitProto,
    pub(crate) iter: RawBitVecIter
}

impl Iterator for LProtoBitVecIter {
    type Item = usize;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {self.iter.next(self.proto)}
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.iter.len();
        (len, Some(len))
    }
}

impl DoubleEndedIterator for LProtoBitVecIter {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {self.iter.next_back(self.proto)}
    }
}

impl ExactSizeIterator for LProtoBitVecIter {
    #[inline(always)]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl Drop for LProtoBitVecIter  {
    #[inline(always)]
    fn drop(&mut self) {/* RawBitVecIter will handle the deallocation */}
}

pub struct LProtoBitVecDrain<'vec>{
    pub(crate) proto: BitProto,
    pub(crate) drain: RawBitVecDrain<'vec>
}

impl<'vec> Iterator for LProtoBitVecDrain<'vec> {
    type Item = usize;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {self.drain.next(self.proto)}
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.drain.len();
        (len, Some(len))
    }
}

impl<'vec> DoubleEndedIterator for LProtoBitVecDrain<'vec> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {self.drain.next_back(self.proto)}
    }
}

impl<'vec> ExactSizeIterator for LProtoBitVecDrain<'vec> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.drain.len()
    }
}

impl<'vec> Drop for LProtoBitVecDrain<'vec>  {
    #[inline(always)]
    fn drop(&mut self) {/* RawBitVecIter will handle the deallocation */}
}