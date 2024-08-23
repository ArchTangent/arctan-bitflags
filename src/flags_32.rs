//! BitFlags with a `u32` representation.

use core::convert::TryFrom;

/// 32-bit bitflags, indexed from bit indexes `[0]` to `[31]`.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitFlags32(pub u32);

impl BitFlags32 {
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
        Self(u32::MAX)
    }
    /// Returns a new instance with the first `n` (1-32) bits set. 
    ///
    /// __Panics__ if `n < 1` or `n > 32`.
    #[inline]
    pub fn with_first_n_set(n: u8) -> Self {
        assert!(n > 0 && n < 33, "Value must be from 1 to 32");
        
        let v = 1 << n - 1;       
        
        BitFlags32(v | (v-1))
    }     
    /// Returns a new instance from a `u32`.
    #[inline]
    pub fn from_u32(val: u32) -> Self {
        Self(val)
    }
    /// Returns the underlying `u32` value.
    #[inline]
    pub fn to_u32(&self) -> u32 {
        self.0
    }
    /// Converts an index (0-31) into a `BitFlags32`.
    ///
    /// __Panics__ if `index > 31`.
    #[inline]
    pub fn from_index(index: usize) -> Self {
        Self::try_from(index).unwrap()
    }
    /// Converts a slice of indexes (0-31) into a `BitFlags32`.
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
        self.0 == u32::MAX
    }
    /// Returns `true` if `self` and `other` have _at least one_ matching set bit.
    #[inline]
    pub fn intersects(&self, other: Self) -> bool {
        (self.0 & other.0) > 0
    }
    /// Returns the bitwise `AND` (`&`) of two flags.
    #[inline]
    pub fn intersection(&self, other: Self) -> Self {
        BitFlags32(self.0 & other.0)
    }
    /// Returns the bits set in `self` that are _not_ set in `other`.
    #[inline]
    pub fn difference(&self, other: Self) -> Self {
        BitFlags32(self.0 & !other.0)
    }
    /// Returns the bits set in `self` or `other`, but _not_ both, using bitwise `XOR` (`^`).
    #[inline]
    pub fn symmetric_difference(&self, other: Self) -> Self {
        BitFlags32(self.0 ^ other.0)
    }
    /// Returns the bitwise `OR` (`|`) of two flags.
    #[inline]
    pub fn union(&self, other: Self) -> Self {
        BitFlags32(self.0 | other.0)
    }
    /// Returns the bitwise negation (`!`) of given flags.
    #[inline]
    pub fn complement(&self) -> Self {
        BitFlags32(!self.0)
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
    /// Sets bit at given index (0-31).
    ///
    /// __Panics__ if `index > 31`.
    #[inline]
    pub fn insert_at_index(&mut self, index: usize) {
        assert!(index < 32, "BitFlags32 structs are indexed from 0 to 31");
        self.0 = self.0 | 2_u32.pow(index as u32);
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
    /// Sets bit at given index (0-31) to specific value (`true` = `1`; `false` = `0`).
    ///
    /// __Panics__ if `index > 31`.
    #[inline]
    pub fn set_at_index(&mut self, index: usize, value: bool) {
        assert!(index < 32, "BitFlags32 structs are indexed from 0 to 31");
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
    /// Toggles bit at given index (0-31).
    ///
    /// __Panics__ if `index > 31`.
    #[inline]
    pub fn toggle_at_index(&mut self, index: usize) {
        assert!(index < 32, "BitFlags32 structs are indexed from 0 to 31");
        self.0 = self.0 ^ 2_u32.pow(index as u32);
    }
    /// Unsets flags that match those of incoming `BitFlags32` (bitwise `AND NOT`).
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 = self.0 & !other.0;
    }
    /// Unsets bit at given index (0-31).
    ///
    /// __Panics__ if `index > 31`.
    #[inline]
    pub fn remove_at_index(&mut self, index: usize) {
        assert!(index < 32, "BitFlags32 structs are indexed from 0 to 31");
        self.0 = self.0 & !2_u32.pow(index as u32);
    }
    /// Returns the number of bits.
    #[inline]
    pub fn num_bits() -> usize {
        32
    }
    /// Returns value of bit at given index (`0` is `false`; `1` is `true`).
    ///
    /// __Panics__ if `index > 31`.
    #[inline]
    pub fn bit_at_index(&self, index: usize) -> bool {
        assert!(index < 32, "BitFlags32 structs are indexed from 0 to 31");
        self.0 & 2_u32.pow(index as u32) > 0
    }
    /// Returns value of bit at given index (`0` is `false`; `1` is `true`). Returns `None` if out
    /// of bounds.
    #[inline]
    pub fn get_bit_at_index(&self, index: usize) -> Option<bool> {
        if index < 32 {
            return Some((self.0 & 2_u32.pow(index as u32)) > 0);
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
        n -= n >> 1;
        BitFlags32(n)
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
    pub fn iter(&self) -> BitFlagsIter32 {
        BitFlagsIter32 { current_bit: 0, bits: self.0 }
    }
}

impl From<u32> for BitFlags32 {
    fn from(value: u32) -> Self {
        BitFlags32(value)
    }
}

impl TryFrom<usize> for BitFlags32 {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 32 {
            Ok(Self(2_u32.pow(value as u32)))
        } else {
            Err("BitFlags32 structs are indexed from 0 to 31")
        }
    }
}

impl core::fmt::Display for BitFlags32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BitFlags32({})", self.0)
    }
}

