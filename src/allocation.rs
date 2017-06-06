// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Types for safely allocating memory.

use core::{fmt, intrinsics, mem};
use super::heap;
use core::ptr::Unique;
use super::result::Result;

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
        unsafe { heap::allocate(len, align).map(|ptr| Self::from_raw(ptr, len, align)) }
    }

    #[inline]
    /// Allocates a block of memory with all bytes initialized to zero, using the specified length
    /// and alignment.
    pub fn zeroed(len: usize, align: usize) -> Result<Allocation> {
        unsafe { heap::allocate_zeroed(len, align).map(|ptr| Self::from_raw(ptr, len, align)) }
    }

    #[inline]
    /// Takes ownership of a raw pointer, length and alignment, and treats the three as an
    /// existing allocation.
    ///
    /// This is unsafe because it assumes that the pointer refers to memory allocated via the Rust
    /// allocation model using the given length and alignment. Undefined behavior will occur if
    /// these assumptions do not hold true.
    pub unsafe fn from_raw(ptr: *mut u8, len: usize, align: usize) -> Allocation {
        Allocation {
            ptr: Unique::new(ptr),
            len: len,
            align: align,
        }
    }

    #[cfg_attr(feature = "clippy", allow(mem_forget))]
    #[inline]
    /// Consumes an allocation without freeing associated memory, returning its pointer, length
    /// and alignment.
    ///
    /// Care must be taken to ensure that the memory is correctly freed after calling this method.
    /// This can be done by reconstructing the allocation via `Allocation::from_raw` and dropping
    /// it immediately afterwards.
    pub fn into_raw(self) -> (*mut u8, usize, usize) {
        let ret = (self.ptr.as_ptr(), self.len, self.align);
        mem::forget(self);
        ret
    }

    #[inline]
    /// Resizes an existing allocation.
    ///
    /// On failure, returns an error without modifying the existing allocation.
    pub fn resize(&mut self, new_len: usize) -> Result<()> {
        unsafe {
            heap::reallocate(self.as_mut_ptr(), self.len, new_len, self.align).map(
                |ptr| {
                    self.ptr = Unique::new(ptr);
                    self.len = new_len;
                    ()
                }
            )
        }
    }

    #[inline]
    /// Resizes an existing allocation without moving it.
    ///
    /// On failure, returns an error without modifying the existing allocation.
    pub fn resize_in_place(&mut self, new_len: usize) -> Result<()> {
        unsafe {
            heap::reallocate_inplace(self.as_mut_ptr(), self.len, new_len, self.align).map(
                |len| {
                    self.len = len;
                    ()
                }
            )
        }
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
            heap::deallocate(self.as_mut_ptr(), self.len, self.align);
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
