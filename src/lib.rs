// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Memory allocation in safe Rust, without aborting on failure.

#![no_std]
#![needs_allocator]
#![feature(allocator)]
#![feature(core_intrinsics)]
#![feature(needs_allocator)]
#![feature(unique)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(clippy))]
#![cfg_attr(feature = "clippy", forbid(clippy_internal))]
#![cfg_attr(feature = "clippy", deny(clippy_pedantic))]
#![cfg_attr(feature = "clippy", forbid(clippy_restrictions))]
#![deny(warnings)]
#![forbid(anonymous_parameters)]
#![forbid(box_pointers)]
#![forbid(fat_ptr_transmutes)]
#![deny(missing_copy_implementations)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_docs)]
#![forbid(trivial_casts)]
#![forbid(trivial_numeric_casts)]
#![forbid(unused_extern_crates)]
#![forbid(unused_import_braces)]
#![deny(unused_qualifications)]
#![allow(unused_attributes)]
#![forbid(unused_results)]
#![forbid(variant_size_differences)]

pub mod allocation;
pub mod error;
mod heap;
pub mod result;
