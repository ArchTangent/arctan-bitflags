//! BitFlags with a `u128` representation.
//!
//! __NOTE__:
//! - JSON and RON (de)serialization of `BitFlags128` is _not_ supported for `nanoserde`.
//! - BIN (de)serialization of `BitFlags128` is supported for both `serde` and `nanoserde`.

use core::convert::TryFrom;

/// 128-bit bitflags, indexed from bit indexes `[0]` to `[127]`.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitFlags128(pub u128);

impl BitFlags128 {
    /// Returns a new instance with all bits set to `0`.
    pub fn new() -> Self {
        Self::empty()
    }
    /// Returns a new instance with all bits set to `0`.
    #[inline]
    pub fn empty() -> Self {
        Self(0)
    }
    /// Returns a new instance with all bits set to `1`.
    #[inline]
    pub fn full() -> Self {
        Self(u128::MAX)
    }
    /// Returns a new instance with the first `n` (1-128) bits set. 
    ///
    /// __Panics__ if `n < 1` or `n > 128`.
    #[inline]
    pub fn with_first_n_set(n: u8) -> Self {
        assert!(n > 0 && n < 129, "Value must be from 1 to 128");
        
        let v = 1 << n - 1;       
        
        BitFlags128(v | (v-1))
    }    
    /// Returns a new instance from a `u128`.
    #[inline]
    pub fn from_u128(val: u128) -> Self {
        Self(val)
    }
    /// Returns the underlying `u128` value.
    #[inline]
    pub fn to_u128(&self) -> u128 {
        self.0
    }
    /// Converts an index (0-127) into a `BitFlags128`.
    ///
    /// __Panics__ if `index > 127`.
    #[inline]
    pub fn from_index(index: usize) -> Self {
        Self::try_from(index).unwrap()
    }
    /// Converts a slice of indexes (0-127) into a `BitFlags128`.
    #[inline]
    pub fn from_slice(s: &[usize]) -> Self {
        let mut bits = Self(0);

        for index in s.iter() {
            let temp_bits = Self::from_index(*index);
            bits.insert(temp_bits);
        }

        bits
    }
    /// Returns `true` if _no_ bits are set.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    /// Returns `true` if _all_ bits are set.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.0 == u128::MAX
    }
    /// Returns `true` if `self` and `other` have _at least one_ matching set bit.
    #[inline]
    pub fn intersects(&self, other: Self) -> bool {
        (self.0 & other.0) > 0
    }
    /// Returns the bitwise `AND` (`&`) of two flags.
    #[inline]
    pub fn intersection(&self, other: Self) -> Self {
        BitFlags128(self.0 & other.0)
    }
    /// Returns the bits set in `self` that are _not_ set in `other`.
    #[inline]
    pub fn difference(&self, other: Self) -> Self {
        BitFlags128(self.0 & !other.0)
    }
    /// Returns the bits set in `self` or `other`, but _not_ both, using bitwise `XOR` (`^`).
    #[inline]
    pub fn symmetric_difference(&self, other: Self) -> Self {
        BitFlags128(self.0 ^ other.0)
    }
    /// Returns the bitwise `OR` (`|`) of two flags.
    #[inline]
    pub fn union(&self, other: Self) -> Self {
        BitFlags128(self.0 | other.0)
    }
    /// Returns the bitwise negation (`!`) of given flags.
    #[inline]
    pub fn complement(&self) -> Self {
        BitFlags128(!self.0)
    }
    /// Returns `true` if current flags contain _all_ incoming flags.
    #[inline]
    pub fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    /// Inserts `other` flags into current flags using bitwise `OR` (`|`).
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 = self.0 | other.0;
    }
    /// Sets bit at given index (0-127).
    ///
    /// __Panics__ if `index > 127`.
    #[inline]
    pub fn insert_at_index(&mut self, index: usize) {
        assert!(index < 128, "BitFlags128 structs are indexed from 0 to 127");
        self.0 = self.0 | 2_u128.pow(index as u32);
    }
    /// Inserts `other` if `value` is `true`; removes `other` if `value` is `false`.
    #[inline]
    pub fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    /// Sets bit at given index (0-127) to specific value (`true` = `1`; `false` = `0`).
    ///
    /// __Panics__ if `index > 127`.
    #[inline]
    pub fn set_at_index(&mut self, index: usize, value: bool) {
        assert!(index < 128, "BitFlags128 structs are indexed from 0 to 127");
        if value {
            self.insert_at_index(index);
        } else {
            self.remove_at_index(index);
        }
    }
    /// Toggles bits based on mask (using bitwise `XOR`).
    #[inline]
    pub fn toggle(&mut self, mask: Self) {
        self.0 = self.0 ^ mask.0;
    }
    /// Toggles bit at given index (0-127).
    ///
    /// __Panics__ if `index > 127`.
    #[inline]
    pub fn toggle_at_index(&mut self, index: usize) {
        assert!(index < 128, "BitFlags128 structs are indexed from 0 to 127");
        self.0 = self.0 ^ 2_u128.pow(index as u32);
    }
    /// Unsets bits that match those of incoming `BitFlags128` (bitwise `AND NOT`).
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 = self.0 & !other.0;
    }
    /// Unsets bit at given index (0-127). Indexes 0-127 allowed.
    ///
    /// __Panics__ if `index > 127`.
    #[inline]
    pub fn remove_at_index(&mut self, index: usize) {
        assert!(index < 128, "BitFlags128 structs are indexed from 0 to 127");
        self.0 = self.0 & !2_u128.pow(index as u32);
    }
    /// Returns the number of bits.
    #[inline]
    pub fn num_bits() -> usize {
        128
    }
    /// Returns value of bit at given index (`0` is `false`; `1` is `true`).
    ///
    /// __Panics__ if `index > 127`.
    #[inline]
    pub fn bit_at_index(&self, index: usize) -> bool {
        assert!(index < 128, "BitFlags128 structs are indexed from 0 to 127");
        self.0 & 2_u128.pow(index as u32) > 0
    }
    /// Returns value of bit at given index (`0` is `false`; `1` is `true`). Returns `None` if out
    /// of bounds.
    #[inline]
    pub fn get_bit_at_index(&self, index: usize) -> Option<bool> {
        if index < 128 {
            return Some((self.0 & 2_u128.pow(index as u32)) > 0);
        }
        None
    }
    /// Returns the value of the highest set bit. If none, returns empty flags.
    #[inline]
    pub fn highest_set_bit(&self) -> Self {
        let mut n = self.0.clone();
        n |= n >> 1;
        n |= n >> 2;
        n |= n >> 4;
        n |= n >> 8;
        n |= n >> 16;
        n |= n >> 32;
        n |= n >> 64;
        n -= n >> 1;
        BitFlags128(n)
    }
    /// Returns the index of the highest set bit of the bitflag, if present.
    #[inline]
    pub fn highest_set_bit_index(&self) -> Option<usize> {
        if self.0 == 0 {
            return None;
        }

        let mut val = self.0.clone();
        let mut bit_ix = 0;
        while val > 0 {
            bit_ix += 1;
            val >>= 1;
        }

        Some(bit_ix)
    }
    /// Counts the number of ones in the bitflag.
    #[inline]
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }
    /// Iterates over set bits of the structure. Returns `Some(bit_index)` if the
    /// bit is set, otherwise `None`.
    ///
    /// E.g. collecting `0b00001001` into a vector would produce `vec![0, 3]`,
    /// representing the 0th and 3rd indexes.
    #[inline]
    pub fn iter(&self) -> BitFlagsIter128 {
        BitFlagsIter128 { current_bit: 0, bits: self.0 }
    }
}

