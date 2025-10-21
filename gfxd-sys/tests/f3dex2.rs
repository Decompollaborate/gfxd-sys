/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![deny(unsafe_op_in_unsafe_fn)]

use core::ffi;

use pretty_assertions::assert_eq;

use gfxd_sys::ptr::{NonNullConst, NonNullMut};

// `bytes` must be nul-terminated
unsafe fn c_str_from_bytes(bytes: &[u8]) -> NonNullConst<ffi::c_char> {
    let buf = unsafe { ffi::CStr::from_bytes_with_nul_unchecked(bytes) }.as_ptr();

    unsafe { NonNullConst::new_unchecked(buf) }
}

fn run_gfxd(dlist_data: &[u8]) -> String {
    let mut out_buf = String::new();

    extern "C" fn output(buf: *const ffi::c_char, count: ffi::c_int) -> ffi::c_int {
        let user_data = unsafe { gfxd_sys::settings::gfxd_udata_get() }.unwrap();
        let out_buf = unsafe { user_data.cast::<String>().as_mut() };

        let data = unsafe { ffi::CStr::from_ptr(buf) };

        out_buf.push_str(&data.to_string_lossy());

        count
    }

    extern "C" fn macro_fn() -> ffi::c_int {
        unsafe {
            /* Print a tab before each macro, and a comma and newline after each macro */
            gfxd_sys::custom_output::gfxd_puts(c_str_from_bytes(b"    \0"));
            gfxd_sys::handlers::gfxd_macro_dflt(); /* Execute the default macro handler */
            gfxd_sys::custom_output::gfxd_puts(c_str_from_bytes(b",\n\0"));
        }
        0
    }

    // Setup
    unsafe {
        gfxd_sys::io::gfxd_input_buffer(
            NonNullConst::new(dlist_data.as_ptr().cast()),
            dlist_data.len() as ffi::c_int,
        );
        gfxd_sys::io::gfxd_output_callback(Some(output));
        gfxd_sys::handlers::gfxd_macro_fn(Some(macro_fn));
        gfxd_sys::settings::gfxd_udata_set(NonNullMut::new(&mut out_buf as _).map(|x| x.cast()));
        gfxd_sys::settings::gfxd_target(Some(gfxd_sys::settings::gfxd_f3dex2));
    }

    // Run
    unsafe {
        gfxd_sys::custom_output::gfxd_puts(c_str_from_bytes(b"{\n\0"));
        gfxd_sys::execution::gfxd_execute();
        gfxd_sys::custom_output::gfxd_puts(c_str_from_bytes(b"}\n\0"));
    }

    out_buf
}

// The data for the first 3 tests was taken from pygfxd tests
// https://github.com/Thar0/pygfxd/tree/98663748bfbfba361c938bfd55326ecc615dc7ba/test

#[test]
fn test_f3dex2_1() {
    static DLIST_DATA: [u8; 0x8] = [
        0xDF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    ];
    static EXPECTED: &'static str = "\
{
    gsSPEndDisplayList(),
}
";

    let out_buf = run_gfxd(&DLIST_DATA);

    assert_eq!(EXPECTED, out_buf);
}

#[test]
fn test_f3dex2_2() {
    static DLIST_DATA: [u8; 0x18] = [
        0x01, 0x00, 0x30, 0x06, 0x42, 0x04, 0x20, 0x69, //
        0x05, 0x00, 0x02, 0x04, 0x00, 0x00, 0x00, 0x00, //
        0xDF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    ];
    static EXPECTED: &'static str = "\
{
    gsSPVertex(0x42042069, 3, 0),
    gsSP1Triangle(0, 1, 2, 0),
    gsSPEndDisplayList(),
}
";

    let out_buf = run_gfxd(&DLIST_DATA);

    assert_eq!(EXPECTED, out_buf);
}

#[test]
fn test_f3dex2_3() {
    static DLIST_DATA: [u8; 0x18] = [
        0xDB, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, //
        0xDC, 0x08, 0x06, 0x0A, 0x09, 0x00, 0x00, 0x08, //
        0xDC, 0x08, 0x09, 0x0A, 0x09, 0x00, 0x00, 0x00, //
    ];
    static EXPECTED: &'static str = "\
{
    gsSPSetLights1(*(Lightsn *)0x09000000),
}
";

    let out_buf = run_gfxd(&DLIST_DATA);

    assert_eq!(EXPECTED, out_buf);
}
