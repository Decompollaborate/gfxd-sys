/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// TODO: Use #![doc = include_str!("../README.md")] instead when the MSRV is 1.54+
//! Raw FFI bindings for glankk's [`libgfxd`](https://github.com/glankk/libgfxd)
//! C library to be used in Rust.
//!
//! Refer to the original documentation for usage. This crate also provides
//! adapted documentation, but it is not guaranteed to be 100% correct.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![deny(improper_ctypes)]
// #![deny(unsafe_op_in_unsafe_fn)]

pub mod arg_type;
pub mod argument_callbacks;
pub mod config;
pub mod custom_output;
pub mod execution;
pub mod handlers;
pub mod io;
pub mod macro_id;
pub mod macro_info;
pub mod settings;

pub mod ffi;
pub mod ptr;
