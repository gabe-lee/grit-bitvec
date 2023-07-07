use crate::{
    size_of,
    align_of,
    needs_drop,
    ptr,
    NonNull,
    PhantomData,
    alloc,
    Layout,
    BitVec,
    BitElem,
};



pub struct BVecIter<ELEM>
where ELEM: BitElem {
    pub(crate) ptr: NonNull<usize>,
    pub(crate) real_cap: usize,
    pub(crate) start: usize,
    pub(crate) count: usize,
    pub(crate) sub: PhantomData<ELEM>
}

impl<ELEM> Iterator for BVecIter<ELEM>
where ELEM: BitElem {
    type Item = ELEM::Base;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            if size_of::<ELEM::Base>() == 0 || ELEM::BITS == 0 {
                Some(ELEM::bits_to_val(0))
            } else {
                let (real_idx, bit_off) = BitVec::<ELEM>::calc_sub_idx_to_real_idx_and_bit_offset(self.start);
                let block_bits =  unsafe{ptr::read(self.ptr.as_ptr().add(real_idx))};
                let val_bits = (block_bits & (ELEM::MASK << bit_off)) >> bit_off;
                self.start += 1;
                self.count -= 1;
                Some(ELEM::bits_to_val(val_bits))
            }
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}

impl<ELEM> DoubleEndedIterator for BVecIter<ELEM>
where ELEM: BitElem {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            if size_of::<ELEM::Base>() == 0 || ELEM::BITS == 0 {
                Some(ELEM::bits_to_val(0))
            } else {
                let (real_idx, bit_off) = BitVec::<ELEM>::calc_sub_idx_to_real_idx_and_bit_offset(self.start+self.count);
                let block_bits =  unsafe{ptr::read(self.ptr.as_ptr().add(real_idx))};
                let val_bits = (block_bits & (ELEM::MASK << bit_off)) >> bit_off;
                self.count -= 1;
                Some(ELEM::bits_to_val(val_bits))
            }
        }
    }
}

impl<ELEM> Drop for BVecIter<ELEM>
where ELEM: BitElem {
    fn drop(&mut self) {
        if needs_drop::<ELEM::Base>() {
            while let Some(_) = self.next() {}
        }
        unsafe {alloc::dealloc(self.ptr.as_ptr().cast(), Layout::from_size_align_unchecked(self.real_cap*size_of::<usize>(), align_of::<usize>()))};
    }
}

pub struct BVecDrain<'vec, ELEM>
where ELEM: BitElem  {
    pub(crate) vec: PhantomData<&'vec ELEM>,
    pub(crate) ptr: NonNull<usize>,
    pub(crate) start: usize,
    pub(crate) count: usize,
}

impl<'vec, ELEM> Iterator for BVecDrain<'vec, ELEM>
where ELEM: BitElem {
    type Item = ELEM::Base;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            if size_of::<ELEM::Base>() == 0 || ELEM::BITS == 0 {
                Some(ELEM::bits_to_val(0))
            } else {
                let (real_idx, bit_off) = BitVec::<ELEM>::calc_sub_idx_to_real_idx_and_bit_offset(self.start);
                let block_bits =  unsafe{ptr::read(self.ptr.as_ptr().add(real_idx))};
                let val_bits = (block_bits & (ELEM::MASK << bit_off)) >> bit_off;
                self.start += 1;
                self.count -= 1;
                Some(ELEM::bits_to_val(val_bits))
            }
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}

impl<'vec, ELEM> DoubleEndedIterator for BVecDrain<'vec, ELEM>
where ELEM: BitElem  {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            if size_of::<ELEM::Base>() == 0 || ELEM::BITS == 0 {
                Some(ELEM::bits_to_val(0))
            } else {
                let (real_idx, bit_off) = BitVec::<ELEM>::calc_sub_idx_to_real_idx_and_bit_offset(self.start+self.count);
                let block_bits =  unsafe{ptr::read(self.ptr.as_ptr().add(real_idx))};
                let val_bits = (block_bits & (ELEM::MASK << bit_off)) >> bit_off;
                self.count -= 1;
                Some(ELEM::bits_to_val(val_bits))
            }
        }
    }
}

impl<'vec, ELEM> Drop for BVecDrain<'vec, ELEM>
where ELEM: BitElem {
    fn drop(&mut self) {
        if needs_drop::<ELEM::Base>() {
            while let Some(_) = self.next() {}
        }
    }
}