/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

//! Pointer utilities.

use core::{cmp, fmt, hash, ptr::NonNull};

use crate::ffi;

#[repr(C)]
pub(crate) struct Opaque {
    // https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// A wrapper around [`NonNull`] that is explicitly a `const` pointer.
#[repr(transparent)]
pub struct NonNullConst<T>(NonNull<T>)
where
    T: ?Sized;

/// A wrapper around [`NonNull`] that is explicitly a `mut` pointer.
#[repr(transparent)]
pub struct NonNullMut<T>(NonNull<T>)
where
    T: ?Sized;

impl<T> NonNullConst<T>
where
    T: ?Sized,
{
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
        // NonNull wants a mut pointer for whatever reason.
        let mut_ptr = cast_mut(ptr);

        // SAFETY: the caller must guarantee that `ptr` is non-null.
        let inner = unsafe { NonNull::new_unchecked(mut_ptr) };

        Self(inner)
    }

    /// Creates a new `NonNullConst` if `ptr` is non-null.
    #[inline]
    #[must_use]
    pub fn new(ptr: *const T) -> Option<Self> {
        // NonNull wants a mut pointer for whatever reason.
        let mut_ptr = cast_mut(ptr);

        NonNull::new(mut_ptr).map(Self)
    }

    /// Converts a reference to a `NonNullConst` pointer.
    #[inline]
    #[must_use]
    pub const fn from_ref(r: &T) -> Self {
        // SAFETY: A reference cannot be null.
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

impl NonNullConst<ffi::c_void> {
    /// Creates a new `NonNullConst`, casting the passsed pointer to void
    /// pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    #[inline]
    #[must_use]
    pub const unsafe fn new_void_unchecked<U>(ptr: *const U) -> Self
    where
        U: ?Sized,
    {
        // Cast to void pointer.
        let void_ptr: *const ffi::c_void = const_cast_to(ptr);
        // NonNull wants a mut pointer for whatever reason.
        let mut_void_ptr = cast_mut(void_ptr);

        // SAFETY: the caller must guarantee that `ptr` is non-null.
        let inner = unsafe { NonNull::new_unchecked(mut_void_ptr) };

        Self(inner)
    }

    /// Creates a new `NonNullConst` if `ptr` is non-null, casting the pointer
    /// to a void pointer.
    #[inline]
    #[must_use]
    pub fn new_void<U>(ptr: *const U) -> Option<Self>
    where
        U: ?Sized,
    {
        // Cast to void pointer.
        let void_ptr: *const ffi::c_void = const_cast_to(ptr);
        // NonNull wants a mut pointer for whatever reason.
        let mut_void_ptr = cast_mut(void_ptr);

        NonNull::new(mut_void_ptr).map(Self)
    }
}

#[cfg(feature = "std")]
impl NonNullConst<ffi::c_char> {
    /// # Safety
    ///
    /// The pointer pointed to be `self` must be a valid C string.
    ///
    /// Refer to [`CStr::from_ptr`] for details.
    ///
    /// [`CStr::from_ptr`]: std::ffi::CStr::from_ptr
    #[inline]
    #[must_use]
    pub unsafe fn as_c_str(&self) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr(self.as_ptr()) }
    }
}

impl<T> NonNullMut<T>
where
    T: ?Sized,
{
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
        let inner = unsafe { NonNull::new_unchecked(ptr) };

        Self(inner)
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

impl NonNullMut<ffi::c_void> {
    /// Creates a new `NonNullMut`, casting the passsed pointer to void
    /// pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    #[inline]
    #[must_use]
    pub const unsafe fn new_void_unchecked<U>(ptr: *mut U) -> Self
    where
        U: ?Sized,
    {
        // Cast to void pointer.
        let void_ptr: *mut ffi::c_void = mut_cast_to(ptr);

        // SAFETY: the caller must guarantee that `ptr` is non-null.
        let inner = unsafe { NonNull::new_unchecked(void_ptr) };

        Self(inner)
    }

    /// Creates a new `NonNullMut` if `ptr` is non-null, casting the pointer to
    /// a void pointer.
    #[inline]
    #[must_use]
    pub fn new_void<U>(ptr: *mut U) -> Option<Self>
    where
        U: ?Sized,
    {
        // Cast to void pointer.
        let void_ptr: *mut ffi::c_void = mut_cast_to(ptr);

        NonNull::new(void_ptr).map(Self)
    }
}

impl<T> fmt::Debug for NonNullConst<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}
impl<T> fmt::Pointer for NonNullConst<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self, f)
    }
}

impl<T> Clone for NonNullConst<T>
where
    T: ?Sized,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for NonNullConst<T> where T: ?Sized {}

impl<T> PartialEq for NonNullConst<T>
where
    T: ?Sized,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        #[allow(ambiguous_wide_pointer_comparisons)]
        self.0.eq(&other.0)
    }
}
impl<T> Eq for NonNullConst<T> where T: ?Sized {}

impl<T> PartialOrd for NonNullConst<T>
where
    T: ?Sized,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for NonNullConst<T>
where
    T: ?Sized,
{
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        #[allow(ambiguous_wide_pointer_comparisons)]
        self.0.cmp(&other.0)
    }
}

impl<T> hash::Hash for NonNullConst<T>
where
    T: ?Sized,
{
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T> From<&T> for NonNullConst<T>
where
    T: ?Sized,
{
    /// Converts a `&T` to a `NonNullConst<T>`.
    ///
    /// This conversion is safe and infallible since references cannot be null.
    #[inline]
    fn from(r: &T) -> Self {
        Self::from_ref(r)
    }
}

impl<T> fmt::Debug for NonNullMut<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}
impl<T> fmt::Pointer for NonNullMut<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self, f)
    }
}

impl<T> Clone for NonNullMut<T>
where
    T: ?Sized,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for NonNullMut<T> where T: ?Sized {}

impl<T> PartialEq for NonNullMut<T>
where
    T: ?Sized,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        #[allow(ambiguous_wide_pointer_comparisons)]
        self.0.eq(&other.0)
    }
}
impl<T> Eq for NonNullMut<T> where T: ?Sized {}

impl<T> PartialOrd for NonNullMut<T>
where
    T: ?Sized,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for NonNullMut<T>
where
    T: ?Sized,
{
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        #[allow(ambiguous_wide_pointer_comparisons)]
        self.0.cmp(&other.0)
    }
}

impl<T> hash::Hash for NonNullMut<T>
where
    T: ?Sized,
{
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T> From<&mut T> for NonNullMut<T>
where
    T: ?Sized,
{
    /// Converts a `&mut T` to a `NonNullMut<T>`.
    ///
    /// This conversion is safe and infallible since references cannot be null.
    #[inline]
    fn from(r: &mut T) -> Self {
        Self::from_mut(r)
    }
}

const fn cast_mut<T>(p: *const T) -> *mut T
where
    T: ?Sized,
{
    p as _
}

const fn cast_const<T>(p: *mut T) -> *const T
where
    T: ?Sized,
{
    p as _
}

const fn const_cast_to<T, U>(p: *const T) -> *const U
where
    T: ?Sized,
{
    p as _
}

const fn mut_cast_to<T, U>(p: *mut T) -> *mut U
where
    T: ?Sized,
{
    p as _
}
