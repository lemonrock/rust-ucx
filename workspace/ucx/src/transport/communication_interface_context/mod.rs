// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::ActiveMessageIdentifier;
use super::CompletionHandleHelper;
use super::super::ZeroBasedHyperThreadIndex;
use super::super::attributes::*;
use super::super::buffers::*;
use super::super::cpu_set::*;
use super::super::ffi_helpers::*;
use super::super::handle_drop_safeties::*;
use super::super::local_to_remote_address_translations::*;
use super::super::status::*;
use super::super::sockets::*;
use super::super::tagged_messages::*;
use super::progress_engine::ProgressEngine;
use super::progress_engine::ProgressCallback;
use super::progress_engine::ProgressCallbackCancel;
use super::memory_domains::*;
use self::active_message_handlers::*;
use self::active_message_tracers::*;
use self::error_handlers::*;
use self::server_connection_requests::*;
use self::unexpected_tagged_message_handlers::*;
use ::libc::c_char;
use ::libc::c_uint;
use ::libc::c_void;
use ::libc::strnlen;
use ::std::borrow::Cow;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::os::unix::io::RawFd;
use ::std::ops::Deref;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::NonNull;
use ::std::ptr::null;
use ::std::ptr::null_mut;
use ::std::ptr::write;
use ::std::slice::from_raw_parts;
use ::std::slice::from_raw_parts_mut;
use ::std::sync::Arc;
use ::ucx_sys::*;


/// Active message handlers.
pub mod active_message_handlers;


/// Active message tracers.
pub mod active_message_tracers;


/// Error handlers.
pub mod error_handlers;


/// Server connection requests.
pub mod server_connection_requests;


/// Unexpected tagged message handlers.
pub mod unexpected_tagged_message_handlers;


include!("AvailableCommunicationInterface.rs");
include!("AvailableCommunicationInterfaces.rs");
include!("CommunicationInterfaceConfiguration.rs");
include!("CommunicationInterfaceContext.rs");
include!("CommunicationInterfaceContextEndPointAddress.rs");
include!("DeviceAddress.rs");
include!("DeviceKind.rs");
include!("InterfaceAddress.rs");
include!("InterfaceFeaturesSupported.rs");
