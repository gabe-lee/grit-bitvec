use crate::{
    size_of,
    align_of,
    ptr,
    NonNull,
    alloc,
    Layout,
};

#[cfg(not(no_global_oom_handling))]
use crate::handle_alloc_error;

pub(crate) struct MemUtil;

impl MemUtil {
    pub(crate) const MAX_CAPACITY_FOR_USIZE : usize =  Self::max_capacity_for_type(size_of::<usize>(), align_of::<usize>());

    #[inline(always)]
    pub(crate) const fn max_capacity_for_type(type_size: usize, type_align: usize) -> usize {
        (isize::MAX as usize - (type_align - 1)) / type_size
    }

    #[inline(never)]
    pub(crate) fn resize_memory_region(old_u8_ptr: Option<*mut u8>, type_size: usize, type_align: usize, current_capacity: usize, target_capacity: usize) -> (NonNull<u8>, usize) {
        if target_capacity > Self::max_capacity_for_type(type_size, type_align) {
            panic!("capacity would overfow user-space memory");
        }
        let new_layout = unsafe { Layout::from_size_align_unchecked(target_capacity*type_size, type_align) };
        let new_ptr = match old_u8_ptr {
            Some(old_ptr) => {
                let old_layout = unsafe { Layout::from_size_align_unchecked(current_capacity*type_size, type_align) };
                unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
            },
            None => {
                unsafe { alloc::alloc(new_layout) }
            }
        };
        match NonNull::new(new_ptr) {
            Some(non_null) => (non_null, target_capacity),
            None => {
                if cfg!(not(no_global_oom_handling)) {
                    handle_alloc_error(new_layout)
                } else {
                    panic!("could not allocate memory!")
                }
            },
        }
    }

    #[inline(never)]
    pub(crate) fn resize_memory_region_of_usize(old_u8_ptr: Option<*mut u8>, current_capacity: usize, target_capacity: usize) -> (NonNull<u8>, usize) {
        if target_capacity > Self::MAX_CAPACITY_FOR_USIZE {
            panic!("capacity would overfow user-space memory");
        }
        let new_layout = unsafe { Layout::from_size_align_unchecked(target_capacity*size_of::<usize>(), align_of::<usize>()) };
        let new_ptr = match old_u8_ptr {
            Some(old_ptr) => {
                let old_layout = unsafe { Layout::from_size_align_unchecked(current_capacity*size_of::<usize>(), align_of::<usize>()) };
                unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
            },
            None => {
                unsafe { alloc::alloc(new_layout) }
            }
        };
        match NonNull::new(new_ptr) {
            Some(non_null) => (non_null, target_capacity),
            None => {
                if cfg!(not(no_global_oom_handling)) {
                    handle_alloc_error(new_layout)
                } else {
                    panic!("could not allocate memory!")
                }
            },
        }
    }


    pub(crate) fn drop_range_in_place<T>(start_ptr: *mut T, count: usize) {
        let mut idx: usize = 0;
        while idx < count {
            unsafe{ptr::drop_in_place(start_ptr.add(idx))};
            idx += 1;
        }
    }
}

pub(crate) struct BitUtil;

impl BitUtil {
    #[inline(always)]
    pub(crate) const fn smear_left(mut val: usize) -> usize {
        match usize::BITS {
            64 => {
                val |= val << 1;
                val |= val << 2;
                val |= val << 4;
                val |= val << 8;
                val |= val << 16;
                val << 32
            },
            32 => {
                val |= val << 1;
                val |= val << 2;
                val |= val << 4;
                val |= val << 8;
                val << 16
            },
            16 => {
                val |= val << 1;
                val |= val << 2;
                val |= val << 4;
                val << 8
            },
            8 => {
                val |= val << 1;
                val |= val << 2;
                val << 4
            },
            _ => {
                val |= val << 1;
                val |= val << 2;
                val |= val << 4;
                val |= val << 8;
                val |= val << 16;
                val << 32
            },
        }
    }

    #[inline(always)]
    pub(crate) const fn smear_neg_bit_left(val: usize, top_bit: usize) -> usize {
        val | Self::smear_left(top_bit)
    }

    #[inline(always)]
    pub(crate) const fn smear_right(mut val: usize) -> usize {
        match usize::BITS {
            64 => {
                val |= val >> 1;
                val |= val >> 2;
                val |= val >> 4;
                val |= val >> 8;
                val |= val >> 16;
                val >> 32
            },
            32 => {
                val |= val >> 1;
                val |= val >> 2;
                val |= val >> 4;
                val |= val >> 8;
                val >> 16
            },
            16 => {
                val |= val >> 1;
                val |= val >> 2;
                val |= val >> 4;
                val >> 8
            },
            8 => {
                val |= val >> 1;
                val |= val >> 2;
                val >> 4
            },
            _ => {
                val |= val >> 1;
                val |= val >> 2;
                val |= val >> 4;
                val |= val >> 8;
                val |= val >> 16;
                val >> 32
            },
        }
    }

    #[inline(always)]
    pub(crate) const fn all_bits_less_than_bit(bit_number: usize) -> usize {
        Self::smear_right(1 << bit_number) >> 1
    }

    // #[inline(always)]
    // pub(crate) const fn all_bits_greater_than_or_equal_to_bit(bit_number: usize) -> usize {
    //     Self::smear_left(1 << bit_number)
    // }

    // #[inline(always)]
    // pub(crate) const fn all_bits_greater_than_bit(bit_number: usize) -> usize {
    //     Self::smear_left(1 << bit_number + 1)
    // }
}