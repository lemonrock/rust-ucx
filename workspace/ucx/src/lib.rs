// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![recursion_limit = "256"]


//! # ucx
//!
//! This is a crate to provide mid-level wrappers around OpenUCX.
//!


include!("c_str.rs");
include!("panic_on_error.rs");
include!("panic_on_error_with_clean_up.rs");
include!("lib.linux.rs");
