/*!
# grit-bitvec

This crate provides `RawBitVec` and ~~`TypedBitVec`~~, a vector that can store data elements of fixed-size bit widths that do not fall into the 8/16/32/64/128
size categories. For example, rather than storing a vector of `bool`s in one byte each, you can store them each as one bit. Another example would be storing unsigned integers that are always in the range 0-3 as exactly 2 bits, 0-7 as 3 bits, or 0-15 as 4 bits, or even a signed integer in the range -1024 to 1023 as 11 bits.

Both `Raw-` and `Typed-` versions use `usize` as the underlying data block type to take advantage of any possible arithmetic optimizations on native-size words, meaning that the maximum bit-width supported is the same as `usize::BITS`

This allows considerable gains in memory usage for applications where the number of elements may be non-trivial, at the cost of processing cost to access the elements.

The additional processing cost is not terrible in most cases, as it is mostly performed with bitwise shifts and simple arithmetic, and is further reduced by using constant comparisons to reduce many bitwise math functions to their easiest possible form. However they are not free, and operations that insert or remove elements in the middle of the `BitVec` may be even more costly due to the need to run those checks and shifts on every element rather than using `ptr::copy()` like `Vec` does internally

~~by default the `small_int_impls` feature is enabled, providing an simple `BitElem` implementation for `bool` and integer types smaller than 16 bits (for example `u8_as_u3` or `i16_as_i11`), and the `large_int_impls` feature can be activated to get similar implementations for bit widths less than `usize::BITS`~~

This crate currently has no documentation, and full testing is ongoing:
### Tested Functions
(self-evident) new()  
(self-evident) with_capacity()  
(self-evident) len()  
(self-evident) cap()  
(self-evident) free()  
(self-evident) clear()  
[x] push()  
[x] pop()  
[ ] insert()  
[ ] remove()  
[ ] insert_bitvec()  
[ ] insert_iter()  
[ ] remove_range()  
[ ] swap()  
[ ] swap_pop()  
[ ] trim_excess_capacity()  
[ ] append_bitvec()  
[ ] append_iter()  
[ ] get()  
[ ] set()  
[ ] replace()  
[ ] drain()  
[ ] into_iter()  


This library is very much in the "unstable" phase and the API may change in the future
*/

pub(crate) use core::{
    mem::{
        self,
        size_of,
        align_of,
        // needs_drop,
    },
    ptr::{
        self,
        NonNull,
    },
    // marker::PhantomData,
};
pub(crate) use std::alloc::{self, Layout};

mod index_proxy;
pub use index_proxy::*;

mod raw_bitvec;
pub use raw_bitvec::*;
mod raw_bitvec_iter;
pub use raw_bitvec_iter::*;

mod typed_bitvec;
pub use typed_bitvec::*;

mod bitvec_elem;
pub use bitvec_elem::*;

mod utils;
pub(crate) use utils::*;

#[cfg(test)]
mod raw_bitvec_test;



