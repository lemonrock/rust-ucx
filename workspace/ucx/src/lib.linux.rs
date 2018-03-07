// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern crate indexmap;
extern crate libc;
extern crate libc_extra;
#[macro_use] extern crate quick_error;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate ucx_sys;


use self::configuration::*;
use self::configuration::non_blocking_request_memory_customization::*;
use self::configuration::values::*;
use self::print_information::PrintInformation;
use self::ucx_services::ZeroBasedHyperThreadIndex;
use ::libc::FILE;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::marker::PhantomData;
use ::std::sync::Arc;
use ::ucx_sys::*;


/// Configuration.
pub mod configuration;


/// Error helpers.
pub mod errors;


/// Print information helpers.
pub mod print_information;


/// Wrapper around UCX services (ucs) component of UCX.
pub mod ucx_services;


include!("Application.rs");
include!("HyperThreadContext.rs");
include!("HyperThreadContextDropSafety.rs");
