# arctan-bitflags
Bitflag structures with 8, 16, 32, 64, and 128-bit representation.

## Primary Types

- `BitFlags8`:   8-bit flags that convert to and from `u8`.
- `BitFlags16`:  16-bit flags that convert to and from `u16`.
- `BitFlags32`:  32-bit flags that convert to and from `u32`.
- `BitFlags64`:  64-bit flags that convert to and from `u64`.
- `BitFlags128`: 128-bit flags that convert to and from `u128`.

## Usage

### Creation

A new empty instance:
```rust
let flags = BitFlags8::new();
```

A new instance with all bits set:
```rust
let flags = BitFlags8::all();
```

A new instance from corresponding integer (all three are identical):
```rust
let flags = BitFlags8::from_bits(0b0110);

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

## Goals

The primary purpose for creation of this library is for *games*. I wanted a bitflag type that met the following criteria:

1. Specific: a type that is explicitly designated for use as a bitflag. While the primitive integer types (`u8`, `u16`, `u32`, `u64`, `u128`) use

2. Flexible: ease of conversion to and from the relevant primitive type.

3. Intuitive: exposes an interface one would reasonably expect bitflags to have.

4. Iteration: ability to iterate over set bits by their bit index.

5. Works with lists of external `String`s: use auxillary structs to map `String` keys to `BitFlag` values. This is useful for making custom `enum`-like objects from external data.

## To-Dos

- Improved documentation to show usage.

- add all `BitFlag` types.

- Add `str` to `flag` conversion with `StrToFlag` types.

- Update `Usage` section.

- Add `serde` support.
