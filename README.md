# grit-bitvec
This crate provides variants of `BitVec` data structure, a vector that can store data elements of fixed-size bit widths
that do not fall into the 8/16/32/64/128 size categories. For example, rather than storing a vector of `bool`s in one
byte each, you can store them each as one bit. Another example would be storing unsigned integers that are always in the
range 0-3 as exactly 2 bits, 0-7 as 3 bits, 0-15 as 4 bits, or even a signed integer in the range -1024 to 1023 as 11 bits
or a struct with 4 bools and 1 u8 in the range of 0-7 as a total of 7 bits.

- [`RawBitVec`] : the base structure that all other variants wrap. Requires a [`BitProto`] to be passed to every function, and is UNSAFE if the same [`BitProto`] isnt used for all method calls on the same [`RawBitVec`] instance
- [`CProtoBitVec<BIT_WIDTH>`] : a wrapper that takes the needed [`BitProto`] and stores it in a monomorphized constant for every separate `<BIT_WIDTH>`
- [`SProtoBitVec`] : a wrapper that keeps a static reference to the needed [`BitProto`] in every instance
- [`LProtoBitVec`] : a wrapper that keeps a full copy of the [`BitProto`] in every instance
- [`TypedBitVec<T: TypedBitElem>`] : a wrapper that not only stores the [`BitProto`] in a monomorphized constant, but the needed functions to translate the raw returned bits into type `<T>`

All versions use `usize` as the underlying data block type to take advantage of any possible arithmetic optimizations on
native-size words, meaning that the maximum bit-width supported is the same as `usize::BITS`

This allows considerable gains in memory usage for applications where the number of elements may be non-trivial, at the
cost of processing time to access the elements.

The additional processing cost is not terrible in most cases, as it is mostly performed with bitwise shifts and simple
arithmetic, and is further reduced by using constant propogation when applcable to reduce many bitwise math functions
to their easiest possible form. However they are not free, and operations that insert or remove elements in the middle
of the `BitVec` may be even more costly due to the need to run those checks and shifts on every element rather than using
`ptr::copy()` like `Vec` does internally

By default the `small_int_impls` feature is enabled, providing simple `TypedBitElem` implementations for `bool` and
integer types smaller than 16 bits (for example `u8_as_u3` or `i16_as_i11`), and the `large_int_impls` feature can
be activated to get similar implementations for bit widths less than `usize::BITS`

### Tested Functions
- [x] new()  
- [x] with_capacity()  
- [x] len()  
- [x] cap()  
- [x] free()  
- [x] clear()  
- [x] grow_exact_for_total_elements_if_needed()  
- [x] grow_exact_for_additional_elements_if_needed()  
- [x] grow_for_total_elements_if_needed()  
- [x] grow_for_additional_elements_if_needed  
- [x] push()  
- [x] pop()  
- [x] insert()  
- [x] remove()  
- [x] insert_bitvec()  
- [x] insert_iter()  
- [x] remove_range()  
- [x] trim_range()  
- [x] swap()  
- [x] swap_pop()  
- [x] shrink_excess_capacity()  
- [x] append_bitvec()  
- [x] append_iter()  
- [x] get()  
- [x] set()  
- [x] replace()  
- [x] drain()  
- [x] into_iter()  

This crate currently has incomplete documentation and is very much in the "unstable" phase. The API may change in the future