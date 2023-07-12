
use crate::{
    size_of,
    align_of,
    ptr,
    NonNull,
    alloc,
    Layout,
    // BitVec,
    IdxProxy, RawBitVec,
};

pub struct RawBitVecIter<const BIT_WIDTH: usize> {
    pub(crate) ptr: NonNull<usize>,
    pub(crate) real_cap: usize,
    pub(crate) start: usize,
    pub(crate) end_excluded: usize,
}

impl<const BIT_WIDTH: usize> Iterator for RawBitVecIter<BIT_WIDTH> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_excluded == self.start {
            None
        } else {
            let val = if BIT_WIDTH == 0 {
                Some(0)
            } else {
                unsafe {
                    let idx_proxy = IdxProxy::<BIT_WIDTH>::from(self.start);
                    let mut block_ptr = self.ptr.as_ptr().add(idx_proxy.real_idx);
                    let mut block_bits = ptr::read(block_ptr);
                    let mut val = (block_bits & idx_proxy.first_mask) >> idx_proxy.first_offset;
                    if idx_proxy.second_mask != 0 {
                        block_ptr = block_ptr.add(1);
                        block_bits = ptr::read(block_ptr);
                        val = val | ((block_bits & idx_proxy.second_mask) << idx_proxy.second_offset);
                    }
                    Some(val)
                }
            };
            self.start += 1;
            val
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<const BIT_WIDTH: usize> DoubleEndedIterator for RawBitVecIter<BIT_WIDTH> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end_excluded == self.start {
            None
        } else {
            let val = if BIT_WIDTH == 0 {
                Some(0)
            } else {
                unsafe {
                    let idx_proxy = IdxProxy::<BIT_WIDTH>::from(self.end_excluded - 1);
                    let mut block_ptr = self.ptr.as_ptr().add(idx_proxy.real_idx);
                    let mut block_bits = ptr::read(block_ptr);
                    let mut val = (block_bits & idx_proxy.first_mask) >> idx_proxy.first_offset;
                    if idx_proxy.second_mask != 0 {
                        block_ptr = block_ptr.add(1);
                        block_bits = ptr::read(block_ptr);
                        val = val | ((block_bits & idx_proxy.second_mask) << idx_proxy.second_offset);
                    }
                    Some(val)
                }
            };
            self.end_excluded -= 1;
            val
        }
    }
}

impl<const BIT_WIDTH: usize> ExactSizeIterator for RawBitVecIter<BIT_WIDTH> {
    fn len(&self) -> usize {
        self.end_excluded - self.start
    }
}

impl<const BIT_WIDTH: usize> Drop for RawBitVecIter<BIT_WIDTH>  {
    fn drop(&mut self) {
        unsafe {alloc::dealloc(self.ptr.as_ptr().cast(), Layout::from_size_align_unchecked(self.real_cap*size_of::<usize>(), align_of::<usize>()))};
    }
}

pub struct RawBitVecDrain<'vec, const BIT_WIDTH: usize> {
    pub(crate) vec: &'vec mut RawBitVec<BIT_WIDTH>,
    pub(crate) start: usize,
    pub(crate) end_excluded: usize,
    pub(crate) begin_idx: usize,
    pub(crate) shift_idx: usize
}

impl<'vec, const BIT_WIDTH: usize> Iterator for RawBitVecDrain<'vec, BIT_WIDTH> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_excluded == self.start {
            None
        } else {
            let val = if BIT_WIDTH == 0 {
                Some(0)
            } else {
                unsafe {
                    let idx_proxy = IdxProxy::<BIT_WIDTH>::from(self.start);
                    Some(self.vec.replace_val_with_idx_proxy(idx_proxy, 0))
                }
            };
            self.start += 1;
            val
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'vec, const BIT_WIDTH: usize> DoubleEndedIterator for RawBitVecDrain<'vec, BIT_WIDTH> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end_excluded == self.start {
            None
        } else {
            let val = if BIT_WIDTH == 0 {
                Some(0)
            } else {
                unsafe {
                    let idx_proxy = IdxProxy::<BIT_WIDTH>::from(self.end_excluded - 1);
                    Some(self.vec.replace_val_with_idx_proxy(idx_proxy, 0))
                }
            };
            self.end_excluded -= 1;
            val
        }
    }
}

impl<'vec, const BIT_WIDTH: usize> ExactSizeIterator for RawBitVecDrain<'vec, BIT_WIDTH> {
    fn len(&self) -> usize {
        self.end_excluded - self.start
    }
}

impl<'vec, const BIT_WIDTH: usize> Drop for RawBitVecDrain<'vec, BIT_WIDTH>  {
    fn drop(&mut self) {
        if self.shift_idx == self.vec.len {
            self.vec.len = self.begin_idx;
        } else {
            let count = self.shift_idx - self.begin_idx;
            let begin_proxy = IdxProxy::<BIT_WIDTH>::from(self.begin_idx);
            let shift_proxy = IdxProxy::<BIT_WIDTH>::from(self.shift_idx);
            unsafe { self.vec.shift_elements_down_with_with_idx_proxy(begin_proxy, shift_proxy, count)};
        }
    }
}