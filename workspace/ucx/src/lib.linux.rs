// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[macro_use] extern crate bitflags;
extern crate indexmap;
extern crate libc;
extern crate libc_extra;
#[macro_use] extern crate quick_error;
extern crate ring;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate spin_locks;
extern crate ucx_sys;


include!("macros/mod.rs");


use self::attributes::*;
use self::buffers::*;
use self::client_server::*;
use self::configuration::non_blocking_request_memory_customization::*;
use self::configuration::values::*;
use self::cpu_set::*;
use self::ffi_helpers::*;
use self::handle_drop_safeties::*;
use self::print_information::PrintInformation;
use self::remotely_accessible::*;
use self::sockets::SocketAddress;
use self::status::*;
use ::libc::c_void;
use ::libc::FILE;
use ::ring::aead::OpeningKey;
use ::ring::aead::SealingKey;
use ::std::cell::RefCell;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::marker::PhantomData;
use ::std::mem::forget;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::os::unix::io::RawFd;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::rc::Rc;
use ::std::rc::Weak;
use ::ucx_sys::*;


/// Attributes of some UCX objects.
pub mod attributes;


/// Traits to help with buffers.
pub mod buffers;


/// Client server model of working.
pub mod client_server;


/// Configuration.
pub mod configuration;


/// Wrapper around CPU set.
pub mod cpu_set;


/// Status, error and non-blocking request support.
pub mod status;


mod ffi_helpers;


mod handle_drop_safeties;


/// Remotely accessible information.
pub mod remotely_accessible;


/// Supportfor InfiniBand sockets and related structures.
pub mod sockets;


/// Print information helpers.
pub mod print_information;


include!("ApplicationContext.rs");
include!("EndPoint.rs");
include!("EndPointReadyToConsumeStreamingData.rs");
include!("MemoryAddress.rs");
include!("MemoryAdvice.rs");
include!("TheirRemotelyAccessibleEndPointAddress.rs");
include!("TheirRemotelyAccessibleMemory.rs");
include!("TheirRemotelyAccessibleMemoryAddress.rs");
include!("TheirRemotelyAccessibleServerAddress.rs");
include!("TheirRemotelyAccessibleWorkerAddress.rs");
include!("OurRemotelyAccessibleMemory.rs");
include!("OurRemotelyAccessibleMemoryAddress.rs");
include!("OurRemotelyAccessibleWorkerAddress.rs");
include!("Worker.rs");
