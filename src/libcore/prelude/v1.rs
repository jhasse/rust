// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The core prelude
//!
//! This module is intended for users of libcore which do not link to libstd as
//! well. This module is imported by default when `#![no_std]` is used in the
//! same manner as the standard library's prelude.

#![unstable(feature = "core_prelude",
            reason = "the libcore prelude has not been scrutinized and \
                      stabilized yet")]

// Reexported core operators
pub use marker::{Copy, Send, Sized, Sync};
pub use ops::{Drop, Fn, FnMut, FnOnce};

// Reexported functions
pub use mem::drop;

// Reexported types and traits
pub use char::CharExt;
pub use clone::Clone;
pub use cmp::{PartialEq, PartialOrd, Eq, Ord};
pub use convert::{AsRef, AsMut, Into, From};
pub use default::Default;
pub use iter::IntoIterator;
pub use iter::{Iterator, DoubleEndedIterator, Extend, ExactSizeIterator};
pub use option::Option::{self, Some, None};
pub use result::Result::{self, Ok, Err};
pub use slice::SliceExt;
pub use str::StrExt;
