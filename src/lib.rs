//! BitFlags with integer (`u8`, `u16`, `u32`, `u64`, `u128`) representation.
//!
//! For more information on the logic involved, refer to the useful wiki on [Set Theory](https://en.wikipedia.org/wiki/Set_theory).

#[cfg(test)]
mod tests;

mod flags128;
mod flags16;
mod flags32;
mod flags64;
mod flags8;

pub use flags128::*;
pub use flags16::*;
pub use flags32::*;
pub use flags64::*;
pub use flags8::*;
