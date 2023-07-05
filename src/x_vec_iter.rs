use crate::{
    size_of,
    align_of,
    needs_drop,
    ptr,
    NonNull,
    PhantomData,
    alloc,
    Layout,
    x_vec::{
        XVec,
        XVecIndex
    }
};



pub struct XVecIter<ELEM, IDX>
where IDX: XVecIndex {
    pub(crate) ptr: NonNull<ELEM>,
    pub(crate) cap: IDX,
    pub(crate) start: IDX,
    pub(crate) count: IDX,
}

impl<ELEM, IDX> Iterator for XVecIter<ELEM, IDX>
where IDX: XVecIndex {
    type Item = ELEM;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == IDX::ZERO {
            None
        } else {
            let val_ptr = if size_of::<ELEM>() != 0 {
                unsafe{self.ptr.as_ptr().add(self.start.to_usize())}
            } else {
                self.ptr.as_ptr()
            };
            let val = unsafe {ptr::read(val_ptr)};
            if size_of::<ELEM>() != 0 {
                self.start += IDX::ONE;
            }
            self.count -= IDX::ONE;
            Some(val)
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count.to_usize(), Some(self.count.to_usize()))
    }
}

impl<ELEM, IDX> DoubleEndedIterator for XVecIter<ELEM, IDX>
where IDX: XVecIndex {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.count == IDX::ZERO {
            None
        } else {
            let val_ptr = if size_of::<ELEM>() != 0 {
                unsafe{self.ptr.as_ptr().add((self.start + self.count).to_usize())}
            } else {
                self.ptr.as_ptr()
            };
            let val = unsafe {ptr::read(val_ptr)};
            self.count -= IDX::ONE;
            Some(val)
        }
    }
}

impl<ELEM, IDX> Drop for XVecIter<ELEM, IDX>
where IDX: XVecIndex {
    fn drop(&mut self) {
        if size_of::<ELEM>() != 0 && needs_drop::<ELEM>() {
            let mut idx = self.count;
            while idx > IDX::ZERO {
                unsafe{ptr::drop_in_place(self.ptr.as_ptr().add((self.start+idx).to_usize()))};
                idx -= IDX::ONE;
            }
        }
        unsafe {alloc::dealloc(self.ptr.as_ptr().cast(), Layout::from_size_align_unchecked(self.cap.to_usize()*size_of::<ELEM>(), align_of::<ELEM>()))};
    }
}
pub struct XVecDrain<'vec, ELEM, IDX>
where IDX: XVecIndex {
    pub(crate) vec: PhantomData<&'vec mut XVec<ELEM, IDX>>,
    pub(crate) ptr: NonNull<ELEM>,
    pub(crate) start: IDX,
    pub(crate) count: IDX,
}

impl<'vec, ELEM, IDX> Iterator for XVecDrain<'vec, ELEM, IDX>
where IDX: XVecIndex {
    type Item = ELEM;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == IDX::ZERO {
            None
        } else {
            let val_ptr = if size_of::<ELEM>() != 0 {
                unsafe{self.ptr.as_ptr().add(self.start.to_usize())}
            } else {
                self.ptr.as_ptr()
            };
            let val = unsafe {ptr::read(val_ptr)};
            if size_of::<ELEM>() != 0 {
                self.start += IDX::ONE;
            }
            self.count -= IDX::ONE;
            Some(val)
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count.to_usize(), Some(self.count.to_usize()))
    }
}

impl<'vec, ELEM, IDX> DoubleEndedIterator for XVecDrain<'vec, ELEM, IDX>
where IDX: XVecIndex {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.count == IDX::ZERO {
            None
        } else {
            let val_ptr = if size_of::<ELEM>() != 0 {
                unsafe{self.ptr.as_ptr().add((self.start + self.count).to_usize())}
            } else {
                self.ptr.as_ptr()
            };
            let val = unsafe {ptr::read(val_ptr)};
            self.count -= IDX::ONE;
            Some(val)
        }
    }
}

impl<'vec, ELEM, IDX> Drop for XVecDrain<'vec, ELEM, IDX>
where IDX: XVecIndex {
    fn drop(&mut self) {
        if size_of::<ELEM>() != 0 && needs_drop::<ELEM>() {
            let mut idx = self.count;
            while idx > IDX::ZERO {
                unsafe{ptr::drop_in_place(self.ptr.as_ptr().add((self.start+idx).to_usize()))};
                idx -= IDX::ONE;
            }
        }
    }
}