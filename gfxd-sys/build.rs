/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

fn main() {
    static C_PATHS: [&str; 6] = [
        "vendor/gfxd.c",
        "vendor/uc_f3d.c",
        "vendor/uc_f3db.c",
        "vendor/uc_f3dex.c",
        "vendor/uc_f3dexb.c",
        "vendor/uc_f3dex2.c",
    ];
    static H_PATHS: [&str; 1] = [
        "vendor/gfxd.h",
    ];

    for path in C_PATHS.iter().chain(&H_PATHS) {
        println!("cargo:rerun-if-changed={path}");
    }

    cc::Build::new()
        .files(C_PATHS)
        .include("vendor")
        .warnings(false)
        .compile("gfxd");
}
