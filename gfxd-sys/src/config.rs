/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

//! Store and switch settings.
//!
//! A single global or thread-local config is used by default, but multiple
//! configurations can be used to quickly switch back and forth between
//! different settings.

use crate::ptr::{NonNullMut, Opaque};

extern "C" {
    /// Allocates a new default-initialized config struct.
    ///
    /// The return value is an opaque pointer to a [`gfxd_config`] that is not
    /// meant to be dereferenced, only used with [`gfxd_free_config`] and
    /// [`gfxd_set_config`].
    ///
    /// The config struct should be freed with [`gfxd_free_config`] when it is
    /// no longer needed.
    pub fn gfxd_alloc_config() -> NonNullMut<gfxd_config>;

    /// Frees a config struct previously allocated with [`gfxd_alloc_config`].
    ///
    /// The config struct must be deselected by [`gfxd_set_config`] first.
    pub fn gfxd_free_config(config: NonNullMut<gfxd_config>);

    /// Selects the config struct to be used when configuring and executing
    /// gfxdis.
    ///
    /// Must not be used while `gfxd` is executing.
    ///
    /// Setting `config` to [`None`] selects the global or thread-local config.
    pub fn gfxd_set_config(config: Option<NonNullMut<gfxd_config>>);

    pub fn gfxd_get_config() -> NonNullMut<gfxd_config>;
}

#[repr(C)]
pub struct gfxd_config {
    _data: Opaque,
}
