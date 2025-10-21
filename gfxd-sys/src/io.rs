/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

//! Read and write `Gfx` packets.
//!
//! The input consists of any number of `Gfx` packets, and the output is the
//! decompiled macros in plain-text. The endianness and microcode type of the
//! input can be set using [`gfxd_endian`] and [`gfxd_target`].
//!
//! Several methods of doing I/O are available. No method is selected by
//! default, meaning there will be no input, and any output will be discarded.
//!
//! [`gfxd_endian`]: crate::settings::gfxd_endian
//! [`gfxd_target`]: crate::settings::gfxd_target

use core::ffi;

use crate::ptr::NonNullConst;

#[link(name = "gfxd", kind = "static")]
extern "C" {
    /// Use the buffer pointed to by `buf`, of `size` bytes.
    pub fn gfxd_input_buffer(buf: Option<NonNullConst<ffi::c_void>>, size: ffi::c_int);

    /// Use the buffer pointed to by `buf`, of `size` bytes.
    pub fn gfxd_output_buffer(buf: Option<NonNullConst<ffi::c_char>>, size: ffi::c_int);

    /// Use `read()` with the provided file descriptor, `fd`.
    pub fn gfxd_input_fd(fd: ffi::c_int);

    /// Use `write()` with the provided file descriptor, `fd`.
    pub fn gfxd_output_fd(fd: ffi::c_int);

    /// Use the provided callback function, `fn`.
    ///
    /// `fn` should copy at most `count` bytes to/from `buf`, and return the
    /// number of bytes actually copied.
    ///
    /// The input callback should return 0 to signal end of input.
    pub fn gfxd_input_callback(fn_: Option<gfxd_input_fn_t>);

    /// Use the provided callback function, `fn`.
    ///
    /// `fn` should copy at most `count` bytes to/from `buf`, and return the
    /// number of bytes actually copied.
    pub fn gfxd_output_callback(fn_: Option<gfxd_output_fn_t>);
}

pub type gfxd_input_fn_t =
    unsafe extern "C" fn(
        buf: *mut ffi::c_void,
        count: ffi::c_int,
    ) -> ffi::c_int;

pub type gfxd_output_fn_t =
    unsafe extern "C" fn(
        buf: *const ffi::c_char,
        count: ffi::c_int,
    ) -> ffi::c_int;
