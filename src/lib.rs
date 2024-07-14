//! BitFlags with integer (`u8`, `u16`, `u32`, `u64`, `u128`) representation.
//!
//! For more information on the logic involved, refer to the useful wiki on [Set Theory](https://en.wikipedia.org/wiki/Set_theory).
#![no_std]

mod flags_8;
mod flags_16;
mod flags_32;
mod flags_64;
mod flags_128;

pub use flags_8::*;
pub use flags_16::*;
pub use flags_32::*;
pub use flags_64::*;
pub use flags_128::*;
