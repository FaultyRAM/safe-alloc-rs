// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Errors in memory management, such as out of memory or bad alignment.

use core::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// A memory management error.
pub enum Error {
    /// There is not enough free memory to satisfy a memory (re)allocation.
    NotEnoughMemory,
    /// An invalid alignment was passed to a memory management function.
    BadAlignment,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Error::NotEnoughMemory => f.write_str("out of memory"),
            Error::BadAlignment => f.write_str("alignment must be a power of two"),
        }
    }
}
