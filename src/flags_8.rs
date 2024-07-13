//! BitFlags with a `u8` representation.

use core::convert::TryFrom;

/// 8-bit bitflags, indexed from bit indexes `[0]` to `[7]`.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BitFlags8(pub u8);

impl BitFlags8 {
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
        Self(u8::MAX)
    }
    /// Returns a new instance from a `u8`.
    #[inline]
    pub fn from_u8(val: u8) -> Self {
        Self(val)
    }
    /// Returns the underlying `u8` value.
    #[inline]
    pub fn to_u8(&self) -> u8 {
        self.0
    }
    /// Converts an index (0-7) into a `BitFlags8`.
    /// 
    /// __Panics__ if `index > 7`.
    #[inline]
    pub fn from_index(index: usize) -> Self {
        Self::try_from(index).unwrap()
    }
    /// Converts a slice of indexes (0-7) into a `BitFlags8`.
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
        self.0 == u8::MAX
    }
    /// Returns `true` if `self` and `other` have _at least one_ matching set bit.
    #[inline]
    pub fn intersects(&self, other: Self) -> bool {
        (self.0 & other.0) > 0
    }
    /// Returns the bitwise `AND` (`&`) of two flags.
    #[inline]
    pub fn intersection(&self, other: Self) -> BitFlags8 {
        BitFlags8(self.0 & other.0)
    }
    /// Returns the bits set in `self` that are _not_ set in `other`.
    #[inline]
    pub fn difference(&self, other: Self) -> BitFlags8 {
        BitFlags8(self.0 & !other.0)
    }
    /// Returns the bits set in `self` or `other`, but _not_ both, using bitwise `XOR` (`^`).
    #[inline]
    pub fn symmetric_difference(&self, other: Self) -> BitFlags8 {
        BitFlags8(self.0 ^ other.0)
    }
    /// Returns the bitwise `OR` (`|`) of two flags.
    #[inline]
    pub fn union(&self, other: Self) -> BitFlags8 {
        BitFlags8(self.0 | other.0)
    }
    /// Returns the bitwise negation (`!`) of given flags.
    #[inline]
    pub fn complement(&self) -> BitFlags8 {
        BitFlags8(!self.0)
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
    /// Sets bit at given index (0-7).
    ///
    /// __Panics__ if `index > 7`.
    #[inline]
    pub fn insert_at_index(&mut self, index: usize) {
        assert!(index < 8, "BitFlags8 structs are indexed from 0 to 7");
        self.0 = self.0 | 2_u8.pow(index as u32);
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
    /// Sets bit at given index (0-7) to specific value (`true` = `1`; `false` = `0`).
    ///
    /// __Panics__ if `index > 7`.
    #[inline]
    pub fn set_at_index(&mut self, index: usize, value: bool) {
        assert!(index < 8, "BitFlags8 structs are indexed from 0 to 7");
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
    /// Toggles bit at given index.
    ///
    /// __Panics__ if `index > 7`.
    #[inline]
    pub fn toggle_at_index(&mut self, index: usize) {
        assert!(index < 8, "BitFlags8 structs are indexed from 0 to 7");
        self.0 = self.0 ^ 2_u8.pow(index as u32);
    }
    /// Unsets bits that match those of incoming `BitFlags8` (bitwise `AND NOT`).
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 = self.0 & !other.0;
    }
    /// Unsets bit at given index (0-7).
    /// 
    /// __Panics__ if `index > 7`.
    #[inline]
    pub fn remove_at_index(&mut self, index: usize) {
        assert!(index < 8, "BitFlags8 structs are indexed from 0 to 7");
        self.0 = self.0 & !2_u8.pow(index as u32);
    }
    /// Returns the bits (internal value).
    #[inline]
    pub fn bits(&self) -> u8 {
        self.0
    }
    /// Returns the number of bits.
    #[inline]
    pub fn num_bits() -> usize {
        8
    }
    /// Returns value of bit at given index (`0` is `false`; `1` is `true`).
    ///
    /// __Panics__ if `index > 7`.
    #[inline]
    pub fn bit_at_index(&self, index: usize) -> bool {
        assert!(index < 8, "BitFlags8 structs are indexed from 0 to 7");
        self.0 & 2_u8.pow(index as u32) > 0
    }
    /// Returns value of bit at given index (`0` is `false`; `1` is `true`). Returns `None` if out
    /// of bounds.
    #[inline]
    pub fn get_bit_at_index(&self, index: usize) -> Option<bool> {
        if index < 8 {
            return Some((self.0 & 2_u8.pow(index as u32)) > 0);
        }
        None
    }
    /// Returns the value of the highest set bit of the bitflag. If `None` -> `0`.
    #[inline]
    pub fn highest_set_bit(&self) -> u8 {
        let mut n = self.0.clone();
        n |= n >> 1;
        n |= n >> 2;
        n |= n >> 4;
        n - (n >> 1)
    }
    /// Returns the index of highest set bit of the bitflag, if present.
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
    pub fn iter(&self) -> BitFlagsIter8 {
        BitFlagsIter8 { current_bit: 0, bits: self.0 }
    }
}

impl From<u8> for BitFlags8 {
    fn from(value: u8) -> BitFlags8 {
        BitFlags8(value)
    }
}

impl TryFrom<u32> for BitFlags8 {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 8 {
            Ok(Self(2_u8.pow(value)))
        } else {
            Err("BitFlags8 structs are indexed from 0 to 7")
        }
    }
}

impl TryFrom<usize> for BitFlags8 {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 8 {
            Ok(Self(2_u8.pow(value as u32)))
        } else {
            Err("BitFlags8 structs are indexed from 0 to 7")
        }
    }
}

impl core::fmt::Display for BitFlags8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BitFlags8({})", self.0)
    }
}

impl core::fmt::Binary for BitFlags8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#010b}", self.0)
    }
}

impl core::ops::BitOr<BitFlags8> for BitFlags8 {
    type Output = BitFlags8;

    fn bitor(self, rhs: BitFlags8) -> Self::Output {
        BitFlags8(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<BitFlags8> for BitFlags8 {
    fn bitor_assign(&mut self, rhs: BitFlags8) {
        self.0 |= rhs.0
    }
}

impl core::ops::BitAnd<BitFlags8> for BitFlags8 {
    type Output = BitFlags8;

    fn bitand(self, rhs: BitFlags8) -> Self::Output {
        BitFlags8(self.0 & rhs.0)
    }
}

impl core::ops::BitAndAssign<BitFlags8> for BitFlags8 {
    fn bitand_assign(&mut self, rhs: BitFlags8) {
        self.0 &= rhs.0
    }
}

impl core::ops::BitXor<BitFlags8> for BitFlags8 {
    type Output = BitFlags8;

    fn bitxor(self, rhs: BitFlags8) -> Self::Output {
        BitFlags8(self.0 ^ rhs.0)
    }
}

impl core::ops::BitXorAssign<BitFlags8> for BitFlags8 {
    fn bitxor_assign(&mut self, rhs: BitFlags8) {
        self.0 ^= rhs.0
    }
}

impl core::ops::Not for BitFlags8 {
    type Output = BitFlags8;

    /// Toggles _all_ bits.
    fn not(self) -> Self::Output {
        BitFlags8(!self.0)
    }
}

/// Iterator over set bits of a `BitFlags8`.
pub struct BitFlagsIter8 {
    current_bit: usize,
    bits:        u8,
}

impl core::iter::Iterator for BitFlagsIter8 {
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
