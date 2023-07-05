use crate::{
    size_of,
    align_of,
    needs_drop,
    ptr,
    NonNull,
    PhantomData,
    alloc,
    Layout,
    x_vec::XVecIndex, 
    bx_vec::{
        BXVecElem,
        BXVec
    }
};



pub struct BXVecIter<ELEM, IDX>
where IDX: XVecIndex, ELEM: BXVecElem {
    pub(crate) ptr: NonNull<usize>,
    pub(crate) real_cap: IDX,
    pub(crate) start: IDX,
    pub(crate) count: IDX,
    pub(crate) sub: PhantomData<ELEM>
}

impl<ELEM, IDX> Iterator for BXVecIter<ELEM, IDX>
where IDX: XVecIndex, ELEM: BXVecElem {
    type Item = ELEM::Base;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == IDX::ZERO {
            None
        } else {
            if size_of::<ELEM::Base>() == 0 || ELEM::BITS == 0 {
                Some(ELEM::bits_to_val(0))
            } else {
                let (real_idx, bit_off) = BXVec::<ELEM, IDX>::calc_sub_idx_to_real_idx_and_bit_offset(self.start);
                let block_bits =  unsafe{ptr::read(self.ptr.as_ptr().add(real_idx))};
                let val_bits = (block_bits & (ELEM::MASK << bit_off)) >> bit_off;
                self.start += IDX::ONE;
                self.count -= IDX::ONE;
                Some(ELEM::bits_to_val(val_bits))
            }
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count.to_usize(), Some(self.count.to_usize()))
    }
}

impl<ELEM, IDX> DoubleEndedIterator for BXVecIter<ELEM, IDX>
where IDX: XVecIndex, ELEM: BXVecElem {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.count == IDX::ZERO {
            None
        } else {
            if size_of::<ELEM::Base>() == 0 || ELEM::BITS == 0 {
                Some(ELEM::bits_to_val(0))
            } else {
                let (real_idx, bit_off) = BXVec::<ELEM, IDX>::calc_sub_idx_to_real_idx_and_bit_offset(self.start+self.count);
                let block_bits =  unsafe{ptr::read(self.ptr.as_ptr().add(real_idx))};
                let val_bits = (block_bits & (ELEM::MASK << bit_off)) >> bit_off;
                self.count -= IDX::ONE;
                Some(ELEM::bits_to_val(val_bits))
            }
        }
    }
}

impl<ELEM, IDX> Drop for BXVecIter<ELEM, IDX>
where IDX: XVecIndex, ELEM: BXVecElem {
    fn drop(&mut self) {
        if size_of::<ELEM::Base>() != 0 && ELEM::BITS != 0 && needs_drop::<ELEM::Base>() {
            while let Some(_) = self.next() {}
        }
        unsafe {alloc::dealloc(self.ptr.as_ptr().cast(), Layout::from_size_align_unchecked(self.real_cap.to_usize()*size_of::<usize>(), align_of::<usize>()))};
    }
}

pub struct BXVecDrain<'vec, ELEM, IDX>
where IDX: XVecIndex, ELEM: BXVecElem  {
    pub(crate) vec: PhantomData<&'vec mut BXVec<ELEM, IDX>>,
    pub(crate) ptr: NonNull<usize>,
    pub(crate) start: IDX,
    pub(crate) count: IDX,
}

impl<'vec, ELEM, IDX> Iterator for BXVecDrain<'vec, ELEM, IDX>
where IDX: XVecIndex, ELEM: BXVecElem {
    type Item = ELEM::Base;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == IDX::ZERO {
            None
        } else {
            if size_of::<ELEM::Base>() == 0 || ELEM::BITS == 0 {
                Some(ELEM::bits_to_val(0))
            } else {
                let (real_idx, bit_off) = BXVec::<ELEM, IDX>::calc_sub_idx_to_real_idx_and_bit_offset(self.start);
                let block_bits =  unsafe{ptr::read(self.ptr.as_ptr().add(real_idx))};
                let val_bits = (block_bits & (ELEM::MASK << bit_off)) >> bit_off;
                self.start += IDX::ONE;
                self.count -= IDX::ONE;
                Some(ELEM::bits_to_val(val_bits))
            }
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count.to_usize(), Some(self.count.to_usize()))
    }
}

impl<'vec, ELEM, IDX> DoubleEndedIterator for BXVecDrain<'vec, ELEM, IDX>
where IDX: XVecIndex, ELEM: BXVecElem  {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.count == IDX::ZERO {
            None
        } else {
            if size_of::<ELEM::Base>() == 0 || ELEM::BITS == 0 {
                Some(ELEM::bits_to_val(0))
            } else {
                let (real_idx, bit_off) = BXVec::<ELEM, IDX>::calc_sub_idx_to_real_idx_and_bit_offset(self.start+self.count);
                let block_bits =  unsafe{ptr::read(self.ptr.as_ptr().add(real_idx))};
                let val_bits = (block_bits & (ELEM::MASK << bit_off)) >> bit_off;
                self.count -= IDX::ONE;
                Some(ELEM::bits_to_val(val_bits))
            }
        }
    }
}

impl<'vec, ELEM, IDX> Drop for BXVecDrain<'vec, ELEM, IDX>
where IDX: XVecIndex, ELEM: BXVecElem {
    fn drop(&mut self) {
        if size_of::<ELEM::Base>() != 0 && ELEM::BITS != 0 && needs_drop::<ELEM::Base>() {
            while let Some(_) = self.next() {}
        }
    }
}