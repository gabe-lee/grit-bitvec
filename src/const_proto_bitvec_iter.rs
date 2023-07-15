use crate::{
    BitProto,
    RawBitVecIter, 
    RawBitVecDrain,
};

pub struct CProtoBitVecIter<const BIT_WIDTH: usize>(pub(crate) RawBitVecIter);

impl<const BIT_WIDTH: usize> CProtoBitVecIter<BIT_WIDTH> {
    pub(crate) const PROTO: BitProto = BitProto::create(BIT_WIDTH);
}

impl<const BIT_WIDTH: usize> Iterator for CProtoBitVecIter<BIT_WIDTH> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {self.0.next(Self::PROTO)}
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len();
        (len, Some(len))
    }
}

impl<const BIT_WIDTH: usize> DoubleEndedIterator for CProtoBitVecIter<BIT_WIDTH> {

    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {self.0.next_back(Self::PROTO)}
    }
}

impl<const BIT_WIDTH: usize> ExactSizeIterator for CProtoBitVecIter<BIT_WIDTH> {

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<const BIT_WIDTH: usize> Drop for CProtoBitVecIter<BIT_WIDTH>  {
    fn drop(&mut self) {/* RawBitVecIter will handle the deallocation */}
}

pub struct CProtoBitVecDrain<'vec, const BIT_WIDTH: usize>(pub(crate) RawBitVecDrain<'vec>);

impl <'vec, const BIT_WIDTH: usize> CProtoBitVecDrain<'vec, BIT_WIDTH> {
    pub(crate) const PROTO: BitProto = BitProto::create(BIT_WIDTH);
}

impl<'vec, const BIT_WIDTH: usize> Iterator for CProtoBitVecDrain<'vec, BIT_WIDTH> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {self.0.next(Self::PROTO)}
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len();
        (len, Some(len))
    }
}

impl<'vec, const BIT_WIDTH: usize> DoubleEndedIterator for CProtoBitVecDrain<'vec, BIT_WIDTH> {

    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {self.0.next_back(Self::PROTO)}
    }
}

impl<'vec, const BIT_WIDTH: usize> ExactSizeIterator for CProtoBitVecDrain<'vec, BIT_WIDTH> {

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'vec, const BIT_WIDTH: usize> Drop for CProtoBitVecDrain<'vec, BIT_WIDTH>  {
    fn drop(&mut self) {/* RawBitVecIter will handle the deallocation */}
}