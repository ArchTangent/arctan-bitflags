# arctan-bitflags
Bitflag structures with 8, 16, 32, 64, and 128-bit representation.

## Primary Types

- `BitFlags8`:   8-bit flags that convert to and from `u8`.
- `BitFlags16`:  16-bit flags that convert to and from `u16`.
- `BitFlags32`:  32-bit flags that convert to and from `u32`.
- `BitFlags64`:  64-bit flags that convert to and from `u64`.
- `BitFlags128`: 128-bit flags that convert to and from `u128`.

## Usage

### `Cargo.toml`

```toml
[dependencies]
arctan-bitflags = { version = "1.0" }
```

With `serde-support` feature (requires `serde`):

```toml
[dependencies]
arctan-bitflags = { version = "1.0", features = ["serde-support"] }
```

For RON, (de)serializing `BitFlags128` requires the `ron` crate's `"integer128"` feature.

With `nanoserde-support` feature (requires `nanoserde` and `std`):

```toml
[dependencies]
arctan-bitflags = { version = "1.0", features = ["nanoserde-support"] }
```

For JSON and RON, (de)serialization of up to 64-bit values (`BitFlags64`) is supported.

For BIN, (de)serialization of 128-bit values (`BitFlags128`) is supported.

### Creation

A new empty instance:
```rust
let flags = BitFlags8::new();

let flags = BitFlags8::empty();
```

A new instance with all bits set:
```rust
let flags = BitFlags8::full();

let flags = BitFlags8::from_u8(u8::MAX);

let flags = BitFlags8::from(u8::MAX);
```

A new instance from corresponding integer (all below are identical):
```rust
let flags = BitFlags8(0b0110);

let flags = BitFlags8::from_u8(0b0110);

let flags = BitFlags8::from(6u8);

let flags: BitFlags8 = 6u8.into();
```

A new instance from a bit index:
```rust
let flags = BitFlags8::from_index(2);
```

A new instance from a slice of indexes:
```rust
let flags = BitFlags8::from_slice(&[2, 4, 5]);
```

### Bit Manipulation

Operations:
```rust
let flags1 = BitFlags8(0b1001);
let flags2 = BitFlags8(0b0001);

assert_eq!(flags1 | flags2, BitFlags8(0b1001)); // BitOr
assert_eq!(flags1 & flags2, BitFlags8(0b0000)); // BitAnd
assert_eq!(flags1 ^ flags2, BitFlags8(0b0011)); // BitXor
assert_eq!(!flags1, BitFlags8(0b1111_1110)); // Not
```

Insertion:
```rust
// Final result: `0b1101`
let mut flags1 = BitFlags8(0b0001);
let flags2 = BitFlags8(0b1001);
flags1.insert(flags2);
flags1.insert_at_index(2);
```

Intersection:
```rust
let flags1 = BitFlags8(0b1001);
let flags2 = BitFlags8(0b0001);
let flags3 = BitFlags8(0b1111);

assert!(flags2.intersects(flags1));
assert!(flags3.contains(flags1));
assert_eq!(flags1.intersection(flags2), BitFlags8(0b0001));
```

Union:
```rust
let flags1 = BitFlags8(0b1001);
let flags2 = BitFlags8(0b0001);

assert_eq!(flags1.union(flags2), BitFlags8(0b1001));
```

For more, see the documentation.

## Goals

The primary purpose for creation of this library is for *games*. I wanted a bitflag type that met the following criteria:

1. Specific: a type that is explicitly designated for use as a bitflag. While the primitive integer types (`u8`, `u16`, `u32`, `u64`, `u128`) use

2. Flexible: ease of conversion to and from the relevant primitive type.

3. Intuitive: exposes an interface one would reasonably expect bitflags to have.

4. Iterable: ability to iterate over set bits by their bit index.

## Credit and Inspirations

I wanted something like the excellent `bitflags` crate without having to define flags in a `struct` with named fields. Many of my games and utilities use flags and enums derived from external (deserialized) lists of strings. As such, this library was born.

### License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), your choice.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
