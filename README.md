# gfxd-sys

Raw FFI bindings for glankk's [`libgfxd`](https://github.com/glankk/libgfxd) C
library to be used in Rust.

Refer to the original documentation for usage. This crate also provides adapted
documentation, but it is not guaranteed to be 100% correct.

## Minimum Supported Rust Version (MSRV)

The current version of gfxd-sys requires **Rust 1.37.0 or greater**.

The current policy is that this may be changed in minor version updates.

## Cargo features

Currently none of the available features are enabled by default.

- `std`: Turns on `std` (or turn off `no_std`, depending on how you prefer it).
  Even when this crate does not depend on Rust's `std`, the internal library it
  is wrapping (`libgfxd`) does depend on the C standard library, including
  IO functions like the `printf` family and allocation functions like the
  `malloc` family.

## License

Licensed under either of MIT license ([LICENSE-MIT](LICENSE-MIT) or
<http://opensource.org/licenses/MIT>).

Every contribution submitted for inclusion in the work by you shall be MIT
licensed, without any additional terms of conditions.

## Versioning and changelog

Currently this crate uses the commit
[75791ff](https://github.com/glankk/libgfxd/commit/75791ff7c5f09edb1a05b6caede8be004d47eee0)
of `libgfxd`.

This library _aims_ to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
We try to always keep backwards compatibility, so no breaking changes should
happen until a major release (i.e. jumping from 1.X.X to 2.0.0), but there are
no hard guarantees since `libgfxd` itself doesn't follow a versioning scheme at
all.

Ideally this crate will provide a new version for each new commit of `libgfxd`,
it also may update in the case any fixes or similar changes are needed.

To see what changed on each release visit either the
[CHANGELOG.md](https://github.com/Decompollaborate/gfxd-sys/blob/-/CHANGELOG.md)
file or check the [releases page on Github](https://github.com/Decompollaborate/gfxd-sys/releases).
You can also use [this link](https://github.com/Decompollaborate/gfxd-sys/releases/latest)
to check the latest release.

## See also

- [`libgfxd`](https://github.com/glankk/libgfxd): The original library.
- [`pygfxd`](https://github.com/Thar0/pygfxd): Python bindings for `libgfxd`.
- [`gfxd-rs`](https://github.com/decompollaborate/gfxd-rs): A safe Rust wrapper
  for `libgfxd`.
