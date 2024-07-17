# arctan-bitflags - To-Dos

## Nice-to-Have

- Proper `nanoserde` (de)serialization of 128-bit numbers in `JSON` and `RON`.
  - `nanoserde::SerJson` `impl`s go up to `u64`- requires a workaround for `u128`.
  - `nanoserde::SerRon` `impl`s go up to `u64`- requires a workaround for `u128`.
  - `nanoserde::DeJsonTok` tokens go up to `u64`- requires a workaround for `u128`.
  - `nanoserde::DeRonTok` tokens go up to `u64`- requires a workaround for `u128`.

- Examples in doc comments.

## Version 1.0.0

- Final proofreading pass

- Test coverage verfication

## Later versions

- Use `#[repr(transparent)]` for each `Bitflags` type?
  - see [nomicon](https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent) for more information