impl core::fmt::Binary for BitFlags32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#034b}", self.0)
    }
}

impl core::ops::BitOr<BitFlags32> for BitFlags32 {
    type Output = BitFlags32;

    fn bitor(self, rhs: BitFlags32) -> Self::Output {
        BitFlags32(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<BitFlags32> for BitFlags32 {
    fn bitor_assign(&mut self, rhs: BitFlags32) {
        self.0 |= rhs.0
    }
}

impl core::ops::BitAnd<BitFlags32> for BitFlags32 {
    type Output = BitFlags32;

    fn bitand(self, rhs: BitFlags32) -> Self::Output {
        BitFlags32(self.0 & rhs.0)
    }
}

impl core::ops::BitAndAssign<BitFlags32> for BitFlags32 {
    fn bitand_assign(&mut self, rhs: BitFlags32) {
        self.0 &= rhs.0
    }
}

impl core::ops::BitXor<BitFlags32> for BitFlags32 {
    type Output = BitFlags32;

    fn bitxor(self, rhs: BitFlags32) -> Self::Output {
        BitFlags32(self.0 ^ rhs.0)
    }
}

impl core::ops::BitXorAssign<BitFlags32> for BitFlags32 {
    fn bitxor_assign(&mut self, rhs: BitFlags32) {
        self.0 ^= rhs.0
    }
}

impl core::ops::Not for BitFlags32 {
    type Output = BitFlags32;

    /// Toggles _all_ bits.
    fn not(self) -> Self::Output {
        BitFlags32(!self.0)
    }
}

/// Iterator over set bits of a `BitFlags32`.
pub struct BitFlagsIter32 {
    current_bit: usize,
    bits:        u32,
}

impl core::iter::Iterator for BitFlagsIter32 {
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
    use super::BitFlags32;
    use serde::{Deserialize, Serialize};

    impl<'de> Deserialize<'de> for BitFlags32 {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<BitFlags32, D::Error> {
            let val = u32::deserialize(d)?;
            Ok(BitFlags32(val))
        }
    }

    impl Serialize for BitFlags32 {
        fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            u32::serialize(&self.0, s)
        }
    }
}

#[cfg(feature = "nanoserde-support")]
mod impl_nanoserde {
    extern crate std;
    use super::BitFlags32;
    use nanoserde::{
        DeBin, DeBinErr, DeJson, DeJsonErr, DeJsonState, DeRon, DeRonErr, DeRonState, SerBin,
        SerJson, SerJsonState, SerRon, SerRonState,
    };
    use std::prelude::v1::*;

    impl DeBin for BitFlags32 {
        fn de_bin(offset: &mut usize, bytes: &[u8]) -> Result<Self, DeBinErr> {
            let l = core::mem::size_of::<u32>();
            if *offset + l > bytes.len() {
                return Err(DeBinErr { o: *offset, l: l, s: bytes.len() });
            }
            let val = u32::from_le_bytes(bytes[*offset..(*offset + l)].try_into().unwrap());
            *offset += l;

            Ok(BitFlags32(val))
        }
    }

    impl SerBin for BitFlags32 {
        fn ser_bin(&self, output: &mut Vec<u8>) {
            let bytes = self.0.to_le_bytes();
            output.extend_from_slice(&bytes);
        }
    }

    impl DeJson for BitFlags32 {
        fn de_json(
            state: &mut DeJsonState,
            input: &mut core::str::Chars,
        ) -> Result<Self, DeJsonErr> {
            let val = state.u64_range(u32::MAX as u64)?;
            state.next_tok(input)?;

            Ok(BitFlags32(val as u32))
        }
    }

    impl SerJson for BitFlags32 {
        fn ser_json(&self, _indent_level: usize, state: &mut SerJsonState) {
            state.out.push_str(&self.0.to_string())
        }
    }

    impl DeRon for BitFlags32 {
        fn de_ron(state: &mut DeRonState, input: &mut core::str::Chars) -> Result<Self, DeRonErr> {
            let val = state.u64_range(u32::MAX as u64)?;
            state.next_tok(input)?;

            Ok(BitFlags32(val as u32))
        }
    }

    impl SerRon for BitFlags32 {
        fn ser_ron(&self, _indent_level: usize, state: &mut SerRonState) {
            state.out.push_str(&self.0.to_string())
        }
    }
}
