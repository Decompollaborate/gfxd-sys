/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ffi;

use pretty_assertions::assert_eq;

fn as_void_ptr_mut<T>(v: &mut T) -> *mut ffi::c_void {
    v as *mut T as _
}

unsafe fn from_void_ptr_mut<'a, T>(v: *mut ffi::c_void) -> &'a mut T {
    unsafe {    &mut *(v as *mut T) }
}

/*
fn as_nonnull_void_ptr_mut<T>(v: &mut T) -> core::ptr::NonNull<ffi::c_void> {
    unsafe {
        NonNull::new_unchecked(as_void_ptr_mut(v))
    }
}
*/

fn run_gfxd(dlist_data: &[u8]) -> String {
    let mut out_buf = String::new();

    extern "C" fn output(
        buf: *const ffi::c_char,
        count: ffi::c_int,
    ) -> ffi::c_int {
        let out_buf = unsafe {
            let user_data = gfxd_sys::settings::gfxd_udata_get();
            from_void_ptr_mut::<String>(user_data)
        };

        let data = unsafe {
            ffi::CStr::from_ptr(buf)
        };

        out_buf.push_str(&data.to_string_lossy());

        count
    }

    extern "C" fn  macro_fn() -> ffi::c_int {
        unsafe {
            /* Print a tab before each macro, and a comma and newline after each macro */
            gfxd_sys::custom_output::gfxd_puts(ffi::CStr::from_bytes_with_nul_unchecked(b"    \0").as_ptr());
            gfxd_sys::handlers::gfxd_macro_dflt(); /* Execute the default macro handler */
            gfxd_sys::custom_output::gfxd_puts(ffi::CStr::from_bytes_with_nul_unchecked(b",\n\0").as_ptr());
        }
        0
    }

    // Setup
    unsafe {
        gfxd_sys::io::gfxd_input_buffer(dlist_data.as_ptr() as *const ffi::c_void, dlist_data.len() as ffi::c_int);
        gfxd_sys::io::gfxd_output_callback(Some(output));
        gfxd_sys::handlers::gfxd_macro_fn(Some(macro_fn));
        gfxd_sys::settings::gfxd_udata_set(as_void_ptr_mut(&mut out_buf));
        gfxd_sys::settings::gfxd_target(gfxd_sys::settings::gfxd_f3dex2);
    }

    unsafe {
        gfxd_sys::custom_output::gfxd_puts(ffi::CStr::from_bytes_with_nul_unchecked(b"{\n\0").as_ptr());
        gfxd_sys::execution::gfxd_execute();
        gfxd_sys::custom_output::gfxd_puts(ffi::CStr::from_bytes_with_nul_unchecked(b"}\n\0").as_ptr());
    }

    out_buf
}

// The data for the first 3 tests was taken from pygfxd tests
// https://github.com/Thar0/pygfxd/tree/98663748bfbfba361c938bfd55326ecc615dc7ba/test

#[test]
fn test_f3dex2_1() {
    static DLIST_DATA: [u8; 0x8] = [
        0xDF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
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
        0x01, 0x00, 0x30, 0x06, 0x42, 0x04, 0x20, 0x69,
        0x05, 0x00, 0x02, 0x04, 0x00, 0x00, 0x00, 0x00,
        0xDF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
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
        0xDB, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18,
        0xDC, 0x08, 0x06, 0x0A, 0x09, 0x00, 0x00, 0x08,
        0xDC, 0x08, 0x09, 0x0A, 0x09, 0x00, 0x00, 0x00,
    ];
    static EXPECTED: &'static str = "\
{
    gsSPSetLights1(*(Lightsn *)0x09000000),
}
";

    let out_buf = run_gfxd(&DLIST_DATA);

    assert_eq!(EXPECTED, out_buf);
}
