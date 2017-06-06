// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Low-level memory allocation APIs.

#![cfg_attr(feature = "clippy", allow(inline_always))]

use core::isize;
use super::error::Error;
use super::result::Result;

extern "C" {
    #[allocator]
    fn __rust_allocate(len: usize, align: usize) -> *mut u8;
    fn __rust_allocate_zeroed(len: usize, align: usize) -> *mut u8;
    fn __rust_deallocate(ptr: *mut u8, old_len: usize, align: usize);
    fn __rust_reallocate(ptr: *mut u8, old_len: usize, len: usize, align: usize) -> *mut u8;
    fn __rust_reallocate_inplace(ptr: *mut u8, old_len: usize, len: usize, align: usize) -> usize;
}

#[inline(always)]
/// Performs sanity checks on the length and alignment of a requested memory allocation.
fn check_len_and_align(len: usize, align: usize) -> Result<()> {
    #[cfg_attr(feature = "clippy", allow(cast_sign_loss))]
    #[inline(always)]
    /// Performs sanity checks on the length of a requested memory allocation.
    fn check_len(len: usize) -> Result<()> {
        if len == 0 {
            Err(Error::ZeroLength)
        } else if len > isize::MAX as usize {
            Err(Error::NotEnoughMemory)
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    /// Performs sanity checks on the alignment of a requested memory allocation.
    fn check_align(align: usize) -> Result<()> {
        if usize::is_power_of_two(align) {
            Ok(())
        } else {
            Err(Error::BadAlignment)
        }
    }

    check_len(len).and_then(|_| check_align(align))
}

#[inline(always)]
/// Performs sanity checks on a raw pointer returned from an allocation function.
fn check_ptr(ptr: *mut u8) -> Result<*mut u8> {
    if ptr.is_null() {
        Err(Error::NotEnoughMemory)
    } else {
        Ok(ptr)
    }
}

#[inline]
/// Allocates a block of memory using the specified length and alignment.
pub unsafe fn allocate(len: usize, align: usize) -> Result<*mut u8> {
    check_len_and_align(len, align).and_then(|_| check_ptr(__rust_allocate(len, align)))
}

#[inline]
/// Allocates a block of memory with all bytes initialized to zero, using the specified length
/// and alignment.
pub unsafe fn allocate_zeroed(len: usize, align: usize) -> Result<*mut u8> {
    check_len_and_align(len, align).and_then(|_| check_ptr(__rust_allocate_zeroed(len, align)))
}

#[inline]
/// Resizes an existing allocation to the specified length.
///
/// The `old_len` and `align` parameters are respectively the length and alignment of the existing
/// allocation.
///
/// If successful, the memory at `ptr` is undefined.
///
/// On failure, returns an `Error` without affecting the existing allocation.
pub unsafe fn reallocate(
    ptr: *mut u8,
    old_len: usize,
    len: usize,
    align: usize,
) -> Result<*mut u8> {
    check_len_and_align(len, align)
        .and_then(|_| check_ptr(__rust_reallocate(ptr, old_len, len, align)))
}

#[inline]
/// Resizes an existing allocation without moving it.
///
/// The `old_len` and `align` parameters are respectively the length and alignment of the existing
/// allocation.
///
/// On failure, returns an `Error` without affecting the existing allocation.
pub unsafe fn reallocate_inplace(
    ptr: *mut u8,
    old_len: usize,
    len: usize,
    align: usize,
) -> Result<usize> {
    check_len_and_align(len, align).map(|_| __rust_reallocate_inplace(ptr, old_len, len, align))
}

#[inline]
/// Deallocates a block of memory.
pub unsafe fn deallocate(ptr: *mut u8, len: usize, align: usize) {
    __rust_deallocate(ptr, len, align)
}
