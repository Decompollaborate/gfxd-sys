/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

//! Utilities for extending argument output.
//!
//! When the default handlers are overridden or extended, the custom handler
//! functions will want to do some output of their own.
//!
//! The following methods are available for inserting custom text into the
//! `gfxd` output.

use crate::ffi;

use crate::{arg_type::ArgType, macro_info::gfxd_value_t};

use crate::ptr::NonNullConst;

extern "C" {
    /// Insert `count` bytes from the buffer at `buf` into the output.
    ///
    /// The number of characters written is returned.
    pub fn gfxd_write(buf: NonNullConst<ffi::c_void>, count: ffi::c_int) -> ffi::c_int;

    /// Insert the null-terminated string at `str` into the output.
    ///
    /// The number of characters written is returned.
    pub fn gfxd_puts(str_: NonNullConst<ffi::c_char>) -> ffi::c_int;

    /// Insert the printf-formatted string described by `fmt` and additional
    /// arguments into the output.
    ///
    /// Limited to 255 characters.
    ///
    /// The number of characters written is returned.
    pub fn gfxd_printf(fmt: NonNullConst<ffi::c_char>, ...) -> ffi::c_int;

    // Rust currently has no way to represent a C `va_list`.
    // pub fn gfxd_vprintf( fmt: NonNullConst<ffi::c_char>, arg: *mut __va_list_tag) -> ffi::c_int;

    /// Insert the type-formatted value into the output.
    ///
    /// The type should be one of the [`ArgType`] constants.
    ///
    /// The number of characters written is returned.
    ///
    /// The macro argument with index `n` can be printed with
    /// `gfxd_print_value(`[`gfxd_arg_type`]`(n), `[`gfxd_arg_value`]`(n))`.
    ///
    /// [`gfxd_arg_type`]: crate::macro_info::gfxd_arg_type
    /// [`gfxd_arg_value`]: crate::macro_info::gfxd_arg_value
    pub fn gfxd_print_value(type_: ArgType, value: NonNullConst<gfxd_value_t>) -> ffi::c_int;
}
