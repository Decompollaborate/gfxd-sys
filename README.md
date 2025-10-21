# gfxd-sys

Raw FFI bindings for glankk's [`libgfxd`](https://github.com/glankk/libgfxd) C
library to be used in Rust.

Refer to the original documentation for usage. This crate also provides adapted
documentation, but it is not guaranteed to be 100% correct.

## Minimum Supported Rust Version (MSRV)

The current version of rabbitizer requires **Rust 1.64.0 or greater**.

The current policy is that this may be changed in minor version updates.

## Cargo features

Currently none of the available features are enabled by default.

- `std`: Turns on `std` (or turn off `no_std`, depending on how you prefer it).
  Even when this crate does not depend on Rust's `std`, the internal library it
  is wrapping (`libgfxd`) does depend on the C standard library, including
  io functions like the `printf` family and allocation functions like the
  `malloc` family.
