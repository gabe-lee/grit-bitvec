use crate::{
    BitUtil,
};

pub unsafe trait BitElem {
    type Base;
    const BITS: usize;
    const MASK: usize = (1 << Self::BITS) - 1;
    fn bits_to_val(bits: usize) -> Self::Base;
    fn val_to_bits(val: Self::Base) -> usize;
}

macro_rules! impl_bitelem_unsigned {
    ($(($BASE:ty, $TYPE:ident, $BITS:expr)),+) => {$(
        #[allow(non_camel_case_types)]
        pub struct $TYPE;
        unsafe impl BitElem for $TYPE {
            type Base = $BASE;
            const BITS: usize = $BITS;
            #[inline(always)]
            fn bits_to_val(bits: usize) -> Self::Base {
                bits as Self::Base
            }
            #[inline(always)]
            fn val_to_bits(val: Self::Base) -> usize {
                (val as usize) & Self::MASK
            }
        }
        impl $TYPE {
            pub const MIN: $BASE = 0;
            pub const MAX: $BASE = (1 << Self::BITS) - 1;
        }
    )+};
}
macro_rules! impl_bitelem_signed {
    ($(($BASE:ty, $TYPE:ident, $BITS:expr)),+) => {$(
        #[allow(non_camel_case_types)]
        pub struct $TYPE;
        unsafe impl BitElem for $TYPE {
            type Base = $BASE;
            const BITS: usize = $BITS;
            #[inline(always)]
            fn bits_to_val(bits: usize) -> Self::Base {
                BitUtil::smear_neg_bit_left(bits, Self::TOP_BIT) as Self::Base
            }
            #[inline(always)]
            fn val_to_bits(val: Self::Base) -> usize {
                let mut neg_bit = (val & Self::Base::MIN) as usize;
                neg_bit >>= Self::Base::BITS as usize - Self::BITS;
                (neg_bit | (val as usize)) & Self::MASK
            }
        }
        impl $TYPE {
            pub(crate) const TOP_BIT: usize = 1 << (Self::BITS - 1);
            pub const MIN: $BASE = -(Self::TOP_BIT as $BASE);
            pub const MAX: $BASE = (Self::TOP_BIT - 1) as $BASE;
        }
    )+};
}

