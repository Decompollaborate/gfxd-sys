/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

//! Change or extend decompiled output.
//!
//! The macro handler function is responsible for writing the output of each
//! decompiled macro. The default macro handler is `gfxd_macro_dflt`, but this
//! can be changed with `gfxd_macro_fn`. The new handler can extend the default
//! function by calling `gfxd_macro_dflt` within it, or it can override it
//! completely.

use core::ffi;

extern "C" {
    /// The default macro handler.
    ///
    /// Outputs the macro name, dynamic display list pointer if one has been
    /// specified, and then each argument in order using the function
    /// registered using `gfxd_arg_fn` (`gfxd_arg_dflt` by default), and
    /// returns zero.
    ///
    /// Because it is designed to be extended, it only outputs
    /// the macro text, without any whitespace or punctuation before or
    /// after.
    ///
    /// When this function is used as the sole macro handler, it will
    /// output the entire display list on one line without any separation
    /// between macros, which is probably not what you want.
    pub fn gfxd_macro_dflt() -> ffi::c_int;

    /// Set `fn` to be the macro handler function.
    ///
    /// `fn` can be null, in which case the handler is reset to the default.
    ///
    /// If fn returns a value other than 0, execution stops (see `gfxd_execute`).
    pub fn gfxd_macro_fn(fn_: gfxd_macro_fn_t);

    /// The default argument handler for `gfxd_macro_dflt`.
    ///
    /// For the argument with index `arg_num`, calls `gfxd_arg_callbacks`, and
    /// prints the argument value if the callback returns zero, or if there is
    /// no callback for the given argument.
    pub fn gfxd_arg_dflt(arg_num: ffi::c_int);

    /// Set `fn` to be the argument handler function, called by
    /// `gfxd_macro_dflt`, for each argument in the current macro, not counting
    /// the dynamic display list pointer if one has been specified.
    ///
    /// `fn` can be null, in which case the handler is reset to the default.
    ///
    /// This only affects the output of `gfxd_macro_dflt`, and has no
    /// observable effect if `gfxd_macro_dflt` is overridden (not extended).
    pub fn gfxd_arg_fn(fn_: gfxd_arg_fn_t);
}

pub type gfxd_macro_fn_t = Option<unsafe extern "C" fn() -> ffi::c_int>;

pub type gfxd_arg_fn_t = Option<unsafe extern "C" fn(arg_num: ffi::c_int)>;
