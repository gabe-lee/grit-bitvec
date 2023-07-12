use std::ops::{Add, AddAssign, Sub, SubAssign, Range, RangeInclusive, RangeTo, RangeToInclusive};

use crate::utils::{BitUtil, MemUtil};

#[derive(Clone, Copy, Debug)]
pub struct IdxProxy<const BIT_WIDTH: usize> {
    pub(crate) bitwise_idx: usize,
    pub(crate) real_idx: usize,
    pub(crate) first_offset: usize,
    pub(crate) first_mask: usize,
    pub(crate) second_offset: usize,
    pub(crate) second_mask: usize
}

impl<const BIT_WIDTH: usize> IdxProxy<BIT_WIDTH> {
    pub(crate) const MASK: usize = (1 << BIT_WIDTH) - 1;
    pub(crate) const MAX_CAPACITY: usize = {
        match BIT_WIDTH {
            0 => usize::MAX,
            _ => Self::calc_elem_count_from_total_bits(BitUtil::calc_total_bits_in_num_usize(MemUtil::MAX_CAPACITY_FOR_USIZE))
        }
    };

    #[inline(always)]
    pub fn idx(&self) -> usize {
        self.bitwise_idx
    }

    #[inline(always)]
    pub(crate) const fn as_real_len_or_cap(&self) -> usize {
        self.real_idx + BitUtil::one_if_val_isnt_zero(self.first_offset)
    }

    #[inline(always)]
    pub(crate) const fn calc_elem_count_from_total_bits(total_bits: usize) -> usize {
        match BIT_WIDTH {
            1 => total_bits,
            2 => total_bits >> 1,
            4 => total_bits >> 2,
            8 => total_bits >> 3,
            16 => total_bits >> 4,
            32 => total_bits >> 5,
            64 => total_bits >> 6,
            128 => total_bits >> 7,
            _ => total_bits / BIT_WIDTH
        }
    }

    #[inline(always)]
    pub(crate) const fn calc_bitwise_count_from_real_count(count: usize) -> usize {
        let total_bits = BitUtil::calc_total_bits_in_num_usize(count);
        Self::calc_elem_count_from_total_bits(total_bits)
    }

    #[inline(always)]
    pub(crate) fn calc_real_count_from_bitwise_count(count: usize) -> usize {
        let idx_proxy = Self::from(count);
        idx_proxy.as_real_len_or_cap()
    }


}

impl<const BIT_WIDTH: usize> From<usize> for IdxProxy<BIT_WIDTH>  {
    #[inline(always)]
    fn from(bitwise_idx: usize) -> Self {
        let (real_idx, first_offset) = match BitUtil::USIZE_BITS {
            64 => match BIT_WIDTH {
                1 => (bitwise_idx >> 6, bitwise_idx & 0b_00111111),
                2 => (bitwise_idx >> 5, (bitwise_idx & 0b_00011111) << 1),
                4 => (bitwise_idx >> 4, (bitwise_idx & 0b_00001111) << 2),
                8 => (bitwise_idx >> 3, (bitwise_idx & 0b_00000111) << 3),
                16 => (bitwise_idx >> 2, (bitwise_idx & 0b_00000011) << 4),
                32 => (bitwise_idx >> 1, (bitwise_idx & 0b_00000001) << 5),
                64 => (bitwise_idx, 0),
                128 => (bitwise_idx << 1, 0),
                _ => {
                    let total_bits = bitwise_idx * BIT_WIDTH;
                    (total_bits >> 6, total_bits & 0b_00111111)
                } 
            },
            32 => match BIT_WIDTH {
                1 => (bitwise_idx >> 5, bitwise_idx & 0b_00011111),
                2 => (bitwise_idx >> 4, (bitwise_idx & 0b_00001111) << 1),
                4 => (bitwise_idx >> 3, (bitwise_idx & 0b_00000111) << 2),
                8 => (bitwise_idx >> 2, (bitwise_idx & 0b_00000011) << 3),
                16 => (bitwise_idx >> 1, (bitwise_idx & 0b_00000001) << 4),
                32 => (bitwise_idx, 0),
                64 => (bitwise_idx << 1, 0),
                128 => (bitwise_idx << 2, 0),
                _ => {
                    let total_bits = bitwise_idx * BIT_WIDTH;
                    (total_bits >> 5, total_bits & 0b_00011111)
                } 
            },
            16 => match BIT_WIDTH {
                1 => (bitwise_idx >> 4, bitwise_idx & 0b_00001111),
                2 => (bitwise_idx >> 3, (bitwise_idx & 0b_00000111) << 1),
                4 => (bitwise_idx >> 2, (bitwise_idx & 0b_00000011) << 2),
                8 => (bitwise_idx >> 1, (bitwise_idx & 0b_00000001) << 3),
                16 => (bitwise_idx, 0),
                32 => (bitwise_idx << 1, 0),
                64 => (bitwise_idx << 2, 0),
                128 => (bitwise_idx << 3, 0),
                _ => {
                    let total_bits = bitwise_idx * BIT_WIDTH;
                    (total_bits >> 4, total_bits & 0b_00001111)
                } 
            }
            _ => {
                let total_bits = bitwise_idx * BIT_WIDTH;
                (total_bits / BitUtil::USIZE_BITS, total_bits % BitUtil::USIZE_BITS)
            }
        };
        let second_offset = BitUtil::USIZE_BITS - first_offset;
        let first_mask = Self::MASK << first_offset;
        let second_mask = BitUtil::right_shift_discard_if_ubits(Self::MASK, second_offset);
        Self {
            bitwise_idx,
            real_idx,
            first_offset,
            first_mask,
            second_offset,
            second_mask
        }
    }
}

