
use crate::{
    mem,
    size_of,
    align_of,
    needs_drop,
    ptr,
    NonNull,
    slice,
    Display,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    Shl,
    Shr,
    Add,
    Sub,
    Mul,
    Div,
    Deref,
    DerefMut,
    Index,
    Range,
    IndexMut,
    RangeFrom,
    RangeFull,
    RangeTo,
    RangeInclusive,
    RangeToInclusive,
    PhantomData,
    alloc,
    Layout,
    x_vec_iter::{XVecIter, XVecDrain},
    utils::MemUtil
};

#[derive(Clone, Copy)]
pub enum Grow<IDX>
where IDX: XVecIndex {
    Exact(IDX),
    Add(IDX),
    OnePointFive,
    Double,
}

impl<IDX> Default for Grow<IDX>
where IDX: XVecIndex {
    fn default() -> Self {
        Self::OnePointFive
    }
}

#[derive(Clone, Copy)]
pub enum Shrink<IDX>
where IDX: XVecIndex {
    Exact(IDX),
    Minimum,
    SubtractOrMinimum(IDX),
    SubtractTruncate(IDX),
    ThreeQuartersOrMinimum,
    ThreeQuartersTruncate,
    HalfOrMinimum,
    HalfTruncate
}

impl<IDX> Default for Shrink<IDX>
where IDX: XVecIndex {
    fn default() -> Self {
        Self::ThreeQuartersOrMinimum
    }
}

pub enum ElementCount<IDX>
where IDX: XVecIndex {
    Total(IDX),
    Change(IDX),
}

pub type XVec32<T> = XVec<T, u32>;
pub type XVec16<T> = XVec<T, u16>;
pub type XVec8<T> = XVec<T, u8>;
#[cfg(target_pointer_width="64")]
pub type XVecHsize<T> = XVec<T, u32>;
#[cfg(target_pointer_width="32")]
pub type XVecHsize<T> = XVec<T, u16>;
#[cfg(target_pointer_width="16")]
pub type XVecHsize<T> = XVec<T, u8>;

pub unsafe trait XVecIndex: Eq + Ord + Copy + Display + Add<Self, Output = Self> + AddAssign<Self> + Sub<Self, Output = Self> + SubAssign<Self> + Mul<Self, Output = Self> + MulAssign<Self> + Div<Self, Output = Self> + DivAssign<Self> + Shl<Self, Output = Self> + Shr<Self, Output = Self> {
    const IDX_MAX: Self;
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    fn from_usize(val: usize) -> Self;
    fn to_usize(self) -> usize;
}

macro_rules! impl_xvec_unsigned {
    ($TYPE:ty) => {
        unsafe impl XVecIndex for $TYPE {
            const IDX_MAX: Self = isize::MAX as Self;
            const ZERO: Self = Self::MIN;
            const ONE: Self = 1;
            const TWO: Self = 2;
            #[inline(always)]
            fn from_usize(val: usize) -> Self {
                val as Self
            }
            #[inline(always)]
            fn to_usize(self) -> usize {
                self as usize
            }
        }
    };
}
impl_xvec_unsigned!(u8);
impl_xvec_unsigned!(u16);
impl_xvec_unsigned!(u32);
impl_xvec_unsigned!(u64);
impl_xvec_unsigned!(usize);

pub struct XVec<TYPE, IDX>
where IDX: XVecIndex {
    pub(crate) ptr: NonNull<TYPE>,
    pub(crate) cap: IDX,
    pub(crate) len: IDX,
}

