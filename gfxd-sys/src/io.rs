/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

//! Read and write `Gfx` packets.
//!
//! The input consists of any number of `Gfx` packets, and the output is the
//! decompiled macros in plain-text. The endianness and microcode type of the
//! input can be set using `gfxd_endian` and `gfxd_target`.
//!
//! Several methods of doing I/O are available. No method is selected by
//! default, meaning there will be no input, and any output will be discarded.

#[link(name = "gfxd", kind = "static")]
extern "C" {
    /// Use the buffer pointed to by `buf`, of `size` bytes.
    pub fn gfxd_input_buffer(buf: *const ::core::ffi::c_void, size: ::core::ffi::c_int);

    /// Use the buffer pointed to by `buf`, of `size` bytes.
    pub fn gfxd_output_buffer(buf: *mut ::core::ffi::c_char, size: ::core::ffi::c_int);
}

extern "C" {
    /// Use `read()` / `write()` with the provided file descriptor, `fd`.
    pub fn gfxd_input_fd(fd: ::core::ffi::c_int);

    /// Use `read()` / `write()` with the provided file descriptor, `fd`.
    pub fn gfxd_output_fd(fd: ::core::ffi::c_int);
}

pub type gfxd_input_fn_t = ::core::option::Option<
    unsafe extern "C" fn(
        buf: *mut ::core::ffi::c_void,
        count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;

pub type gfxd_output_fn_t = ::core::option::Option<
    unsafe extern "C" fn(
        buf: *const ::core::ffi::c_char,
        count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;

extern "C" {
    /// Use the provided callback function, `fn`.
    ///
    /// `fn` should copy at most `count` bytes to/from `buf`, and return the
    /// number of bytes actually copied. The input callback should return 0 to
    /// signal end of input.
    pub fn gfxd_input_callback(fn_: gfxd_input_fn_t);

    /// Use the provided callback function, `fn`.
    ///
    /// `fn` should copy at most `count` bytes to/from `buf`, and return the
    /// number of bytes actually copied. The input callback should return 0 to
    /// signal end of input.
    pub fn gfxd_output_callback(fn_: gfxd_output_fn_t);
}
