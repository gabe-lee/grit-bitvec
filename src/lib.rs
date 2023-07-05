
pub mod x_vec;
pub mod x_vec_iter;
pub mod bx_vec;
pub mod bx_vec_iter;
pub(crate) mod utils;

pub(crate) use core::{
    mem::{
        self,
        size_of,
        align_of,
        needs_drop,
    },
    ptr::{
        self,
        NonNull,
    },
    slice,
    fmt::Display,
    ops::{
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
        RangeToInclusive
    },
    marker::PhantomData,
};
pub(crate) use std::alloc::{self, Layout};

#[cfg(not(no_global_oom_handling))]
pub(crate) use std::alloc::handle_alloc_error;