impl<ELEM, IDX> XVec<ELEM, IDX>
where IDX: XVecIndex {
    #[inline(always)]
    pub fn len(&self) -> IDX {
        self.len
    }

    #[inline(always)]
    pub fn cap(&self) -> IDX {
        self.cap
    }

    #[inline(always)]
    pub fn free(&self) -> IDX {
        self.cap - self.len
    }

    #[inline(always)]
    pub fn new() -> Self {
        if size_of::<ELEM>() == 0 {
            Self {
                ptr: NonNull::dangling(),
                cap: IDX::IDX_MAX,
                len: IDX::ZERO
            }
        } else {
            Self {
                ptr: NonNull::dangling(),
                cap: IDX::ZERO,
                len: IDX::ZERO
            }
        }
    }

    #[inline(always)]
    pub fn with_capacity(cap: IDX) -> Self {
        let mut new_vec = Self::new();
        new_vec.resize(cap);
        new_vec
    }

    #[inline]
    pub fn grow_if_needed(&mut self, needed: ElementCount<IDX>, mode: Grow<IDX>) {
        if size_of::<ELEM>() != 0 {
            let target_cap = match needed {
                ElementCount::Total(cap) => cap,
                ElementCount::Change(count) => self.len + count,
            };
            if target_cap > self.cap {
                self.grow(mode);
            }
        }
    }

    #[inline]
    pub fn grow(&mut self, mode: Grow<IDX>) {
        if size_of::<ELEM>() != 0 {
            let new_cap = match mode {
                Grow::Exact(val) => {
                    if val < self.cap {
                        return self.shrink(Shrink::Exact(val));
                    } else if val > self.cap {
                        val
                    } else {
                        return;
                    }
                },
                Grow::Add(count) => self.cap + count,
                Grow::OnePointFive => self.cap + (self.cap >> IDX::ONE).min(IDX::ONE),
                Grow::Double => (self.cap << IDX::ONE).min(IDX::ONE),
            };
            self.resize(new_cap)
        }
    }

    #[inline]
    pub fn push(&mut self, val: ELEM, grow: Grow<IDX>) {
        if self.len == IDX::IDX_MAX {
            panic!("XVec is completely full");
        }
        if size_of::<ELEM>() != 0 {
            self.grow_if_needed(ElementCount::Change(IDX::ONE), grow);
        }
        unsafe {self.push_unchecked(val)};
    }

    #[inline]
    pub unsafe fn push_unchecked(&mut self, val: ELEM) {
        if size_of::<ELEM>() != 0 {
            unsafe { ptr::write(self.ptr.as_ptr().add(self.len.to_usize()), val) };
        }
        self.len += IDX::ONE;
    }

    #[inline]
    pub fn clear(&mut self) {
        if size_of::<ELEM>() != 0 && needs_drop::<ELEM>() {
            let mut idx = IDX::ZERO;
            while idx < self.len {
                unsafe { ptr::drop_in_place(self.ptr.as_ptr().add(idx.to_usize()))};
                idx += IDX::ONE;
            }
        }
        self.len = IDX::ZERO
    }

    #[inline]
    fn resize(&mut self, target_capacity: IDX) {
        if size_of::<ELEM>() != 0 {
            let old_u8_ptr = match self.cap != IDX::ZERO {
                true => Some(self.ptr.cast::<u8>().as_ptr()),
                false => None,
            };
            let (new_non_null_u8_ptr, new_cap) = MemUtil::resize_memory_region(
                old_u8_ptr,
                size_of::<ELEM>(),
                align_of::<ELEM>(),
                self.cap.to_usize(),
                target_capacity.to_usize(),
            );
            self.ptr = new_non_null_u8_ptr.cast();
            self.cap = IDX::from_usize(new_cap);
        }
    }

    #[inline(always)]
    pub unsafe fn pop_unchecked(&mut self) -> ELEM {
        self.len -= IDX::ONE;
        unsafe { ptr::read(self.ptr.as_ptr().add(self.len.to_usize()))}
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<ELEM> {
        if self.len == IDX::ZERO {
            None
        } else {
            Some(unsafe {self.pop_unchecked()})
        }
    }

    #[inline]
    pub unsafe fn insert_unchecked(&mut self, idx: IDX, val: ELEM) {
        if idx == self.len {
            return self.push_unchecked(val);
        }
        if size_of::<ELEM>() != 0 {
            ptr::copy(
                self.ptr.as_ptr().add(idx.to_usize()),
                self.ptr.as_ptr().add(idx.to_usize() + 1),
                (self.len - idx).to_usize(),
            );
            ptr::write(self.ptr.as_ptr().add(idx.to_usize()), val);
        }
        self.len += IDX::ONE;
    }

    #[inline]
    pub fn insert(&mut self, idx: IDX, val: ELEM, grow: Grow<IDX>) {
        if self.len == IDX::IDX_MAX {
            panic!("XVec at maximum capcity")
        }
        if idx > self.len {
            panic!("insert index out of bounds");
        }
        if size_of::<ELEM>() != 0 {
            self.grow_if_needed(ElementCount::Change(IDX::ONE), grow);
        }
        unsafe {self.insert_unchecked(idx, val)};
    }

    #[inline]
    pub unsafe fn remove_unchecked(&mut self, idx: IDX) -> ELEM {
        self.len -= IDX::ONE;
        let val = ptr::read(self.ptr.as_ptr().add(idx.to_usize()));
        if size_of::<ELEM>() != 0 {
            ptr::copy(
                self.ptr.as_ptr().add(idx.to_usize() + 1),
                self.ptr.as_ptr().add(idx.to_usize()),
                (self.len - idx).to_usize(),
            );
        }
        val
    }

    #[inline(always)]
    pub fn remove(&mut self, idx: IDX) -> Option<ELEM> {
        match idx >= self.len {
            true => None,
            false => Some(unsafe{self.remove_unchecked(idx)}),
        }
    }

    #[inline(always)]
    pub fn shrink(&mut self, mode: Shrink<IDX>) {
        if size_of::<ELEM>() != 0 {
            let new_cap = match mode {
                Shrink::Exact(val) => {
                    if val > self.cap {
                        return self.grow(Grow::Exact(val));
                    } else if val < self.cap {
                        val
                    } else {
                        return;
                    }
                },
                Shrink::Minimum => self.len,
                Shrink::SubtractOrMinimum(count) => self.len.max(self.cap - count),
                Shrink::SubtractTruncate(count) => self.cap - count,
                Shrink::ThreeQuartersOrMinimum => self.len.max((self.cap >> IDX::ONE) + (self.cap >> IDX::TWO)),
                Shrink::ThreeQuartersTruncate => (self.cap >> IDX::ONE) + (self.cap >> IDX::TWO),
                Shrink::HalfOrMinimum => self.len.max(self.cap >> IDX::ONE),
                Shrink::HalfTruncate => self.cap >> IDX::ONE,
            };
            self.set_exact_capacity(new_cap);
        }
    }

    #[inline(always)]
    pub fn set_exact_capacity(&mut self, new_cap: IDX) {
        if size_of::<ELEM>() != 0 {
            if self.len > new_cap {
                let trim_count = self.len - new_cap;
                let trim_start = self.len - trim_count;
                self.len = new_cap;
                MemUtil::drop_range_in_place(unsafe {self.ptr.as_ptr().add(trim_start.to_usize())}, trim_count.to_usize())
            }
            self.resize(new_cap)
        }
    }

    pub fn combine_append(&mut self, mut other: Self, mode: Grow<IDX>) {
        if size_of::<ELEM>() != 0 {
            self.grow_if_needed(ElementCount::Change(other.len), mode);
            unsafe {ptr::copy_nonoverlapping(other.ptr.as_ptr(), self.ptr.as_ptr().add(self.len.to_usize()), other.len.to_usize())}
            unsafe{alloc::dealloc(other.ptr.as_ptr().cast(), Layout::from_size_align_unchecked(other.cap.to_usize()*size_of::<ELEM>(), align_of::<ELEM>()))};
        } else {
            if IDX::IDX_MAX - other.len > self.len {
                panic!("combine would overflow integer length")
            }
        }
        self.len += other.len;
        other.len = IDX::ZERO;
        other.cap = IDX::ZERO;
        other.ptr = NonNull::dangling();
    }

    pub fn drain_append(&mut self, other: &mut Self, mode: Grow<IDX>) {
        if size_of::<ELEM>() != 0 {
            self.grow_if_needed(ElementCount::Change(other.len), mode);
            unsafe {ptr::copy_nonoverlapping(other.ptr.as_ptr(), self.ptr.as_ptr().add(self.len.to_usize()), other.len.to_usize())}
        }
        other.len = IDX::ZERO;
        self.len += other.len;
    }

    pub fn clone_append(&mut self, other: &Self, mode: Grow<IDX>)
    where ELEM: Clone {
        if size_of::<ELEM>() != 0 {
            self.grow_if_needed(ElementCount::Change(other.len), mode);
            for elem in other.iter() {
                unsafe{self.push_unchecked(elem.clone())};
            }
        }
        self.len += other.len;
    }

    #[inline(always)]
    pub unsafe fn index_unchecked(&self, idx: IDX) -> &ELEM {
        if size_of::<ELEM>() != 0 {
            &*self.ptr.as_ptr().add(idx.to_usize())
        } else {
             self.ptr.as_ref()
        }
    }

    #[inline(always)]
    pub unsafe fn index_mut_unchecked(&mut self, idx: IDX) -> &mut ELEM {
        if size_of::<ELEM>() != 0 {
            &mut *self.ptr.as_ptr().add(idx.to_usize())
        } else {
             self.ptr.as_mut()
        }
    }

    #[inline(always)]
    pub fn index_optional(&self, idx: IDX) -> Option<&ELEM> {
        if idx < self.len {
            Some(unsafe {self.index_unchecked(idx)})
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn index_mut_optional(&mut self, idx: IDX) -> Option<&mut ELEM> {
        if idx < self.len {
            Some(unsafe {self.index_mut_unchecked(idx)})
        } else {
            None
        }
    }

    pub fn drain<'vec>(&'vec mut self) -> XVecDrain<'vec, ELEM, IDX> {
        let drain = XVecDrain {
            vec: PhantomData,
            ptr: self.ptr,
            start: IDX::ZERO,
            count: self.len
        };
        self.len = IDX::ZERO;
        drain
    }
}

impl<ELEM, IDX> Drop for XVec<ELEM, IDX>
where IDX: XVecIndex {
    fn drop(&mut self) {
        self.clear();
        if size_of::<ELEM>() != 0 {
            if self.cap > IDX::ZERO {
                let layout = Layout::array::<ELEM>(self.cap.to_usize()).unwrap();
                unsafe {alloc::dealloc(self.ptr.as_ptr().cast(), layout)};
            }
        }
    }
}

impl<ELEM, IDX> Deref for XVec<ELEM, IDX>
where IDX: XVecIndex {
    type Target = [ELEM];

    fn deref(&self) -> &Self::Target {
        unsafe {slice::from_raw_parts(self.ptr.as_ptr(), self.len.to_usize()) }
    }
}

impl<ELEM, IDX> DerefMut for XVec<ELEM, IDX>
where IDX: XVecIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len.to_usize()) }
    }
}

