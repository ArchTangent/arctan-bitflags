//! BitFlags with a `u16` representation.

use core::convert::TryFrom;

/// 16-bit bitflags, indexed from bit indexes `[0]` to `[15]`.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitFlags16(pub u16);

impl BitFlags16 {
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
        Self(u16::MAX)
    }
    /// Returns a new instance with bits in range `(start..=end)` set.
    ///
    /// __Panics__ if `start > 15`, `end > 15`, or `start > end`.
    #[inline]
    pub fn with_set_bit_range(start: u8, end: u8) -> Self {
        assert!(
            start < 16 && end < 16 && start <= end,
            "BitFlags8 are indexed from 0 to 15, and start must be <= end"
        );

        let e = 1 << end;
        BitFlags16(e | e - (1 << start))
    }
    /// Returns a new instance from a `u16`.
    #[inline]
    pub fn from_u16(val: u16) -> Self {
        Self(val)
    }
    /// Returns the underlying `u16` value.
    #[inline]
    pub fn to_u16(&self) -> u16 {
        self.0
    }
    /// Converts an index (0-15) into a `BitFlags16`.
    ///
    /// __Panics__ if `index > 15`.
    pub fn from_index(index: usize) -> Self {
        Self::try_from(index).unwrap()
    }
    /// Converts a slice of indexes (0-15) into a `BitFlags16`.
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
        self.0 == u16::MAX
    }
    /// Returns `true` if `self` and `other` have _at least one_ matching set bit.
    #[inline]
    pub fn intersects(&self, other: Self) -> bool {
        (self.0 & other.0) > 0
    }
    /// Returns the bitwise `AND` (`&`) of two flags.
    #[inline]
    pub fn intersection(&self, other: Self) -> Self {
        BitFlags16(self.0 & other.0)
    }
    /// Returns the bits set in `self` that are _not_ set in `other`.
    #[inline]
    pub fn difference(&self, other: Self) -> Self {
        BitFlags16(self.0 & !other.0)
    }
    /// Returns the bits set in `self` or `other`, but _not_ both, using bitwise `XOR` (`^`).
    #[inline]
    pub fn symmetric_difference(&self, other: Self) -> Self {
        BitFlags16(self.0 ^ other.0)
    }
    /// Returns the bitwise `OR` (`|`) of two flags.
    #[inline]
    pub fn union(&self, other: Self) -> Self {
        BitFlags16(self.0 | other.0)
    }
    /// Returns the bitwise negation (`!`) of given flags.
    #[inline]
    pub fn complement(&self) -> Self {
        BitFlags16(!self.0)
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
    /// Sets bit at given index (0-15).
    ///
    /// __Panics__ if `index > 15`.
    #[inline]
    pub fn insert_at_index(&mut self, index: usize) {
        assert!(index < 16, "BitFlags16 are indexed from 0 to 15");
        self.0 = self.0 | 2_u16.pow(index as u32);
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
    /// Sets bit at given index (0-15) to specific value (`true` = `1`; `false` = `0`).
    ///
    /// __Panics__ if `index > 15`.
    #[inline]
    pub fn set_at_index(&mut self, index: usize, value: bool) {
        assert!(index < 16, "BitFlags16 are indexed from 0 to 15");
        if value {
            self.insert_at_index(index);
        } else {
            self.remove_at_index(index);
        }
    }
    /// Sets bits in range `(start..=end)`.
    ///
    /// __Panics__ if `start > 15`, `end > 15`, or `start > end`.
    #[inline]
    pub fn set_bit_range(&mut self, start: u8, end: u8) {
        assert!(
            start < 16 && end < 16 && start <= end,
            "BitFlags16 are indexed from 0 to 15, and start must be <= end"
        );

        let e = 1 << end;
        self.0 |= e | e - (1 << start);
    }
    /// Toggles bits based on mask (using bitwise `XOR`).
    #[inline]
    pub fn toggle(&mut self, mask: Self) {
        self.0 = self.0 ^ mask.0;
    }
    /// Toggles bit at given index (0-15).
    ///
    /// __Panics__ if `index > 15`.
    #[inline]
    pub fn toggle_at_index(&mut self, index: usize) {
        assert!(index < 16, "BitFlags16 are indexed from 0 to 15");
        self.0 = self.0 ^ 2_u16.pow(index as u32);
    }
    /// Unsets bits that match those of incoming `BitFlags16` (bitwise `AND NOT`).
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 = self.0 & !other.0;
    }
    /// Unsets bit at given index (0-15).
    ///
    /// __Panics__ if `index > 15`.
    #[inline]
    pub fn remove_at_index(&mut self, index: usize) {
        assert!(index < 16, "BitFlags16 are indexed from 0 to 15");
        self.0 = self.0 & !2_u16.pow(index as u32);
    }
    /// Returns the number of bits.
    #[inline]
    pub fn num_bits() -> usize {
        16
    }
    /// Returns value of bit at given index (`0` is `false`; `1` is `true`).
    ///
    /// __Panics__ if `index > 15`.
    #[inline]
    pub fn bit_at_index(&self, index: usize) -> bool {
        assert!(index < 16, "BitFlags16 are indexed from 0 to 15");
        self.0 & 2_u16.pow(index as u32) > 0
    }
    /// Returns value of bit at given index (`0` is `false`; `1` is `true`). Returns `None` if out
    /// of bounds.
    #[inline]
    pub fn get_bit_at_index(&self, index: usize) -> Option<bool> {
        if index < 16 {
            return Some((self.0 & 2_u16.pow(index as u32)) > 0);
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
        n -= n >> 1;
        BitFlags16(n)
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
    pub fn iter(&self) -> BitFlagsIter16 {
        BitFlagsIter16 { current_bit: 0, bits: self.0 }
    }
}

impl From<u16> for BitFlags16 {
    fn from(val: u16) -> Self {
        BitFlags16(val)
    }
}

impl TryFrom<u32> for BitFlags16 {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 16 {
            Ok(Self(2_u16.pow(value)))
        } else {
            Err("BitFlags16 are indexed from 0 to 15")
        }
    }
}

impl TryFrom<usize> for BitFlags16 {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 16 {
            Ok(Self(2_u16.pow(value as u32)))
        } else {
            Err("BitFlags16 are indexed from 0 to 15")
        }
    }
}

