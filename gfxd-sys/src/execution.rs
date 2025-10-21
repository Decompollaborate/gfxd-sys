/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

//! Disassemble the `Gfx` packets.
//!
//! Decompilation is started using the [`gfxd_execute`] function. When gfxd is
//! executing (i.e. after [`gfxd_execute`] has been entered, and before it
//! returns), the general settings and the I/O settings should not be changed.

use core::ffi;

extern "C" {
    /// Start executing gfxd with the current settings.
    ///
    /// For each macro, the macro handler registered with [`gfxd_macro_fn`] is
    /// called.
    ///
    /// Execution ends when the input ends, the macro handler returns non-zero,
    /// when an invalid macro is encountered and [`gfxd_stop_on_invalid`] is
    /// enabled, or when `SPBranchList` or `SPEndDisplayList` is encountered
    /// and [`gfxd_stop_on_end`] is enabled.
    ///
    /// If execution ends due to an invalid macro, `-1` is returned.
    ///
    /// If execution ends because the macro handler returns non-zero, the
    /// return value from the macro handler is returned.
    ///
    /// Otherwise zero is returned.
    ///
    /// [`gfxd_macro_fn`]: crate::handlers::gfxd_macro_fn
    /// [`gfxd_stop_on_invalid`]: crate::settings::gfxd_stop_on_invalid
    /// [`gfxd_stop_on_end`]: crate::settings::gfxd_stop_on_end
    pub fn gfxd_execute() -> ffi::c_int;
}
