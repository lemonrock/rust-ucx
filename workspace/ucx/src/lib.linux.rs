// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern crate indexmap;
extern crate libc;
#[macro_use] extern crate quick_error;
extern crate ucx_sys;


/// Configuration.
pub mod configuration;


/// Error helpers.
pub mod errors;


/// Print information helpers.
pub mod print_information;


/// Wrapper around UCX services (ucs) component of UCX.
pub mod ucx_services;
