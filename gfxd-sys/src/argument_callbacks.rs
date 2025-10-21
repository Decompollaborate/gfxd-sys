/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

//! Override default macro arguments.
//!
//! Callbacks can be registered that will be executed when an argument of a
//! certain type is encountered.
//!
//! The default argument handler [`gfxd_arg_dflt`] will execute callbacks as
//! needed using [`gfxd_arg_callbacks`].
//!
//! If a callback returns non-zero, [`gfxd_arg_dflt`] will not output anything.
//!
//! This is to allow callbacks to override the default argument output.
//! Otherwise, [`gfxd_arg_dflt`] will output the argument value after the
//! callback function's output.
//!
//! [`gfxd_arg_dflt`]: crate::handlers::gfxd_arg_dflt

use core::ffi;

extern "C" {
    /// Examines the argument with index `arg_num` and executes the callback
    /// function for that argument type, if such a callback is supported and
    /// has been registered.
    /// This function returns the value that was returned by the callback
    /// function.
    /// If no callback function has been registered for the argument type, zero
    /// is returned.
    ///
    ///
    /// Most argument callbacks have some extra parameters containing
    /// information that might be relevant to the argument that triggered the
    /// callback.
    /// The extra information is extracted only from the current macro, as
    /// `gfxd` does not retain any context information from previous or
    /// subsequent macros.
    /// If any of the extra parameter values is not available in the current
    /// macro, the value for that parameter is substituted with `-1` for signed
    /// parameters, and zero for unsigned parameters.
    pub fn gfxd_arg_callbacks(arg_num: ffi::c_int) -> ffi::c_int;

    /// Set the callback function for palette arguments.
    ///
    /// The argument type is [`gfxd_Tlut`].
    /// The palette index is in `idx` and the number of colors in `count`.
    ///
    /// [`gfxd_Tlut`]: crate::arg_type::ArgType::gfxd_Tlut
    pub fn gfxd_tlut_callback(fn_: Option<gfxd_tlut_fn_t>);

    /// Set the callback function for texture arguments.
    ///
    /// The argument type is [`gfxd_Timg`].
    /// The image format is in `fmt` and `siz`, the dimensions in `width` and
    /// `height`, and the palette index in `pal`.
    ///
    /// [`gfxd_Timg`]: crate::arg_type::ArgType::gfxd_Timg
    pub fn gfxd_timg_callback(fn_: Option<gfxd_timg_fn_t>);

    /// Set the callback function for frame buffer arguments.
    ///
    /// The argument type is [`gfxd_Cimg`].
    /// The image format is in `fmt` and `siz`, and the horizontal resolution
    /// in `width`.
    ///
    /// [`gfxd_Cimg`]: crate::arg_type::ArgType::gfxd_Cimg
    pub fn gfxd_cimg_callback(fn_: Option<gfxd_cimg_fn_t>);

    /// Set the callback function for depth buffer arguments.
    ///
    /// The argument type is [`gfxd_Zimg`].
    ///
    /// [`gfxd_Zimg`]: crate::arg_type::ArgType::gfxd_Zimg
    pub fn gfxd_zimg_callback(fn_: Option<gfxd_zimg_fn_t>);

    /// Set the callback function for display list arguments.
    ///
    /// The argument type is [`gfxd_Dl`].
    ///
    /// [`gfxd_Dl`]: crate::arg_type::ArgType::gfxd_Dl
    pub fn gfxd_dl_callback(fn_: Option<gfxd_dl_fn_t>);

    /// Set the callback function for matrix arguments.
    ///
    /// The argument type is [`gfxd_Mtxptr`].
    ///
    /// [`gfxd_Mtxptr`]: crate::arg_type::ArgType::gfxd_Mtxptr
    pub fn gfxd_mtx_callback(fn_: Option<gfxd_mtx_fn_t>);

    /// Set the callback function for lookat array arguments.
    ///
    /// The argument type is [`gfxd_Lookatptr`].
    /// The number of lookat structures (1 or 2) is in `count`.
    ///
    /// [`gfxd_Lookatptr`]: crate::arg_type::ArgType::gfxd_Lookatptr
    pub fn gfxd_lookat_callback(fn_: Option<gfxd_lookat_fn_t>);

    /// Set the callback function for diffuse (`Light *`) or ambient
    /// (`Ambient *`) light arguments.
    ///
    /// The argument type is [`gfxd_Lightptr`].
    ///
    /// [`gfxd_Lightptr`]: crate::arg_type::ArgType::gfxd_Lightptr
    pub fn gfxd_light_callback(fn_: Option<gfxd_light_fn_t>);

    /// Set the callback function for Lights_M_ arguments.
    ///
    /// The argument type is [`gfxd_Lightsn`].
    /// The number of diffuse lights used is in `num`.
    ///
    /// [`gfxd_Lightsn`]: crate::arg_type::ArgType::gfxd_Lightsn
    pub fn gfxd_lightsn_callback(fn_: Option<gfxd_lightsn_fn_t>);

    /// Set the callback function for segment base arguments.
    ///
    /// The argument type is [`gfxd_Segptr`].
    /// The segment number is in `num`.
    ///
    /// [`gfxd_Segptr`]: crate::arg_type::ArgType::gfxd_Segptr
    pub fn gfxd_seg_callback(fn_: Option<gfxd_seg_fn_t>);

    /// Set the callback function for vertex array arguments.
    ///
    /// The argument type is [`gfxd_Vtxptr`].
    /// The number of vertex structures is in `num`.
    ///
    /// [`gfxd_Vtxptr`]: crate::arg_type::ArgType::gfxd_Vtxptr
    pub fn gfxd_vtx_callback(fn_: Option<gfxd_vtx_fn_t>);

    /// Set the callback function for viewport arguments.
    ///
    /// The argument type is [`gfxd_Vpptr`].
    ///
    /// [`gfxd_Vpptr`]: crate::arg_type::ArgType::gfxd_Vpptr
    pub fn gfxd_vp_callback(fn_: Option<gfxd_vp_fn_t>);

    /// Set the callback function for microcode text arguments.
    ///
    /// The argument type is [`gfxd_Uctext`].
    /// The size of the text segment is in `size`.
    ///
    /// [`gfxd_Uctext`]: crate::arg_type::ArgType::gfxd_Uctext
    pub fn gfxd_uctext_callback(fn_: Option<gfxd_uctext_fn_t>);

    /// Set the callback function for microcode data arguments.
    ///
    /// The argument type is [`gfxd_Ucdata`].
    /// The size of the data segment is in `size`.
    ///
    /// [`gfxd_Ucdata`]: crate::arg_type::ArgType::gfxd_Ucdata
    pub fn gfxd_ucdata_callback(fn_: Option<gfxd_ucdata_fn_t>);

    /// Set the callback function for generic pointer arguments.
    ///
    /// The argument type is [`gfxd_Dram`].
    /// The size of the data is in `size`.
    ///
    /// [`gfxd_Dram`]: crate::arg_type::ArgType::gfxd_Dram
    pub fn gfxd_dram_callback(fn_: Option<gfxd_dram_fn_t>);
}

