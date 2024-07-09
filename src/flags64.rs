//! BitFlags with a `u64` representation.

use std::convert::TryFrom;

/// 64-bit bitflags, indexed from bit indexes `[0]` to `[63]`.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BitFlags64(pub u64);

impl BitFlags64 {
    /// Returns new (empty) instance.
    pub fn new() -> Self {
        Self::empty()
    }
    /// Returns empty `BitFlags64` (with value of 0).
    #[inline]
    pub fn empty() -> Self {
        Self(0)
    }
    /// Returns full `BitFlags64` (with value of u64::MAX).
    #[inline]
    pub fn all() -> Self {
        Self(u64::MAX)
    }
    /// Returns new instance using specified bits.
    #[inline]
    pub fn from_bits(val: u64) -> Self {
        Self(val)
    }
    /// Converts an index into a BitFlag. 64 indexes allowed (0-63). `usize` conversion 
    /// to `u32` is fine as no bitflag struct has more than `u32::MAX` bits.
    #[inline]
    pub fn from_index(index: usize) -> Self {
        Self::try_from(index).unwrap()
    }
    /// Converts a slice of indexes into a BitFlag. 64 indexes allowed (0-63).
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
        self.0 == u64::MAX
    }      
    /// Returns true if current flags contain _at least one_ of the incoming flags.
    #[inline]
    pub fn intersects(&self, other: BitFlags64) -> bool {
        (self.0 & other.0) > 0
    }
    /// Returns true if current flags contain _all_ incoming flags.
    #[inline]
    pub fn contains(&self, other: BitFlags64) -> bool {
        (self.0 & other.0) == other.0
    }
    /// Inserts flags into current `BitFlags64` (bitwise OR).
    #[inline]
    pub fn insert(&mut self, other: BitFlags64) {
        self.0 = self.0 | other.0;
    }
    /// Inserts flag at given index to specific value (true or false; 0 or 1).
    /// Only 64 indexes allowed (0-63).
    #[inline]
    pub fn insert_at_index(&mut self, index: usize) {
        assert!(index < 64, "up to 64 unique tags allowed per category for BitFlags64");
        self.0 = self.0 | 2_u64.pow(index as u32);
    }    
    /// Inserts or removes the specified flags depending on the passed value.
    #[inline]
    pub fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }      
    /// Sets flag at given index to specific value (true or false; 0 or 1).
    /// Only 64 indexes allowed (0-63).
    #[inline]
    pub fn set_at_index(&mut self, index: usize, value: bool) {
        assert!(index < 64, "up to 64 unique tags allowed per category for BitFlags64");
        if value {
            self.insert_at_index(index);
        } else {
            self.remove_at_index(index);
        }        
    }       
    /// Toggles current flags based on incoming `BitFlags64` (bitwise XOR).
    #[inline]
    pub fn toggle(&mut self, other: BitFlags64) {
        self.0 = self.0 ^ other.0;
    }
    /// Toggles flag at given index.  Only 64 indexes allowed (0-63).
    #[inline]
    pub fn toggle_at_index(&mut self, index: usize) {
        assert!(index < 64, "up to 64 unique tags allowed per category for BitFlags64");
        self.0 = self.0 ^ 2_u64.pow(index as u32);
    }       
    /// Removes current flags that match those of incoming `BitFlags64` (bitwise AND NOT).
    #[inline]
    pub fn remove(&mut self, other: BitFlags64) {
        self.0 = self.0 & !other.0;
    }
    /// Removes flag at given index.  Only 64 indexes allowed (0-63).
    #[inline]
    pub fn remove_at_index(&mut self, index: usize) {
        assert!(index < 64, "up to 64 unique tags allowed per category for BitFlags64");
        self.0 = self.0 & !2_u64.pow(index as u32);
    }         
    /// Returns the bits (internal value).
    #[inline]
    pub fn bits(&self) -> u64 {
        self.0
    }
    /// Returns the number of bits.
    #[inline]
    pub fn num_bits() -> usize {
        64
    }       
    /// Returns value of bit at given index (0 is false; 1 is true). 
    /// Only 64 indexes allowed (0-63) - panics if out of bounds.
    #[inline]
    pub fn bit_at_index(&self, index: usize) -> bool {
        assert!(index < 64, "up to 64 unique flags allowed for BitFlags64");
        self.0 & 2_u64.pow(index as u32) > 0
    }        
    /// Returns value of bit at given index (0 is false; 1 is true).  Returns None if out
    /// of bounds.  For cases not meant to fail, index directly with std::ops::Index.
    #[inline]
    pub fn get_bit_at_index(&self, index: usize) -> Option<bool> {
        if index < 64 {
            return Some((self.0 & 2_u64.pow(index as u32)) > 0);
        } 
        None
    }
    /// Returns the value of the highest set bit (`1`) of the bitflag.  If None -> `0`.
    #[inline]
    pub fn highest_set_bit(&self) -> u64 {
        let mut n = self.0.clone();
        n |= n >>  1;
        n |= n >>  2;
        n |= n >>  4;
        n |= n >>  8;
        n |= n >>  16;
        n |= n >>  32;
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
    pub fn iter(&self) -> BitFlagsIter64 {
        BitFlagsIter64 { current_bit: 0, bits: self.0 }
    }
}

impl From<u64> for BitFlags64 {
    fn from(value: u64) -> BitFlags64 {
        BitFlags64(value)
    }
}

impl TryFrom<u32> for BitFlags64 {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 64 {
            Ok(Self(2_u64.pow(value)))
        } else {
            Err("Only values 0-63 allowed to make a BitFlags64")
        }        
    }
}

impl TryFrom<usize> for BitFlags64 {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 64 {
            Ok(Self(2_u64.pow(value as u32)))
        } else {
            Err("Only values 0-63 allowed to make a BitFlags64")
        }                
    }
}

impl std::fmt::Display for BitFlags64{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BitFlags64({})", self.0)
    }
}

impl std::fmt::Binary for BitFlags64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#066b}", self.0)
    }
}

/// Iterator over set bits of a `BitFlags64`.
pub struct BitFlagsIter64 {
    current_bit: usize,
    bits: u64
}

impl std::iter::Iterator for BitFlagsIter64 {
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
