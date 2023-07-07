use crate::{
    size_of,
    align_of,
};

pub(crate) struct MemUtil;

impl MemUtil {
    pub(crate) const MAX_CAPACITY_FOR_USIZE : usize =  Self::max_capacity_for_type(size_of::<usize>(), align_of::<usize>());

    #[inline(always)]
    pub(crate) const fn max_capacity_for_type(type_size: usize, type_align: usize) -> usize {
        (isize::MAX as usize - (type_align - 1)) / type_size
    }
}

pub(crate) struct BitUtil;

impl BitUtil {
    pub(crate) const USIZE_BYTES: usize = usize::BITS as usize >> 3;
    pub(crate) const USIZE_BITS: usize = usize::BITS as usize;
    
    #[inline(always)]
    pub(crate) const fn smear_left(mut val: usize) -> usize {
        if usize::BITS > 1 {
            val |= val << 1;
        }
        if usize::BITS > 2 {
            val |= val << 2;
        }
        if usize::BITS > 4 {
            val |= val << 4;
        }
        if usize::BITS > 8 {
            val |= val << 8;
        }
        if usize::BITS > 16 {
            val |= val << 16;
        }
        if usize::BITS > 32 {
            val |= val << 32;
        }
        val
    }

    #[inline(always)]
    pub(crate) const fn smear_neg_bit_left(val: usize, top_bit: usize) -> usize {
        val | Self::smear_left(top_bit)
    }

    #[inline(always)]
    pub(crate) const fn smear_right(mut val: usize) -> usize {
        if usize::BITS > 1 {
            val |= val >> 1;
        }
        if usize::BITS > 2 {
            val |= val >> 2;
        }
        if usize::BITS > 4 {
            val |= val >> 4;
        }
        if usize::BITS > 8 {
            val |= val >> 8;
        }
        if usize::BITS > 16 {
            val |= val >> 16;
        }
        if usize::BITS > 32 {
            val |= val >> 32;
        }
        val
    }

    #[inline(always)]
    pub(crate) const fn all_bits_less_than_bit(bit_number: usize) -> usize {
        Self::smear_right(1 << bit_number) >> 1
    }

    // #[inline(always)]
    // pub(crate) fn saturating_left_shift_mult(val: usize, shift: usize) -> usize {
    //     let saturating_mask = Self::smear_left(1 << (Self::USIZE_BITS - shift));
    //     let saturated = Self::smear_right(val & saturating_mask);
    //     saturated | (val << shift)
    // }

    #[inline(always)]
    pub(crate) const fn one_if_val_isnt_zero(mut val: usize) -> usize {
        val = Self::smear_right(val);
        val & 1
    }

    #[inline(always)]
    pub(crate) const fn zero_mask_if_bit_offset_is_zero(mut bit_off: usize) -> usize {
        bit_off |= bit_off << 1;
        bit_off |= bit_off << 2;
        bit_off |= bit_off << 4;
        bit_off |= bit_off >> 8;
        bit_off |= bit_off << 8;
        bit_off |= bit_off << 16;
        bit_off | bit_off << 32
    }

    #[inline(always)]
    pub(crate) const fn calc_total_bits_in_num_usize(num_usize: usize) -> usize {
        match Self::USIZE_BITS {
            64 => num_usize << 6,
            32 => num_usize << 5,
            16 => num_usize << 4,
            _ => num_usize * Self::USIZE_BITS
        }
    }

    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero_mask_if_bit_offset_is_zero() {
        assert_eq!(0, BitUtil::zero_mask_if_bit_offset_is_zero(0));
        assert_eq!(usize::MAX, BitUtil::zero_mask_if_bit_offset_is_zero(1));
        assert_eq!(usize::MAX, BitUtil::zero_mask_if_bit_offset_is_zero(3));
        assert_eq!(usize::MAX, BitUtil::zero_mask_if_bit_offset_is_zero(10));
        assert_eq!(usize::MAX, BitUtil::zero_mask_if_bit_offset_is_zero(42));
        assert_eq!(usize::MAX, BitUtil::zero_mask_if_bit_offset_is_zero(63));
        assert_eq!(usize::MAX, BitUtil::zero_mask_if_bit_offset_is_zero(64));
        assert_eq!(usize::MAX, BitUtil::zero_mask_if_bit_offset_is_zero(128));
        assert_eq!(usize::MAX, BitUtil::zero_mask_if_bit_offset_is_zero(256));
        // fails at bit offset 512 or greater
        assert_ne!(usize::MAX, BitUtil::zero_mask_if_bit_offset_is_zero(512));
    }
}