/// The argument type is [`gfxd_Tlut`].
/// The palette index is in `idx` and the number of colors in `count`.
///
/// [`gfxd_Tlut`]: crate::arg_type::ArgType::gfxd_Tlut
pub type gfxd_tlut_fn_t = unsafe extern "C" fn(tlut: u32, idx: i32, count: i32) -> ffi::c_int;

/// The argument type is [`gfxd_Timg`].
/// The image format is in `fmt` and `siz`, the dimensions in `width` and
/// `height`, and the palette index in `pal`.
///
/// [`gfxd_Timg`]: crate::arg_type::ArgType::gfxd_Timg
pub type gfxd_timg_fn_t = unsafe extern "C" fn(
    timg: u32,
    fmt: i32,
    siz: i32,
    width: i32,
    height: i32,
    pal: i32,
) -> ffi::c_int;

/// The argument type is [`gfxd_Cimg`].
/// The image format is in `fmt` and `siz`, and the horizontal resolution
/// in `width`.
///
/// [`gfxd_Cimg`]: crate::arg_type::ArgType::gfxd_Cimg
pub type gfxd_cimg_fn_t =
    unsafe extern "C" fn(cimg: u32, fmt: i32, siz: i32, width: i32) -> ffi::c_int;

/// The argument type is [`gfxd_Zimg`].
///
/// [`gfxd_Zimg`]: crate::arg_type::ArgType::gfxd_Zimg
pub type gfxd_zimg_fn_t = unsafe extern "C" fn(zimg: u32) -> ffi::c_int;