impl From<u128> for BitFlags128 {
    fn from(value: u128) -> Self {
        BitFlags128(value)
    }
}

impl TryFrom<u32> for BitFlags128 {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 128 {
            Ok(Self(2_u128.pow(value)))
        } else {
            Err("BitFlags128 structs are indexed from 0 to 127")
        }
    }
}

impl TryFrom<usize> for BitFlags128 {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 128 {
            Ok(Self(2_u128.pow(value as u32)))
        } else {
            Err("BitFlags128 structs are indexed from 0 to 127")
        }
    }
}

impl core::fmt::Display for BitFlags128 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BitFlags128({})", self.0)
    }
}

impl core::fmt::Binary for BitFlags128 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#0130b}", self.0)
    }
}

impl core::ops::BitOr<BitFlags128> for BitFlags128 {
    type Output = BitFlags128;

    fn bitor(self, rhs: BitFlags128) -> Self::Output {
        BitFlags128(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<BitFlags128> for BitFlags128 {
    fn bitor_assign(&mut self, rhs: BitFlags128) {
        self.0 |= rhs.0
    }
}

impl core::ops::BitAnd<BitFlags128> for BitFlags128 {
    type Output = BitFlags128;

    fn bitand(self, rhs: BitFlags128) -> Self::Output {
        BitFlags128(self.0 & rhs.0)
    }
}

impl core::ops::BitAndAssign<BitFlags128> for BitFlags128 {
    fn bitand_assign(&mut self, rhs: BitFlags128) {
        self.0 &= rhs.0
    }
}

impl core::ops::BitXor<BitFlags128> for BitFlags128 {
    type Output = BitFlags128;

    fn bitxor(self, rhs: BitFlags128) -> Self::Output {
        BitFlags128(self.0 ^ rhs.0)
    }
}

impl core::ops::BitXorAssign<BitFlags128> for BitFlags128 {
    fn bitxor_assign(&mut self, rhs: BitFlags128) {
        self.0 ^= rhs.0
    }
}

impl core::ops::Not for BitFlags128 {
    type Output = BitFlags128;

    /// Toggles _all_ bits.
    fn not(self) -> Self::Output {
        BitFlags128(!self.0)
    }
}

/// Iterator over set bits of a `BitFlags128`.
pub struct BitFlagsIter128 {
    current_bit: usize,
    bits:        u128,
}

impl core::iter::Iterator for BitFlagsIter128 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.bits != 0 {
            if self.bits & 1 == 1 {
                let ret = Some(self.current_bit);
                self.bits >>= 1;
                self.current_bit += 1;
                return ret;
            }
            self.bits >>= 1;
            self.current_bit += 1;
        }
        None
    }
}

//  #######   ########   ######   ########  #######
//  ##    ##  ##        ##        ##        ##    ##
//  ##    ##  ######     ######   ######    #######
//  ##    ##  ##              ##  ##        ##   ##
//  #######   ########  #######   ########  ##    ##

#[cfg(feature = "serde-support")]
mod impl_serde {
    use super::BitFlags128;
    use serde::{Deserialize, Serialize};

    impl<'de> Deserialize<'de> for BitFlags128 {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<BitFlags128, D::Error> {
            let val = u128::deserialize(d)?;
            Ok(BitFlags128(val))
        }
    }

    impl Serialize for BitFlags128 {
        fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            u128::serialize(&self.0, s)
        }
    }
}

#[cfg(feature = "nanoserde-support")]
mod impl_nanoserde {
    extern crate std;
    use super::BitFlags128;
    use nanoserde::{DeBin, DeBinErr, SerBin};
    use std::prelude::v1::*;

    impl DeBin for BitFlags128 {
        fn de_bin(offset: &mut usize, bytes: &[u8]) -> Result<Self, DeBinErr> {
            let l = core::mem::size_of::<u128>();
            if *offset + l > bytes.len() {
                return Err(DeBinErr { o: *offset, l: l, s: bytes.len() });
            }
            let val = u128::from_le_bytes(bytes[*offset..(*offset + l)].try_into().unwrap());
            *offset += l;

            Ok(BitFlags128(val))
        }
    }

    impl SerBin for BitFlags128 {
        fn ser_bin(&self, output: &mut Vec<u8>) {
            let bytes = self.0.to_le_bytes();
            output.extend_from_slice(&bytes);
        }
    }
}
