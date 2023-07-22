use crate::{
    BitUtil, 
    Ordering
};

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
pub struct BitProto {
    pub(crate) MASK: usize,
    pub(crate) BITS: usize,
    pub(crate) MAX_CAPACITY: usize
}

impl BitProto {
    pub(crate) const MAX_TRUE_CAP: usize = (usize::MAX / usize::BITS as usize);
    pub(crate) const MAX_TRUE_BITS: usize = Self::MAX_TRUE_CAP * usize::BITS as usize;

    #[inline(always)]
    pub const fn create(bit_width: usize) -> Self {
        if bit_width == 0 {
            panic!("bit_width cannot be 0 (use a zero-typed Vec instead)");
        }
        if bit_width > usize::BITS as usize {
            panic!("bit_width cannot be greater than usize::BITS");
        }
        let mask = if bit_width == usize::BITS as usize {
            usize::MAX
        } else {
            (1 << bit_width) - 1
        };
        Self {
            MASK: mask,
            BITS: bit_width,
            MAX_CAPACITY: Self::MAX_TRUE_BITS / bit_width,
        }
    }

    #[inline(always)]
    pub const fn idx_proxy(proto: BitProto, bitwise_idx: usize) -> IdxProxy {
        let total_bits = bitwise_idx * proto.BITS;
        let (real_idx, first_offset) = match BitUtil::USIZE_BITS {
            64 => (total_bits >> 6, total_bits & 0b_00111111),
            32 => (total_bits >> 5, total_bits & 0b_00011111),
            16 => (total_bits >> 4, total_bits & 0b_00001111),
            _ => (total_bits / BitUtil::USIZE_BITS, total_bits % BitUtil::USIZE_BITS)
        };
        let second_offset = BitUtil::USIZE_BITS - first_offset;
        let first_mask = proto.MASK << first_offset;
        let second_mask = BitUtil::right_shift_discard_if_ubits(proto.MASK, second_offset);
        IdxProxy {
            bitwise_idx,
            real_idx,
            first_offset,
            first_mask,
            second_offset,
            second_mask
        }
    }

    #[inline(always)]
    pub(crate) const fn calc_block_count_from_bitwise_count(proto: BitProto, bitwise_count: usize) -> usize {
        let total_bits = bitwise_count * proto.BITS;
        let (real_count, bit_offset) = match BitUtil::USIZE_BITS {
            64 => (total_bits >> 6, total_bits & 0b_00111111),
            32 => (total_bits >> 5, total_bits & 0b_00011111),
            16 => (total_bits >> 4, total_bits & 0b_00001111),
            _ => (total_bits / BitUtil::USIZE_BITS, total_bits % BitUtil::USIZE_BITS)
        };
        real_count + BitUtil::one_if_val_isnt_zero(bit_offset)
    }

    #[inline(always)]
    pub(crate) const fn calc_bitwise_count_from_block_count(proto: BitProto, block_count: usize) -> usize {
        BitUtil::calc_total_bits_in_num_usize(block_count) / proto.BITS
    }

    #[inline(always)]
    pub(crate) fn check_value(proto: BitProto, val: usize) -> Result<(), String> {
        match val > proto.MASK {
            true => Err(format!("value cannot be represented in {} bits:\nmax bits = {:064b}\nval bits = {:064b}", proto.BITS, proto.MASK, val)),
            false => Ok(())
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct IdxProxy {
    pub(crate) bitwise_idx: usize,
    pub(crate) real_idx: usize,
    pub(crate) first_offset: usize,
    pub(crate) first_mask: usize,
    pub(crate) second_offset: usize,
    pub(crate) second_mask: usize
}

impl IdxProxy {
    #[inline(always)]
    pub fn idx(&self) -> usize {
        self.bitwise_idx
    }
}

impl Ord for IdxProxy {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.bitwise_idx.cmp(&other.bitwise_idx)
    }
}

impl PartialOrd<Self> for IdxProxy {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for IdxProxy {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.bitwise_idx == other.bitwise_idx
    }
}
impl Eq for IdxProxy {}