/// The argument type is [`gfxd_Dl`].
///
/// [`gfxd_Dl`]: crate::arg_type::ArgType::gfxd_Dl
pub type gfxd_dl_fn_t = unsafe extern "C" fn(dl: u32) -> ffi::c_int;

/// The argument type is [`gfxd_Mtxptr`].
///
/// [`gfxd_Mtxptr`]: crate::arg_type::ArgType::gfxd_Mtxptr
pub type gfxd_mtx_fn_t = unsafe extern "C" fn(mtx: u32) -> ffi::c_int;

/// The argument type is [`gfxd_Lookatptr`].
/// The number of lookat structures (1 or 2) is in `count`.
///
/// [`gfxd_Lookatptr`]: crate::arg_type::ArgType::gfxd_Lookatptr
pub type gfxd_lookat_fn_t = unsafe extern "C" fn(lookat: u32, count: i32) -> ffi::c_int;

/// The argument type is [`gfxd_Lightptr`].
///
/// [`gfxd_Lightptr`]: crate::arg_type::ArgType::gfxd_Lightptr
pub type gfxd_light_fn_t = unsafe extern "C" fn(light: u32) -> ffi::c_int;

/// The argument type is [`gfxd_Lightsn`].
/// The number of diffuse lights used is in `num`.
///
/// [`gfxd_Lightsn`]: crate::arg_type::ArgType::gfxd_Lightsn
pub type gfxd_lightsn_fn_t = unsafe extern "C" fn(lightsn: u32, num: i32) -> ffi::c_int;

/// The argument type is [`gfxd_Segptr`].
/// The segment number is in `num`.
///
/// [`gfxd_Segptr`]: crate::arg_type::ArgType::gfxd_Segptr
pub type gfxd_seg_fn_t = unsafe extern "C" fn(seg: u32, num: i32) -> ffi::c_int;

/// The argument type is [`gfxd_Vtxptr`].
/// The number of vertex structures is in `num`.
///
/// [`gfxd_Vtxptr`]: crate::arg_type::ArgType::gfxd_Vtxptr
pub type gfxd_vtx_fn_t = unsafe extern "C" fn(vtx: u32, num: i32) -> ffi::c_int;

/// The argument type is [`gfxd_Vpptr`].
///
/// [`gfxd_Vpptr`]: crate::arg_type::ArgType::gfxd_Vpptr
pub type gfxd_vp_fn_t = unsafe extern "C" fn(vp: u32) -> ffi::c_int;

/// The argument type is [`gfxd_Uctext`].
/// The size of the text segment is in `size`.
///
/// [`gfxd_Uctext`]: crate::arg_type::ArgType::gfxd_Uctext
pub type gfxd_uctext_fn_t = unsafe extern "C" fn(text: u32, size: u32) -> ffi::c_int;

/// The argument type is [`gfxd_Ucdata`].
/// The size of the data segment is in `size`.
///
/// [`gfxd_Ucdata`]: crate::arg_type::ArgType::gfxd_Ucdata
pub type gfxd_ucdata_fn_t = unsafe extern "C" fn(data: u32, size: u32) -> ffi::c_int;

/// The argument type is [`gfxd_Dram`].
/// The size of the data is in `size`.
///
/// [`gfxd_Dram`]: crate::arg_type::ArgType::gfxd_Dram
pub type gfxd_dram_fn_t = unsafe extern "C" fn(dram: u32, size: u32) -> ffi::c_int;