unsafe impl<ELEM, IDX> Send for XVec<ELEM, IDX> where ELEM: Send, IDX: XVecIndex {}
unsafe impl<ELEM, IDX> Sync for XVec<ELEM, IDX> where ELEM: Sync, IDX: XVecIndex {}

impl<ELEM, IDX> Index<IDX> for XVec<ELEM, IDX>
where IDX: XVecIndex {
    type Output = ELEM;
    #[inline(always)]
    fn index(&self, index: IDX) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds (len = {}, idx = {})", self.len, index);
        }
        unsafe {self.index_unchecked(index)}
    }
}

impl<ELEM, IDX> IndexMut<IDX> for XVec<ELEM, IDX>
where IDX: XVecIndex {
    #[inline(always)]
    fn index_mut(&mut self, index: IDX) -> &mut ELEM {
        if index >= self.len {
            panic!("index out of bounds (len = {}, idx = {})", self.len, index);
        }
        unsafe {self.index_mut_unchecked(index)}
    }
}

impl<ELEM, IDX> Index<Range<IDX>> for XVec<ELEM, IDX>
where IDX: XVecIndex {
    type Output = [ELEM];
    #[inline(always)]
    fn index(&self, index: Range<IDX>) -> &Self::Output {
         Index::index(&**self, (index.start.to_usize())..(index.end.to_usize()))
    }
}

