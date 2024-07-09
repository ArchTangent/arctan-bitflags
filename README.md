# arctan-bitflags
Rust bitflags structures with 8, 16, 32, 64, and 128-bit representation.

## Primary Types

- `BitFlags8`: 8-bit flag that freely converts to and from `u8`.
- `BitFlags16`: 8-bit flag that freely converts to and from `u16`.
- `BitFlags32`: 8-bit flag that freely converts to and from `u32`.
- `BitFlags64`: 8-bit flag that freely converts to and from `u64`.
- `BitFlags128`: 8-bit flag that freely converts to and from `u128`.

## Goals

The primary purpose for creation of this library is for *games*. I wanted a bitflag type that met the following criteria:

1. Specific: a type that is explicitly designated for use as a bitflag. While the primitive integer types (`u8`, `u16`, `u32`, `u64`, `u128`) use

2. Flexible: ease of conversion to and from the relevant primitive type.

3. Intuitive: exposes an interface one would reasonably expect bitflags to have.
