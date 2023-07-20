use crate::{
    ptr,
    NonNull,
    alloc,
    RawBitVec, 
    BitProto, 
    MemUtil,
};

pub struct RawBitVecIter {
    pub(crate) ptr: NonNull<usize>,
    pub(crate) true_cap: usize,
    pub(crate) start: usize,
    pub(crate) end_excluded: usize,
}

impl RawBitVecIter {
    #[inline]
    pub unsafe fn next(&mut self, proto: BitProto) -> Option<usize> {
        match self.start == self.end_excluded {
            true => None,
            false => Some(self.next_unchecked(proto))
        }
    }

    #[inline]
    pub unsafe fn next_unchecked(&mut self, proto: BitProto) -> usize {
        let idx_proxy = BitProto::idx_proxy(proto, self.start);
        let mut block_ptr = self.ptr.as_ptr().add(idx_proxy.real_idx);
        let mut block_bits = ptr::read(block_ptr);
        let mut val = (block_bits & idx_proxy.first_mask) >> idx_proxy.first_offset;
        if idx_proxy.second_mask != 0 {
            block_ptr = block_ptr.add(1);
            block_bits = ptr::read(block_ptr);
            val = val | ((block_bits & idx_proxy.second_mask) << idx_proxy.second_offset);
        }
        self.start += 1;
        val
    }

    #[inline]
    pub unsafe fn next_back(&mut self, proto: BitProto) -> Option<usize> {
        match self.start == self.end_excluded {
            true => None,
            false => Some(self.next_back_unchecked(proto))
        }
    }

    pub unsafe fn next_back_unchecked(&mut self, proto: BitProto) -> usize {
        self.end_excluded -= 1;
        let idx_proxy = BitProto::idx_proxy(proto, self.end_excluded);
        let mut block_ptr = self.ptr.as_ptr().add(idx_proxy.real_idx);
        let mut block_bits = ptr::read(block_ptr);
        let mut val = (block_bits & idx_proxy.first_mask) >> idx_proxy.first_offset;
        if idx_proxy.second_mask != 0 {
            block_ptr = block_ptr.add(1);
            block_bits = ptr::read(block_ptr);
            val = val | ((block_bits & idx_proxy.second_mask) << idx_proxy.second_offset);
        }
        val
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.end_excluded - self.start
    }
}

impl Drop for RawBitVecIter  {
    #[inline(always)]
    fn drop(&mut self) {
        if self.true_cap > 0 {
            unsafe {alloc::dealloc(self.ptr.as_ptr().cast(), MemUtil::usize_array_layout(self.true_cap))};
        }
    }
}

pub struct RawBitVecDrain<'vec> {
    pub(crate) vec: &'vec mut RawBitVec,
    pub(crate) start: usize,
    pub(crate) end_excluded: usize,
}

impl<'vec> RawBitVecDrain<'vec> {
    #[inline]
    pub unsafe fn next(&mut self, proto: BitProto) -> Option<usize> {
        match self.start == self.end_excluded {
            true => None,
            false => Some(self.next_unchecked(proto))
        }
    }

    #[inline]
    pub unsafe fn next_unchecked(&mut self, proto: BitProto) -> usize {
        let idx_proxy = BitProto::idx_proxy(proto, self.start);
        let val = self.vec.replace_val_with_idx_proxy(idx_proxy, 0);
        self.start += 1;
        val
    }

    #[inline]
    pub unsafe fn next_back(&mut self, proto: BitProto) -> Option<usize> {
        match self.start == self.end_excluded {
            true => None,
            false => Some(self.next_back_unchecked(proto))
        }
    }

    #[inline]
    pub unsafe fn next_back_unchecked(&mut self, proto: BitProto) -> usize {
        self.end_excluded -= 1;
        let idx_proxy = BitProto::idx_proxy(proto, self.end_excluded);
        let val = self.vec.replace_val_with_idx_proxy(idx_proxy, 0);
        val
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.end_excluded - self.start
    }
}

impl<'vec> Drop for RawBitVecDrain<'vec>  {
    #[inline(always)]
    fn drop(&mut self) {
        self.vec.len = 0;
    }
}
