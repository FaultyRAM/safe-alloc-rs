// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Types for safely allocating memory.

#![cfg_attr(feature = "clippy", allow(inline_always))]

use core::{fmt, intrinsics, isize};
use core::ptr::Unique;
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
/// Performs sanity checks on the length and alignment of a requested memory allocation.
fn check_len_and_align(len: usize, align: usize) -> Result<()> {
    check_len(len).and_then(
        |_| if usize::is_power_of_two(align) {
            Ok(())
        } else {
            Err(Error::BadAlignment)
        }
    )
}

#[allow(missing_copy_implementations)]
/// An owned, allocated block of memory.
pub struct Allocation {
    /// A raw pointer to the allocated block of memory.
    ptr: Unique<u8>,
    /// The length in bytes of the allocation.
    len: usize,
    /// The alignment in bytes of the allocation.
    align: usize,
}

#[cfg_attr(feature = "clippy", allow(len_without_is_empty))]
impl Allocation {
    #[inline]
    /// Allocates a block of memory using the specified length and alignment.
    pub fn new(len: usize, align: usize) -> Result<Allocation> {
        check_len_and_align(len, align).and_then(
            |_| {
                let ptr = unsafe { __rust_allocate(len, align) };
                if ptr.is_null() {
                    Err(Error::NotEnoughMemory)
                } else {
                    Ok(
                        Allocation {
                            ptr: unsafe { Unique::new(ptr) },
                            len: len,
                            align: align,
                        }
                    )
                }
            }
        )
    }

    #[inline]
    /// Allocates a block of memory with all bytes initialized to zero, using the specified length
    /// and alignment.
    pub fn zeroed(len: usize, align: usize) -> Result<Allocation> {
        check_len_and_align(len, align).and_then(
            |_| {
                let ptr = unsafe { __rust_allocate_zeroed(len, align) };
                if ptr.is_null() {
                    Err(Error::NotEnoughMemory)
                } else {
                    Ok(
                        Allocation {
                            ptr: unsafe { Unique::new(ptr) },
                            len: len,
                            align: align,
                        }
                    )
                }
            }
        )
    }

    #[inline]
    /// Resizes an existing allocation.
    ///
    /// On failure, returns an error without modifying the existing allocation.
    pub fn resize(&mut self, new_len: usize) -> Result<()> {
        check_len(new_len).and_then(
            |_| {
                let ptr =
                    unsafe { __rust_reallocate(self.as_mut_ptr(), self.len, new_len, self.align) };
                if ptr.is_null() {
                    Err(Error::NotEnoughMemory)
                } else {
                    self.ptr = unsafe { Unique::new(ptr) };
                    self.len = new_len;
                    Ok(())
                }
            }
        )
    }

    #[inline]
    /// Resizes an existing allocation without moving it.
    ///
    /// On failure, returns an error without modifying the existing allocation.
    pub fn resize_in_place(&mut self, new_len: usize) -> Result<()> {
        check_len(new_len).and_then(
            |_| {
                self.len = unsafe {
                    __rust_reallocate_inplace(self.as_mut_ptr(), self.len, new_len, self.align)
                };
                Ok(())
            }
        )
    }

    /// Creates a new memory allocation with the same length, alignment and contents as an
    /// existing allocation.
    pub fn duplicate(&self) -> Result<Allocation> {
        Allocation::new(self.len, self.align).map(
            |mut new_alloc| {
                unsafe {
                    intrinsics::copy_nonoverlapping(
                        self.as_ptr(),
                        new_alloc.as_mut_ptr(),
                        self.len,
                    );
                }
                new_alloc
            }
        )
    }

    /// Returns a raw pointer to the allocated block of memory.
    pub fn as_ptr(&self) -> *const u8 {
        unsafe { &*self.ptr.as_ptr() }
    }

    /// Returns a mutable raw pointer to the allocated block of memory.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    /// Returns the length in bytes of the allocated block of memory.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the alignment in bytes of the allocated block of memory.
    pub fn align(&self) -> usize {
        self.align
    }
}

impl Drop for Allocation {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            __rust_deallocate(self.as_mut_ptr(), self.len, self.align);
        }
    }
}

impl fmt::Debug for Allocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Allocation")
            .field("ptr", &self.ptr.as_ptr())
            .field("len", &self.len)
            .field("align", &self.align)
            .finish()
    }
}