impl core::fmt::Display for BitFlags16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BitFlags16({})", self.0)
    }
}

impl core::fmt::Binary for BitFlags16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#018b}", self.0)
    }
}

impl core::ops::BitOr<BitFlags16> for BitFlags16 {
    type Output = BitFlags16;

    fn bitor(self, rhs: BitFlags16) -> Self::Output {
        BitFlags16(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<BitFlags16> for BitFlags16 {
    fn bitor_assign(&mut self, rhs: BitFlags16) {
        self.0 |= rhs.0
    }
}

impl core::ops::BitAnd<BitFlags16> for BitFlags16 {
    type Output = BitFlags16;

    fn bitand(self, rhs: BitFlags16) -> Self::Output {
        BitFlags16(self.0 & rhs.0)
    }
}

impl core::ops::BitAndAssign<BitFlags16> for BitFlags16 {
    fn bitand_assign(&mut self, rhs: BitFlags16) {
        self.0 &= rhs.0
    }
}

impl core::ops::BitXor<BitFlags16> for BitFlags16 {
    type Output = BitFlags16;

    fn bitxor(self, rhs: BitFlags16) -> Self::Output {
        BitFlags16(self.0 ^ rhs.0)
    }
}

impl core::ops::BitXorAssign<BitFlags16> for BitFlags16 {
    fn bitxor_assign(&mut self, rhs: BitFlags16) {
        self.0 ^= rhs.0
    }
}

impl core::ops::Not for BitFlags16 {
    type Output = BitFlags16;

    /// Toggles _all_ bits.
    fn not(self) -> Self::Output {
        BitFlags16(!self.0)
    }
}

/// Iterator over set bits of a `BitFlags16`.
pub struct BitFlagsIter16 {
    current_bit: usize,
    bits:        u16,
}

impl core::iter::Iterator for BitFlagsIter16 {
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
    use super::BitFlags16;
    use serde::{Deserialize, Serialize};

    impl<'de> Deserialize<'de> for BitFlags16 {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<BitFlags16, D::Error> {
            let val = u16::deserialize(d)?;
            Ok(BitFlags16(val))
        }
    }

    impl Serialize for BitFlags16 {
        fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            u16::serialize(&self.0, s)
        }
    }
}

#[cfg(feature = "nanoserde-support")]
mod impl_nanoserde {
    extern crate std;
    use super::BitFlags16;
    use nanoserde::{
        DeBin, DeBinErr, DeJson, DeJsonErr, DeJsonState, DeRon, DeRonErr, DeRonState, SerBin,
        SerJson, SerJsonState, SerRon, SerRonState,
    };
    use std::prelude::v1::*;

    impl DeBin for BitFlags16 {
        fn de_bin(offset: &mut usize, bytes: &[u8]) -> Result<Self, DeBinErr> {
            let l = core::mem::size_of::<u16>();
            if *offset + l > bytes.len() {
                return Err(DeBinErr { o: *offset, l: l, s: bytes.len() });
            }
            let val = u16::from_le_bytes(bytes[*offset..(*offset + l)].try_into().unwrap());
            *offset += l;

            Ok(BitFlags16(val))
        }
    }

    impl SerBin for BitFlags16 {
        fn ser_bin(&self, output: &mut Vec<u8>) {
            let bytes = self.0.to_le_bytes();
            output.extend_from_slice(&bytes);
        }
    }

    impl DeJson for BitFlags16 {
        fn de_json(
            state: &mut DeJsonState,
            input: &mut core::str::Chars,
        ) -> Result<Self, DeJsonErr> {
            let val = state.u64_range(u16::MAX as u64)?;
            state.next_tok(input)?;

            Ok(BitFlags16(val as u16))
        }
    }

    impl SerJson for BitFlags16 {
        fn ser_json(&self, _indent_level: usize, state: &mut SerJsonState) {
            state.out.push_str(&self.0.to_string())
        }
    }

    impl DeRon for BitFlags16 {
        fn de_ron(state: &mut DeRonState, input: &mut core::str::Chars) -> Result<Self, DeRonErr> {
            let val = state.u64_range(u16::MAX as u64)?;
            state.next_tok(input)?;

            Ok(BitFlags16(val as u16))
        }
    }

    impl SerRon for BitFlags16 {
        fn ser_ron(&self, _indent_level: usize, state: &mut SerRonState) {
            state.out.push_str(&self.0.to_string())
        }
    }
}