impl<const BIT_WIDTH: usize> Add<usize> for IdxProxy<BIT_WIDTH>  {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.bitwise_idx + rhs)
    }
}

impl<const BIT_WIDTH: usize> AddAssign<usize> for IdxProxy<BIT_WIDTH> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: usize) {
        *self = self.add(rhs)
    }
}

impl<const BIT_WIDTH: usize> Sub<usize> for IdxProxy<BIT_WIDTH>  {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: usize) -> Self::Output {
        Self::from(self.bitwise_idx - rhs)
    }
}

impl<const BIT_WIDTH: usize> SubAssign<usize> for IdxProxy<BIT_WIDTH> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: usize) {
        *self = self.sub(rhs)
    }
}

impl<const BIT_WIDTH: usize> Ord for IdxProxy<BIT_WIDTH> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bitwise_idx.cmp(&other.bitwise_idx)
    }
}

impl<const BIT_WIDTH: usize> PartialOrd<Self> for IdxProxy<BIT_WIDTH> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.bitwise_idx.partial_cmp(&other.bitwise_idx)
    }
}

impl<const BIT_WIDTH: usize> PartialEq<Self> for IdxProxy<BIT_WIDTH> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.bitwise_idx == other.bitwise_idx
    }
}
impl<const BIT_WIDTH: usize> Eq for IdxProxy<BIT_WIDTH> {}

pub struct IdxProxyRange<const BIT_WIDTH: usize> {
    pub(crate) start: usize,
    pub(crate) end_excluded: usize
}

impl<const BIT_WIDTH: usize> From<Range<usize>> for IdxProxyRange<BIT_WIDTH> {
    fn from(value: Range<usize>) -> Self {
        Self { start: value.start, end_excluded: value.end }
    }
}

impl<const BIT_WIDTH: usize> From<RangeInclusive<usize>> for IdxProxyRange<BIT_WIDTH> {
    fn from(value: RangeInclusive<usize>) -> Self {
        Self { start: *value.start(), end_excluded: *value.end()+1 }
    }
}

impl<const BIT_WIDTH: usize> From<RangeTo<usize>> for IdxProxyRange<BIT_WIDTH> {
    fn from(value: RangeTo<usize>) -> Self {
        Self { start: 0, end_excluded: value.end }
    }
}

impl<const BIT_WIDTH: usize> From<RangeToInclusive<usize>> for IdxProxyRange<BIT_WIDTH> {
    fn from(value: RangeToInclusive<usize>) -> Self {
        Self { start: 0, end_excluded: value.end+1 }
    }
}

impl<const BIT_WIDTH: usize> Iterator for IdxProxyRange<BIT_WIDTH> {
    type Item = IdxProxy<BIT_WIDTH>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.start < self.end_excluded {
            true => {
                let idx_proxy = IdxProxy::<BIT_WIDTH>::from(self.start);
                self.start += 1;
                Some(idx_proxy)
            },
            false => None
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.end_excluded - self.start;
        (count, Some(count))
    }
}

impl<const BIT_WIDTH: usize> DoubleEndedIterator for IdxProxyRange<BIT_WIDTH> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.start < self.end_excluded {
            true => {
                let idx_proxy = IdxProxy::<BIT_WIDTH>::from(self.end_excluded-1);
                self.end_excluded -= 1;
                Some(idx_proxy)
            },
            false => None
        }
    }
}

impl<const BIT_WIDTH: usize> ExactSizeIterator for IdxProxyRange<BIT_WIDTH> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.end_excluded - self.start
    }
}

// #[cfg(test)]
// mod test {
//     use std::ops::{Sub, Add};

//     use crate::*;

//     #[test]
//     fn math() {
//         let idx_proxy = IdxProxy::<5>{ real_idx: 1, bit_offset: 3};
//         let minus = idx_proxy.sub(1);
//         let minus_minus = minus.sub(2);
//         let plus = idx_proxy.add(1);
//         let plus_plus = idx_proxy.add(13);
//         let minus_plus = minus.add(1);
//         assert_eq!(minus, IdxProxy::<5>{ real_idx: 0, bit_offset: 62});
//         assert_eq!(minus_minus, IdxProxy::<5>{ real_idx: 0, bit_offset: 52});
//         assert_eq!(plus, IdxProxy::<5>{ real_idx: 1, bit_offset: 8});
//         assert_eq!(plus_plus, IdxProxy::<5>{ real_idx: 2, bit_offset: 4});
//         assert_eq!(minus_plus, IdxProxy::<5>{ real_idx: 1, bit_offset: 3});
//     }
// }
