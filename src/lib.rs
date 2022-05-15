#![no_std]
#![doc = include_str!("../README.md")]
#![deny(missing_docs, rustdoc::broken_intra_doc_links)]

use core::ops::Range;

/// Provides a simpler interface to extract from a given value and replace bits in a given value
/// than manipulating the value directly using bit shifting and masking.
pub trait BitsExt {
    /// Extracts a single bit from the integer at the given index. Returns `true` if the bit is
    /// set, `false` otherwise.
    fn extract_bit(self, index: usize) -> bool;

    /// Extracts the given range of bits from the integer.
    fn extract_bits(self, range: Range<usize>) -> Self;

    /// Replaces a single bit in the integer at the given index.
    fn replace_bit(self, index: usize, value: bool) -> Self;

    /// Replaces the given range of bits in the integer with the given value.
    fn replace_bits(self, range: Range<usize>, value: Self) -> Self;
}

macro_rules! bits_ext_impl {
    ($t:ident) => {
        impl BitsExt for $t {
            fn extract_bit(self, index: usize) -> bool {
                (self >> index) & 1 == 1
            }

            fn extract_bits(self, range: Range<usize>) -> Self {
                (self >> range.start) & ((1 << range.len()) - 1)
            }

            fn replace_bit(self, index: usize, value: bool) -> Self {
                (self & !(1 << index)) | ((value as Self) << index)
            }

            fn replace_bits(self, range: Range<usize>, value: Self) -> Self {
                let mask = (1 << range.len()) - 1;

                (self & !(mask << range.start)) | ((value & mask) << range.start)
            }
        }
    }
}

bits_ext_impl!(u8);
bits_ext_impl!(u16);
bits_ext_impl!(u32);
bits_ext_impl!(u64);
bits_ext_impl!(usize);

bits_ext_impl!(i8);
bits_ext_impl!(i16);
bits_ext_impl!(i32);
bits_ext_impl!(i64);
bits_ext_impl!(isize);

#[cfg(test)]
mod tests {
    #[test]
    fn extract_bits() {
        use crate::BitsExt;

        assert_eq!(0xdeadbeef_u32.extract_bits(0..16), 0xbeef);
        assert_eq!(0xdeadbeef_u32.extract_bits(16..32), 0xdead);
    }

    #[test]
    fn replace_bits() {
        use crate::BitsExt;

        assert_eq!(0xdeadbeef_u32.replace_bits(0..16, 0xcafe), 0xdeadcafe);
    }

    #[test]
    fn single_bit() {
        use crate::BitsExt;

        assert_eq!(0xf0_u8.extract_bit(7), true);
        assert_eq!(0xf0_u8.replace_bit(7, true), 0xf0);
        assert_eq!(0xf0_u8.replace_bit(7, false), 0x70);
    }
}
