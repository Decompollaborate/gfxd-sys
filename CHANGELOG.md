# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2026-09-25

### Upstream

- Update `libgfxd` from commit [75791ff](https://github.com/glankk/libgfxd/commit/75791ff7c5f09edb1a05b6caede8be004d47eee0)` to commit [49ec1bb](https://github.com/glankk/libgfxd/commit/49ec1bb893a16b769ddfd0b329d238a658dc0ea2)
- Includes the following changes:
  - <https://github.com/glankk/libgfxd/pull/13>: "Fix: get_more_input does not respect stop_on_end - #13".

### Fixes

- `[Upstream change]` Respect `stop_on_end` and signal no errors even when
  there's invalid data after the found end.

## [0.1.1] - 2025-11-10

### Fixed

- Add some missing metadata in the `Cargo.toml` file.
- Fix some metadata typos.

## [0.1.0] - 2025-11-04

- Initial release.

[unreleased]: https://github.com/Decompollaborate/gfxd-sys/compare/0.1.2...HEAD

[0.1.2]: https://github.com/Decompollaborate/gfxd-sys/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/Decompollaborate/gfxd-sys/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/Decompollaborate/gfxd-sys/releases/tag/0.1.0