impl<ELEM, IDX> IndexMut<Range<IDX>> for XVec<ELEM, IDX>
where IDX: XVecIndex {
    #[inline(always)]
    fn index_mut(&mut self, index: Range<IDX>) -> &mut[ELEM] {
         IndexMut::index_mut(&mut **self, (index.start.to_usize())..(index.end.to_usize()))
    }
}

impl<ELEM, IDX> Index<RangeInclusive<IDX>> for XVec<ELEM, IDX>
where IDX: XVecIndex {
    type Output = [ELEM];
    #[inline(always)]
    fn index(&self, index: RangeInclusive<IDX>) -> &Self::Output {
         Index::index(&**self, (index.start().to_usize())..=(index.end().to_usize()))
    }
}

impl<ELEM, IDX> IndexMut<RangeInclusive<IDX>> for XVec<ELEM, IDX>
where IDX: XVecIndex {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeInclusive<IDX>) -> &mut[ELEM] {
         IndexMut::index_mut(&mut **self, (index.start().to_usize())..=(index.end().to_usize()))
    }
}

impl<ELEM, IDX> Index<RangeFrom<IDX>> for XVec<ELEM, IDX>
where IDX: XVecIndex {
    type Output = [ELEM];
    #[inline(always)]
    fn index(&self, index: RangeFrom<IDX>) -> &Self::Output {
         Index::index(&**self, (index.start.to_usize())..)
    }
}

