//! BitFlags with integer (`u8`, `u16`, `u32`, `u64`, `u128`) representation.

#[cfg(test)]
mod tests;

mod flags8;
mod flags16;
mod flags32;
mod flags64;
mod flags128;

pub use flags8::*;
pub use flags16::*;
pub use flags32::*;
pub use flags64::*;
pub use flags128::*;