#[cfg(feature="small_int_impls")]
unsafe impl BitElem for bool {
    type Base = bool;
    const BITS: usize = 1;
    #[inline(always)]
    fn bits_to_val(bits: usize) -> Self::Base {
        (bits & 1) == 1
    }
    #[inline(always)]
    fn val_to_bits(val: Self::Base) -> usize {
        val as usize
    }
}
#[cfg(feature="small_int_impls")]
impl_bitelem_unsigned!(
    (u8, u8_as_u1, 1),
    (u8, u8_as_u2, 2),
    (u8, u8_as_u3, 3),
    (u8, u8_as_u4, 4),
    (u8, u8_as_u5, 5),
    (u8, u8_as_u6, 6),
    (u8, u8_as_u7, 7),
    (u16, u16_as_u9, 9),
    (u16, u16_as_u10, 10),
    (u16, u16_as_u11, 11),
    (u16, u16_as_u12, 12),
    (u16, u16_as_u13, 13),
    (u16, u16_as_u14, 14),
    (u16, u16_as_u15, 15)
);
#[cfg(feature="small_int_impls")]
impl_bitelem_signed!(
    (i8, i8_as_i1, 1),
    (i8, i8_as_i2, 2),
    (i8, i8_as_i3, 3),
    (i8, i8_as_i4, 4),
    (i8, i8_as_i5, 5),
    (i8, i8_as_i6, 6),
    (i8, i8_as_i7, 7),
    (i16, i16_as_i9, 9),
    (i16, i16_as_i10, 10),
    (i16, i16_as_i11, 11),
    (i16, i16_as_i12, 12),
    (i16, i16_as_i13, 13),
    (i16, i16_as_i14, 14),
    (i16, i16_as_i15, 15)
);
#[cfg(all(feature="large_int_impls",any(target_pointer_width="64",target_pointer_width="32")))]
impl_bitelem_unsigned!(
    (u32, u32_as_u17, 17),
    (u32, u32_as_u18, 18),
    (u32, u32_as_u19, 19),
    (u32, u32_as_u20, 20),
    (u32, u32_as_u21, 21),
    (u32, u32_as_u22, 22),
    (u32, u32_as_u23, 23),
    (u32, u32_as_u24, 24),
    (u32, u32_as_u25, 25),
    (u32, u32_as_u26, 26),
    (u32, u32_as_u27, 27),
    (u32, u32_as_u28, 28),
    (u32, u32_as_u29, 29),
    (u32, u32_as_u30, 30),
    (u32, u32_as_u31, 31)
);
#[cfg(all(feature="large_int_impls",any(target_pointer_width="64",target_pointer_width="32")))]
impl_bitelem_signed!(
    (i32, i32_as_i17, 17),
    (i32, i32_as_i18, 18),
    (i32, i32_as_i19, 19),
    (i32, i32_as_i20, 20),
    (i32, i32_as_i21, 21),
    (i32, i32_as_i22, 22),
    (i32, i32_as_i23, 23),
    (i32, i32_as_i24, 24),
    (i32, i32_as_i25, 25),
    (i32, i32_as_i26, 26),
    (i32, i32_as_i27, 27),
    (i32, i32_as_i28, 28),
    (i32, i32_as_i29, 29),
    (i32, i32_as_i30, 30),
    (i32, i32_as_i31, 31)
);
#[cfg(all(feature="large_int_impls",target_pointer_width="64"))]
impl_bitelem_unsigned!(
    (u64, u64_as_u33, 33),
    (u64, u64_as_u34, 34),
    (u64, u64_as_u35, 35),
    (u64, u64_as_u36, 36),
    (u64, u64_as_u37, 37),
    (u64, u64_as_u38, 38),
    (u64, u64_as_u39, 39),
    (u64, u64_as_u40, 40),
    (u64, u64_as_u41, 41),
    (u64, u64_as_u42, 42),
    (u64, u64_as_u43, 43),
    (u64, u64_as_u44, 44),
    (u64, u64_as_u45, 45),
    (u64, u64_as_u46, 46),
    (u64, u64_as_u47, 47),
    (u64, u64_as_u48, 48),
    (u64, u64_as_u49, 49),
    (u64, u64_as_u50, 50),
    (u64, u64_as_u51, 51),
    (u64, u64_as_u52, 52),
    (u64, u64_as_u53, 53),
    (u64, u64_as_u54, 54),
    (u64, u64_as_u55, 55),
    (u64, u64_as_u56, 56),
    (u64, u64_as_u57, 57),
    (u64, u64_as_u58, 58),
    (u64, u64_as_u59, 59),
    (u64, u64_as_u60, 60),
    (u64, u64_as_u61, 61),
    (u64, u64_as_u62, 62),
    (u64, u64_as_u63, 63)
);
#[cfg(all(feature="large_int_impls",target_pointer_width="64"))]
impl_bitelem_signed!(
    (i64, i64_as_i33, 33),
    (i64, i64_as_i34, 34),
    (i64, i64_as_i35, 35),
    (i64, i64_as_i36, 36),
    (i64, i64_as_i37, 37),
    (i64, i64_as_i38, 38),
    (i64, i64_as_i39, 39),
    (i64, i64_as_i40, 40),
    (i64, i64_as_i41, 41),
    (i64, i64_as_i42, 42),
    (i64, i64_as_i43, 43),
    (i64, i64_as_i44, 44),
    (i64, i64_as_i45, 45),
    (i64, i64_as_i46, 46),
    (i64, i64_as_i47, 47),
    (i64, i64_as_i48, 48),
    (i64, i64_as_i49, 49),
    (i64, i64_as_i50, 50),
    (i64, i64_as_i51, 51),
    (i64, i64_as_i52, 52),
    (i64, i64_as_i53, 53),
    (i64, i64_as_i54, 54),
    (i64, i64_as_i55, 55),
    (i64, i64_as_i56, 56),
    (i64, i64_as_i57, 57),
    (i64, i64_as_i58, 58),
    (i64, i64_as_i59, 59),
    (i64, i64_as_i60, 60),
    (i64, i64_as_i61, 61),
    (i64, i64_as_i62, 62),
    (i64, i64_as_i63, 63)
);

#[derive(Clone, Copy, Debug)]
pub enum Grow {
    Exact,
    ExactPlus(usize),
    OnePointFive,
    Double,
}

impl Default for Grow {
    fn default() -> Self {
        Self::OnePointFive
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Shrink {
    Minimum,
    SubtractOrMinimum(usize),
    SubtractTruncate(usize),
    ThreeQuartersOrMinimum,
    ThreeQuartersTruncate,
    HalfOrMinimum,
    HalfTruncate
}

impl Default for Shrink {
    fn default() -> Self {
        Self::ThreeQuartersOrMinimum
    }
}

pub enum ElementCount {
    Total(usize),
    Additional(usize),
}