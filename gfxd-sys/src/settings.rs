/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

//! These functions control general input and output settings.

#[repr(C)]
pub struct gfxd_ucode {
    _unused: [u8; 0],
}
pub type gfxd_ucode_t = *const gfxd_ucode;

extern "C" {
    pub static gfxd_f3d: gfxd_ucode_t;
}
extern "C" {
    pub static gfxd_f3db: gfxd_ucode_t;
}
extern "C" {
    pub static gfxd_f3dex: gfxd_ucode_t;
}
extern "C" {
    pub static gfxd_f3dexb: gfxd_ucode_t;
}
extern "C" {
    pub static gfxd_f3dex2: gfxd_ucode_t;
}


extern "C" {
    /// Select `ucode` as the target microcode.
    ///
    /// `ucode` can be `gfxd_f3d`, `gfxd_f3db`, `gfxd_f3dex`, `gfxd_f3dexb`, or
    /// `gfxd_f3dex2`.
    ///
    /// The microcode must be selected before `gfxd_execute`, as no microcode
    /// is selected by default.
    pub fn gfxd_target(ucode: gfxd_ucode_t);
}


pub const gfxd_endian_big: _bindgen_ty_4 = _bindgen_ty_4::gfxd_endian_big;
pub const gfxd_endian_little: _bindgen_ty_4 = _bindgen_ty_4::gfxd_endian_little;
pub const gfxd_endian_host: _bindgen_ty_4 = _bindgen_ty_4::gfxd_endian_host;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_4 {
    gfxd_endian_big = 0,
    gfxd_endian_little = 1,
    gfxd_endian_host = 2,
}

extern "C" {
    /// Select `endian` as the endianness of the input, and `wordsize` as the
    /// size of each word in number of bytes.
    ///
    /// `endian` can be `gfxd_endian_big`, `gfxd_endian_little`, or
    /// `gfxd_endian_host` (the endianness of the host machine).
    ///
    /// `wordsize` can be 1, 2, 4, or 8. Big endian is selected by default,
    /// with a word size of 4.
    pub fn gfxd_endian(endian: ::core::ffi::c_int, wordsize: ::core::ffi::c_int);
}

extern "C" {
    /// Enable or disable the use of dynamic `g` macros instead of static `gs`
    /// macros, and select the dynamic display list pointer argument to be
    /// used.
    ///
    /// `arg` will be used by `gfxd_macro_dflt` as the first argument to
    /// dynamic macros.
    ///
    /// If `arg` is `null`, dynamic macros are disabled, and `gs` macros are
    /// used.
    ///
    /// Also affects the result of `gfxd_macro_name`, as it will return either
    /// the dynamic or static version of the macro name as selected by this
    /// setting.
    pub fn gfxd_dynamic(arg: *const ::core::ffi::c_char);
}

pub const gfxd_stop_on_invalid: _bindgen_ty_3 = _bindgen_ty_3::gfxd_stop_on_invalid;
pub const gfxd_stop_on_end: _bindgen_ty_3 = _bindgen_ty_3::gfxd_stop_on_end;
pub const gfxd_emit_dec_color: _bindgen_ty_3 = _bindgen_ty_3::gfxd_emit_dec_color;
pub const gfxd_emit_q_macro: _bindgen_ty_3 = _bindgen_ty_3::gfxd_emit_q_macro;
pub const gfxd_emit_ext_macro: _bindgen_ty_3 = _bindgen_ty_3::gfxd_emit_ext_macro;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_3 {
    gfxd_stop_on_invalid = 0,
    gfxd_stop_on_end = 1,
    gfxd_emit_dec_color = 2,
    gfxd_emit_q_macro = 3,
    gfxd_emit_ext_macro = 4,
}

extern "C" {
    /// Enable or disable the feature specified by `cap`.
    ///
    /// Can be one of the following;
    /// - `gfxd_stop_on_invalid`: Stop execution when encountering an invalid
    ///   macro.
    ///
    ///   Enabled by default.
    ///
    /// - `gfxd_stop_on_end`: Stop execution when encountering a `SPBranchList`
    ///   or `SPEndDisplayList`.
    ///
    ///   Enabled by default.
    ///
    /// - `gfxd_emit_dec_color`: Print color components as decimal instead of
    ///   hexadecimal.
    ///
    ///   Disabled by default.
    ///
    /// - `gfxd_emit_q_macro`: Print fixed-point conversion `q` macros for
    ///   fixed-point values.
    ///   Disabled by default.
    ///
    /// - `gfxd_emit_ext_macro`: Emit non-standard macros.
    ///
    ///   Some commands are valid (though possibly meaningless), but have no
    ///   macros associated with them, such as a standalone `G_RDPHALF_1`. When
    ///   this feature is enabled, such a command will produce a non-standard
    ///   `gsDPHalf1` macro instead of a raw hexadecimal command.
    ///
    ///   Also enables some non-standard multi-packet texture loading macros.
    ///
    ///   Disabled by default.
    pub fn gfxd_enable(cap: ::core::ffi::c_int);

    /// Enable or disable the feature specified by `cap`.
    ///
    /// See [`gfxd_enable`] for possible values.
    pub fn gfxd_disable(cap: ::core::ffi::c_int);
}

extern "C" {
    /// Set or get a generic pointer that can be used to pass user-defined data
    /// in and out of callback functions.
    pub fn gfxd_udata_set(ptr: *mut ::core::ffi::c_void);

    /// Set or get a generic pointer that can be used to pass user-defined data
    /// in and out of callback functions.
    pub fn gfxd_udata_get() -> *mut ::core::ffi::c_void;
}
