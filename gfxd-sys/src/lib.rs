/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![deny(improper_ctypes)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod io;
pub mod handlers;
pub mod argument_callbacks;
pub mod settings;
pub mod execution;
pub mod macro_info;
pub mod custom_output;
pub mod config;
pub mod macro_id;
pub mod arg_type;

pub mod ptr;
