//! BitFlags with a `u16` representation.

use core::convert::TryFrom;

/// 16-bit bitflags, indexed from bit indexes `[0]` to `[15]`.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BitFlags16(pub u16);

impl BitFlags16 {
    /// Returns new (empty) instance.
    pub fn new() -> Self {
        Self::empty()
    }
    /// Returns empty `BitFlags16` (with value of 0).
    #[inline]
    pub fn empty() -> Self {
        Self(0)
    }
    /// Returns full `BitFlags16` (with value of u16::MAX).
    #[inline]
    pub fn all() -> Self {
        Self(u16::MAX)
    }
    /// Returns new instance using specified bits.
    #[inline]
    pub fn from_bits(val: u16) -> Self {
        Self(val)
    }
    /// Converts an index into a BitFlag. 16 indexes allowed (0-15).
    #[inline]
    pub fn from_index(index: usize) -> Self {
        Self::try_from(index).unwrap()
    }
    /// Converts a slice of indexes into a BitFlag. 16 indexes allowed (0-15).
    #[inline]
    pub fn from_slice(s: &[usize]) -> Self {
        let mut bits = Self(0);

        for index in s.iter() {
            let temp_bits = Self::from_index(*index);
            bits.insert(temp_bits);
        }

        bits
    }
    /// Returns true if no flags are set.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    /// Returns true if all flags are set.
    #[inline]
    pub fn is_all(&self) -> bool {
        self.0 == u16::MAX
    }
    /// Returns true if current flags contain _at least one_ of the incoming flags.
    #[inline]
    pub fn intersects(&self, other: Self) -> bool {
        (self.0 & other.0) > 0
    }
    /// Bitwise `AND` (`&`) of two flags.
    #[inline]
    pub fn intersection(&self, other: Self) -> BitFlags16 {
        BitFlags16(self.0 & other.0)
    }
    /// Bitwise `OR` (`|`) of two flags.
    #[inline]
    pub fn union(&self, other: Self) -> BitFlags16 {
        BitFlags16(self.0 | other.0)
    }    
    /// Returns true if current flags contain _all_ incoming flags.
    #[inline]
    pub fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    /// Inserts flags into current `BitFlags16` (bitwise OR).
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 = self.0 | other.0;
    }
    /// Inserts flag at given index to specific value (true or false; 0 or 1).
    /// Only 16 indexes allowed (0-15).
    #[inline]
    pub fn insert_at_index(&mut self, index: usize) {
        assert!(index < 16, "up to 16 unique flags allowed for BitFlags16");
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
    /// Sets flag at given index to specific value (true or false; 0 or 1).
    /// Only 16 indexes allowed (0-15).
    #[inline]
    pub fn set_at_index(&mut self, index: usize, value: bool) {
        assert!(index < 16, "up to 16 unique flags allowed for BitFlags16");
        if value {
            self.insert_at_index(index);
        } else {
            self.remove_at_index(index);
        }
    }
    /// Toggles current flags based on incoming `BitFlags16` (bitwise XOR).
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 = self.0 ^ other.0;
    }
    /// Toggles flag at given index.Only 16 indexes allowed (0-15).
    #[inline]
    pub fn toggle_at_index(&mut self, index: usize) {
        assert!(index < 16, "up to 16 unique flags allowed for BitFlags16");
        self.0 = self.0 ^ 2_u16.pow(index as u32);
    }
    /// Removes current flags that match those of incoming `BitFlags16` (bitwise AND NOT).
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 = self.0 & !other.0;
    }
    /// Removes flag at given index.  Only 16 indexes allowed (0-15).
    #[inline]
    pub fn remove_at_index(&mut self, index: usize) {
        assert!(index < 16, "up to 16 unique flags allowed for BitFlags16");
        self.0 = self.0 & !2_u16.pow(index as u32);
    }
    /// Returns the bits (internal value).
    #[inline]
    pub fn bits(&self) -> u16 {
        self.0
    }
    /// Returns the number of bits.
    #[inline]
    pub fn num_bits() -> usize {
        16
    }
    /// Returns value of bit at given index (0 is false; 1 is true).
    ///
    /// Indexes (0-15) allowed. Will panic if index is out of bounds.
    #[inline]
    pub fn bit_at_index(&self, index: usize) -> bool {
        assert!(index < 16, "up to 16 unique flags allowed for BitFlags16");
        self.0 & 2_u16.pow(index as u32) > 0
    }
    /// Returns value of bit at given index (0 is false; 1 is true).  Returns None if out
    /// of bounds.  For cases not meant to fail, index directly with core::ops::Index.
    #[inline]
    pub fn get_bit_at_index(&self, index: usize) -> Option<bool> {
        if index < 16 {
            return Some((self.0 & 2_u16.pow(index as u32)) > 0);
        }
        None
    }
    /// Returns the value of the highest set bit (`1`) of the bitflag.  If None -> `0`.
    #[inline]
    pub fn highest_set_bit(&self) -> u16 {
        let mut n = self.0.clone();
        n |= n >> 1;
        n |= n >> 2;
        n |= n >> 4;
        n |= n >> 8;
        n - (n >> 1)
    }
    /// Returns the index of the highest set bit (`1`) of the bitflag, if present
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
    /// Iterates over set bits of the structure.  Returns `Some(bit_index)` if the
    /// bit is 1, otherwise `None`.  E.g. collecting `0b00001001` into a vector would
    /// produce `vec![0, 3]`, representing the 0th and 3rd indexes.
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
            Err("BitFlags16 allows indexes of 0-15 only")
        }
    }
}

impl TryFrom<usize> for BitFlags16 {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 16 {
            Ok(Self(2_u16.pow(value as u32)))
        } else {
            Err("BitFlags16 allows indexes of 0-15 only")
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

impl core::ops::BitAnd<BitFlags16> for BitFlags16 {
    type Output = BitFlags16;

    fn bitand(self, rhs: BitFlags16) -> Self::Output {
        BitFlags16(self.0 & rhs.0)
    }
}

impl core::ops::BitXor<BitFlags16> for BitFlags16 {
    type Output = BitFlags16;

    fn bitxor(self, rhs: BitFlags16) -> Self::Output {
        BitFlags16(self.0 ^ rhs.0)
    }
}

impl core::ops::Not for BitFlags16 {
    type Output = BitFlags16;

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
