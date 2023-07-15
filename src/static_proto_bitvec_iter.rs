use crate::{
    BitProto,
    RawBitVecIter, 
    RawBitVecDrain,
};

pub struct SProtoBitVecIter {
    pub(crate) proto: &'static BitProto,
    pub(crate) iter: RawBitVecIter
}

impl Iterator for SProtoBitVecIter {
    type Item = usize;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {self.iter.next(*self.proto)}
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.iter.len();
        (len, Some(len))
    }
}

impl DoubleEndedIterator for SProtoBitVecIter {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {self.iter.next_back(*self.proto)}
    }
}

impl ExactSizeIterator for SProtoBitVecIter {
    #[inline(always)]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl Drop for SProtoBitVecIter  {
    #[inline(always)]
    fn drop(&mut self) {/* RawBitVecIter will handle the deallocation */}
}

pub struct SProtoBitVecDrain<'vec>{
    pub(crate) proto: &'static BitProto,
    pub(crate) drain: RawBitVecDrain<'vec>
}

impl<'vec> Iterator for SProtoBitVecDrain<'vec> {
    type Item = usize;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {self.drain.next(*self.proto)}
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.drain.len();
        (len, Some(len))
    }
}

impl<'vec> DoubleEndedIterator for SProtoBitVecDrain<'vec> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {self.drain.next_back(*self.proto)}
    }
}

impl<'vec> ExactSizeIterator for SProtoBitVecDrain<'vec> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.drain.len()
    }
}

impl<'vec> Drop for SProtoBitVecDrain<'vec>  {
    #[inline(always)]
    fn drop(&mut self) {/* RawBitVecIter will handle the deallocation */}
}