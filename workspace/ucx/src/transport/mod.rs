// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::configuration::values::UcsGlobalLogLevelSetting;
use super::configuration::values::SignalNumber;
use super::ffi_helpers::*;
use super::handle_drop_safeties::*;
use super::print_information::PrintInformation;
use super::status::*;
use ::libc::FILE;
use ::std::ffi::CString;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::slice::from_raw_parts;
use ::std::sync::Arc;
use ::ucx_sys::*;


/// Communication interface context and supporting logic.
///
/// Used to create remote end points.
///
/// Used to receive active messages and tagged messages from remote peers.
pub mod communication_interface_context;


/// Memory domains needed to create communication interface contexts.
pub mod memory_domain;


/// A progress engine is used to progress non-blocking operations.
pub mod progress_engine;


/// A remote end point is used to send data and perform remote memory operations.
pub mod remote_end_point;


include!("ActiveMessageIdentifier.rs");
include!("AsynchronousContext.rs");
include!("CompletionHandleHelper.rs");
include!("UcsGlobalConfiguration.rs");
