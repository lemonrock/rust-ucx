// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern crate indexmap;
extern crate libc;
extern crate libc_extra;
#[macro_use] extern crate quick_error;
extern crate ring;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate ucx_sys;


include!("c_str.rs");
include!("panic_on_error.rs");
include!("panic_on_error_with_clean_up.rs");


use self::attributes::*;
use self::buffers::*;
use self::configuration::non_blocking_request_memory_customization::*;
use self::configuration::values::*;
use self::cpu_set::*;
use self::errors::*;
use self::print_information::PrintInformation;
use ::libc::c_void;
use ::libc::FILE;
use ::ring::aead::OpeningKey;
use ::ring::aead::SealingKey;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::marker::PhantomData;
use ::std::mem::uninitialized;
use ::std::os::unix::io::RawFd;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::rc::Rc;
use ::ucx_sys::*;


/// Attributes of some UCX objects.
pub mod attributes;


/// Traits to help with buffers.
pub mod buffers;


/// Wrapper around CPU set.
pub mod cpu_set;


/// Configuration.
pub mod configuration;


/// Error helpers.
pub mod errors;


/// Print information helpers.
pub mod print_information;


include!("ApplicationContext.rs");
include!("ApplicationContextHandleDropSafety.rs");
include!("MemoryAddress.rs");
include!("MemoryAdvice.rs");
include!("OurRemotelyAccessibleMemory.rs");
include!("OurRemotelyAccessibleMemoryHandleDropSafety.rs");
include!("OurRemotelyAccessibleMemoryAddress.rs");
include!("OurRemotelyAccessibleWorkerAddress.rs");
include!("Worker.rs");
include!("WorkerHandleDropSafety.rs");
