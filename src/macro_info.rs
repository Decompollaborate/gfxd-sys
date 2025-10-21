/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

//! Inspect information from the current macro.
//!
//! The following functions can be used to obtain information about the current
//! macro and its arguments.
//!
//! They should only be used in custom handlers and callbacks from within
//! [`gfxd_execute`]. If used elsewhere, their behavior is undefined.
//!
//! [`gfxd_execute`]: crate::execution::gfxd_execute

use crate::ffi;

use crate::arg_type::ArgType;

use crate::ptr::NonNullConst;

extern "C" {
    /// Returns the offset in the input data of the current macro.
    ///
    /// The offset starts at zero when [`gfxd_execute`] is called.
    ///
    /// [`gfxd_execute`]: crate::execution::gfxd_execute
    pub fn gfxd_macro_offset() -> ffi::c_int;

    /// Returns the number of `Gfx` packets within the current macro.
    pub fn gfxd_macro_packets() -> ffi::c_int;

    /// Run `fn` for each individual sub-packet the current macro is made up
    /// of.
    ///
    /// During execution of `fn`, the current sub-packet becomes the current
    /// macro that is used by other macro information functions.
    ///
    /// If the current macro is made up of only a single packet it is processed
    /// as a single sub-packet, there is no need to check if the current macro
    /// is a multi-packet macro.
    ///
    /// If at any point `fn` returns a value other than 0, the remaining
    /// sub-packets are skipped and the return value of `fn` is returned.
    ///
    /// If `fn` is [`None`] no processing is done and 0 is returned.
    pub fn gfxd_foreach_pkt(fn_: Option<unsafe extern "C" fn() -> ffi::c_int>) -> ffi::c_int;

    /// Returns a pointer to the input data for the current macro.
    ///
    /// The data is not byte-swapped.
    ///
    /// The data has a length of `sizeof(Gfx) * gfxd_macro_packets()`.
    pub fn gfxd_macro_data() -> NonNullConst<ffi::c_void>;

    /// Returns a number that uniquely identifies the current macro.
    ///
    /// The number will be one of the constants in [`MacroId`].
    ///
    /// [`MacroId`]: crate::macro_id::MacroId
    pub fn gfxd_macro_id() -> ffi::c_int;

    /// Returns the name of the current macro.
    ///
    /// If the macro does not have a name (i.e. it's invalid), [`None`] is
    /// returned.
    ///
    /// If a dynamic display list pointer has been specified, the dynamic `g`
    /// version is returned. Otherwise the static `gs` version is returned.
    ///
    /// The returned pointer is invalidated by a subsequent call to
    /// [`gfxd_macro_name`].
    pub fn gfxd_macro_name() -> Option<NonNullConst<ffi::c_char>>;

    /// Returns the number of arguments to the current macro.
    ///
    /// Does not include a dynamic display list pointer if one has been
    /// specified.
    pub fn gfxd_arg_count() -> ffi::c_int;

    /// Returns a number that identifies the type of the argument with index
    /// `arg_num`.
    ///
    /// The number will be one of the constants in [`ArgType`].
    pub fn gfxd_arg_type(arg_num: ffi::c_int) -> ffi::c_int;

    /// Returns the name of the argument with index `arg_num`.
    ///
    /// Argument names are not canonical, nor are they needed for macro
    /// disassembly, but they can be useful for informational and diagnostic
    /// purposes.
    pub fn gfxd_arg_name(arg_num: ffi::c_int) -> NonNullConst<ffi::c_char>;

    /// Returns the data format of the argument with index `arg_num`.
    ///
    /// The return value will be [`gfxd_argfmt_i`] for `i32`, [`gfxd_argfmt_u`]
    /// for [`u32`], or [`gfxd_argfmt_f`] for `f32`.
    ///
    /// When accessing the value of the argument with [`gfxd_arg_value`], the
    /// member with the corresponding type should be used.
    pub fn gfxd_arg_fmt(arg_num: ffi::c_int) -> ffi::c_int;

    /// Returns a pointer to the value of the argument with index `arg_num`.
    ///
    /// The value is a union of type `gfxd_value_t`
    pub fn gfxd_arg_value(arg_num: ffi::c_int) -> Option<NonNullConst<gfxd_value_t>>;

    /// Returns a pointer to the value of the argument that is of `type`, and
    /// has order `idx` in all arguments of that type.
    ///
    /// An `idx` of zero returns the first argument that has the specified
    /// type.
    ///
    /// If there is no argument with the given type and order, [`None`] is
    /// returned.
    pub fn gfxd_value_by_type(
        type_: ArgType,
        idx: ffi::c_int,
    ) -> Option<NonNullConst<gfxd_value_t>>;

    /// Returns non-zero if the argument with index `arg_num` is "valid", for
    /// some definition of valid.
    ///
    /// An invalid argument generally means that the disassembler found
    /// inconsistencies in the input data, or that the data can not be
    /// reproduced by the current macro type.
    ///
    /// The argument still has a value that can be printed, though the value is
    /// not guaranteed to make any sense.
    pub fn gfxd_arg_valid(arg_num: ffi::c_int) -> ffi::c_int;
}

pub const gfxd_argfmt_i: _bindgen_ty_5 = _bindgen_ty_5::gfxd_argfmt_i;
pub const gfxd_argfmt_u: _bindgen_ty_5 = _bindgen_ty_5::gfxd_argfmt_u;
pub const gfxd_argfmt_f: _bindgen_ty_5 = _bindgen_ty_5::gfxd_argfmt_f;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_5 {
    gfxd_argfmt_i = 0,
    gfxd_argfmt_u = 1,
    gfxd_argfmt_f = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union gfxd_value_t {
    pub i: i32,
    pub u: u32,
    pub f: f32,
}
