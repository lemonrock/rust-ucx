// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![feature(thread_local)]
#![recursion_limit = "256"]


//! # ucx
//!
//! This is a crate to provide mid-level wrappers around OpenUCX.
//!


#[cfg(target_os = "linux")] #[macro_use] extern crate bitflags;
#[cfg(target_os = "linux")] extern crate indexmap;
#[cfg(target_os = "linux")] extern crate libc;
#[cfg(target_os = "linux")] extern crate libc_extra;
#[cfg(target_os = "linux")] #[macro_use] extern crate quick_error;
#[cfg(target_os = "linux")] extern crate ring;
#[cfg(target_os = "linux")] extern crate serde;
#[cfg(target_os = "linux")] #[macro_use] extern crate serde_derive;
#[cfg(target_os = "linux")] extern crate spin_locks;
#[cfg(target_os = "linux")] extern crate ucx_sys;


#[cfg(target_os = "linux")] include!("macros/mod.rs");


#[cfg(target_os = "linux")] use self::attributes::*;
#[cfg(target_os = "linux")] use self::buffers::*;
#[cfg(target_os = "linux")] use self::client_server::*;
#[cfg(target_os = "linux")] use self::configuration::non_blocking_request_memory_customization::*;
#[cfg(target_os = "linux")] use self::configuration::values::*;
#[cfg(target_os = "linux")] use self::cpu_set::*;
#[cfg(target_os = "linux")] use self::ffi_helpers::*;
#[cfg(target_os = "linux")] use self::handle_drop_safeties::*;
#[cfg(target_os = "linux")] use self::print_information::PrintInformation;
#[cfg(target_os = "linux")] use self::remotely_accessible::*;
#[cfg(target_os = "linux")] use self::sockets::SocketAddress;
#[cfg(target_os = "linux")] use self::status::*;
#[cfg(target_os = "linux")] use self::status::non_blocking_requests::*;
#[cfg(target_os = "linux")] use ::libc::c_void;
#[cfg(target_os = "linux")] use ::libc::FILE;
#[cfg(target_os = "linux")] use ::libc_extra::sched::sched_getcpu;
#[cfg(target_os = "linux")] use ::ring::aead::OpeningKey;
#[cfg(target_os = "linux")] use ::ring::aead::SealingKey;
#[cfg(target_os = "linux")] use ::std::cell::RefCell;
#[cfg(target_os = "linux")] use ::std::fmt;
#[cfg(target_os = "linux")] use ::std::fmt::Debug;
#[cfg(target_os = "linux")] use ::std::fmt::Formatter;
#[cfg(target_os = "linux")] use ::std::marker::PhantomData;
#[cfg(target_os = "linux")] use ::std::mem::forget;
#[cfg(target_os = "linux")] use ::std::mem::transmute;
#[cfg(target_os = "linux")] use ::std::mem::uninitialized;
#[cfg(target_os = "linux")] use ::std::mem::zeroed;
#[cfg(target_os = "linux")] use ::std::os::unix::io::RawFd;
#[cfg(target_os = "linux")] use ::std::ptr::NonNull;
#[cfg(target_os = "linux")] use ::std::ptr::null_mut;
#[cfg(target_os = "linux")] use ::std::rc::Rc;
#[cfg(target_os = "linux")] use ::std::rc::Weak;
#[cfg(target_os = "linux")] use ::ucx_sys::*;


/// Attributes of some UCX objects.
#[cfg(target_os = "linux")] pub mod attributes;


/// Traits and structs to help with un-managed buffers from UCX.
#[cfg(target_os = "linux")] pub mod buffers;


/// Client server model of working.
#[cfg(target_os = "linux")] pub mod client_server;


/// Data types used for describing data sent in tagged messages and streams.
/// A data type lets UCX manage serialization and deserialization.
#[cfg(target_os = "linux")] pub mod data_types;


/// Configuration.
#[cfg(target_os = "linux")] pub mod configuration;


#[cfg(target_os = "linux")] mod cpu_set;


/// Status, error and non-blocking request support.
#[cfg(target_os = "linux")] pub mod status;


#[cfg(target_os = "linux")] mod ffi_helpers;


#[cfg(target_os = "linux")] mod handle_drop_safeties;


/// Remotely accessible information.
#[cfg(target_os = "linux")] pub mod remotely_accessible;


/// Supportfor InfiniBand sockets and related structures.
#[cfg(target_os = "linux")] pub mod sockets;


/// Print information helpers.
#[cfg(target_os = "linux")] pub mod print_information;


#[cfg(target_os = "linux")] include!("ApplicationContext.rs");
#[cfg(target_os = "linux")] include!("callback_is_ignored.rs");
#[cfg(target_os = "linux")] include!("EndPoint.rs");
#[cfg(target_os = "linux")] include!("MemoryAdvice.rs");
#[cfg(target_os = "linux")] include!("OurLocalMemoryAddressToMakeRemotelyAccessible.rs");
#[cfg(target_os = "linux")] include!("TheirRemotelyAccessibleEndPointAddress.rs");
#[cfg(target_os = "linux")] include!("TheirRemotelyAccessibleMemory.rs");
#[cfg(target_os = "linux")] include!("TheirRemotelyAccessibleMemoryAddress.rs");
#[cfg(target_os = "linux")] include!("TheirRemotelyAccessibleServerAddress.rs");
#[cfg(target_os = "linux")] include!("TheirRemotelyAccessibleWorkerAddress.rs");
#[cfg(target_os = "linux")] include!("OurRemotelyAccessibleMemory.rs");
#[cfg(target_os = "linux")] include!("OurRemotelyAccessibleMemoryAddress.rs");
#[cfg(target_os = "linux")] include!("OurRemotelyAccessibleWorkerAddress.rs");
#[cfg(target_os = "linux")] include!("Worker.rs");
#[cfg(target_os = "linux")] include!("ZeroBasedHyperThreadIndex.rs");