impl<ELEM, IDX> IndexMut<RangeFrom<IDX>> for XVec<ELEM, IDX>
where IDX: XVecIndex {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeFrom<IDX>) -> &mut[ELEM] {
         IndexMut::index_mut(&mut **self, (index.start.to_usize())..)
    }
}

impl<ELEM, IDX> Index<RangeTo<IDX>> for XVec<ELEM, IDX>
where IDX: XVecIndex {
    type Output = [ELEM];
    #[inline(always)]
    fn index(&self, index: RangeTo<IDX>) -> &Self::Output {
         Index::index(&**self, ..(index.end.to_usize()))
    }
}

impl<ELEM, IDX> IndexMut<RangeTo<IDX>> for XVec<ELEM, IDX>
where IDX: XVecIndex {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeTo<IDX>) -> &mut[ELEM] {
         IndexMut::index_mut(&mut **self, ..(index.end.to_usize()))
    }
}

impl<ELEM, IDX> Index<RangeFull> for XVec<ELEM, IDX>
where IDX: XVecIndex {
    type Output = [ELEM];
    #[inline(always)]
    fn index(&self, index: RangeFull) -> &Self::Output {
         Index::index(&**self, index)
    }
}

impl<ELEM, IDX> IndexMut<RangeFull> for XVec<ELEM, IDX>
where IDX: XVecIndex {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeFull) -> &mut[ELEM] {
         IndexMut::index_mut(&mut **self, index)
    }
}

impl<ELEM, IDX> Index<RangeToInclusive<IDX>> for XVec<ELEM, IDX>
where IDX: XVecIndex {
    type Output = [ELEM];
    #[inline(always)]
    fn index(&self, index: RangeToInclusive<IDX>) -> &Self::Output {
         Index::index(&**self, ..=(index.end.to_usize()))
    }
}

impl<ELEM, IDX> IndexMut<RangeToInclusive<IDX>> for XVec<ELEM, IDX>
where IDX: XVecIndex {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeToInclusive<IDX>) -> &mut[ELEM] {
         IndexMut::index_mut(&mut **self, ..=(index.end.to_usize()))
    }
}

impl<ELEM, IDX> Clone for XVec<ELEM, IDX>
where IDX: XVecIndex, ELEM: Clone {
    fn clone(&self) -> Self {
        if size_of::<ELEM>() == 0 {
            Self {
                ptr: self.ptr,
                cap: self.cap,
                len: self.len
            }
        } else {
            let (new_ptr, _) = MemUtil::resize_memory_region(None, size_of::<ELEM>(), align_of::<ELEM>(), 0, self.cap.to_usize());
            let new_ptr = new_ptr.cast::<ELEM>();
            let new_short_vec = Self {
                ptr: new_ptr,
                cap: self.cap,
                len: self.len
            };
            let mut idx = IDX::ZERO;
            while idx < self.len {
                unsafe {ptr::write(new_short_vec.ptr.as_ptr().add(idx.to_usize()), self.index_unchecked(idx).clone())};
                idx += IDX::ONE;
            }
            new_short_vec
        }
    }
}



impl<ELEM, IDX> IntoIterator for XVec<ELEM, IDX>
where IDX: XVecIndex {
    type Item = ELEM;
    type IntoIter = XVecIter<ELEM, IDX>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = XVecIter{
            ptr: self.ptr,
            cap: self.cap,
            start: IDX::ZERO,
            count: self.len,
        };
        mem::forget(self);
        iter
    }
}

