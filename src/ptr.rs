/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

//! Pointer utilities.

use core::ptr::NonNull;

#[repr(C)]
pub(crate) struct Opaque {
    // https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// A wrapper around [`NonNull`] that is explicitly a `const` pointer.
#[repr(transparent)]
#[derive(Debug)]
pub struct NonNullConst<T>(NonNull<T>);

/// A wrapper around [`NonNull`] that is explicitly a `mut` pointer.
#[repr(transparent)]
#[derive(Debug)]
pub struct NonNullMut<T>(NonNull<T>);

impl<T> NonNullConst<T> {
    /// Convert into a plain [`NonNull`].
    #[inline]
    #[must_use]
    pub const fn into_nonnull(self) -> NonNull<T> {
        self.0
    }
    /// Take a reference to the internal [`NonNull`].
    #[inline]
    #[must_use]
    pub const fn as_nonnull(&self) -> &NonNull<T> {
        &self.0
    }

    /// Creates a new `NonNullConst`.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(ptr: *const T) -> Self {
        // SAFETY: the caller must guarantee that `ptr` is non-null.
        unsafe { Self(NonNull::new_unchecked(cast_mut(ptr))) }
    }

    /// Creates a new `NonNullConst` if `ptr` is non-null.
    #[inline]
    #[must_use]
    pub fn new(ptr: *const T) -> Option<Self> {
        NonNull::new(cast_mut(ptr)).map(Self)
    }

    /// Converts a reference to a `NonNullConst` pointer.
    #[inline]
    #[must_use]
    pub const fn from_ref(r: &T) -> Self {
        // SAFETY: A reference cannot be null.
        unsafe { Self::new_unchecked(r) }
    }

    /// Converts a mutable reference to a `NonNullConst` pointer.
    #[inline]
    #[must_use]
    pub fn from_mut(r: &mut T) -> Self {
        // SAFETY: A mutable reference cannot be null.
        unsafe { Self::new_unchecked(r) }
    }

    /// Acquires the underlying `*const` pointer.
    #[inline]
    #[must_use]
    pub const fn as_ptr(&self) -> *const T {
        self.0.as_ptr()
    }

    /// Returns a shared reference to the value.
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that
    /// the pointer is [convertible to a reference](core::ptr#pointer-to-reference-conversion).
    #[inline]
    #[must_use]
    pub unsafe fn as_ref<'a>(&self) -> &'a T {
        // SAFETY: the caller must guarantee that `self` meets all the
        // requirements for a reference.
        unsafe { &*self.as_ptr() }
    }

    /// Casts to a pointer of another type.
    #[inline]
    #[must_use]
    pub const fn cast<U>(self) -> NonNullConst<U> {
        let ptr = const_cast_to(self.0.as_ptr());

        // SAFETY: `self` is known to be a non-null pointer
        unsafe { NonNullConst::new_unchecked(ptr) }
    }
}

impl<T> NonNullMut<T> {
    /// Convert into a plain [`NonNull`].
    #[inline]
    #[must_use]
    pub const fn into_nonnull(self) -> NonNull<T> {
        self.0
    }
    /// Take a reference to the internal [`NonNull`].
    #[inline]
    #[must_use]
    pub const fn as_nonnull(&self) -> &NonNull<T> {
        &self.0
    }

    /// Creates a new `NonNullMut`.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(ptr: *mut T) -> Self {
        // SAFETY: the caller must guarantee that `ptr` is non-null.
        unsafe { Self(NonNull::new_unchecked(ptr)) }
    }

    /// Creates a new `NonNullMut` if `ptr` is non-null.
    #[inline]
    #[must_use]
    pub fn new(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Converts a mutable reference to a `NonNullMut` pointer.
    #[inline]
    #[must_use]
    pub fn from_mut(r: &mut T) -> Self {
        // SAFETY: A mutable reference cannot be null.
        unsafe { Self::new_unchecked(r) }
    }

    /// Acquires the underlying `*const` pointer.
    #[inline]
    #[must_use]
    pub const fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }

    /// Returns an unique reference to the value.
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that
    /// the pointer is [convertible to a reference](core::ptr#pointer-to-reference-conversion).
    #[inline]
    #[must_use]
    pub unsafe fn as_mut<'a>(&mut self) -> &'a mut T {
        unsafe { &mut *self.as_ptr() }
    }

    /// Casts to a pointer of another type.
    #[inline]
    #[must_use]
    pub const fn cast<U>(self) -> NonNullMut<U> {
        let ptr = mut_cast_to(self.0.as_ptr());

        // SAFETY: `self` is known to be a non-null pointer
        unsafe { NonNullMut::new_unchecked(ptr) }
    }

    /// Casts to a const pointer of the same type.
    #[inline]
    #[must_use]
    pub const fn cast_const(self) -> NonNullConst<T> {
        let ptr = cast_const(self.0.as_ptr());

        // SAFETY: `self` is known to be a non-null pointer
        unsafe { NonNullConst::new_unchecked(ptr) }
    }
}

impl<T> Clone for NonNullConst<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for NonNullConst<T> {}

impl<T> Clone for NonNullMut<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for NonNullMut<T> {}

const fn cast_mut<T>(p: *const T) -> *mut T {
    p as _
}

const fn cast_const<T>(p: *mut T) -> *const T {
    p as _
}

const fn const_cast_to<T, U>(p: *const T) -> *const U {
    p as _
}

const fn mut_cast_to<T, U>(p: *mut T) -> *mut U {
    p as _
}
