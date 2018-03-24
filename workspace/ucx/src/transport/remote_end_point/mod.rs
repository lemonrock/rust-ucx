// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::super::attributes::*;
use super::super::ffi_helpers::*;
use super::super::handle_drop_safeties::*;
use super::super::local_to_remote_memory_address_translations::*;
use super::super::sockets::*;
use super::super::status::*;
use super::super::tagged_messages::TagValue;
use super::communication_interface_context::*;
use super::memory_domain::*;
use super::ActiveMessageIdentifier;
use super::Completion;
use super::CompletionHandler;
use ::libc::c_void;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::ops::Deref;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::slice::from_raw_parts;
use ::std::sync::Arc;
use ::ucx_sys::*;


include!("ActiveMessageImmediateHeader.rs");
include!("ActiveMessageSendingRemoteEndPoint.rs");
include!("buffered_copy_deserialize_callback.rs");
include!("buffered_copy_serialize_callback.rs");
include!("BufferedCopyDeserializer.rs");
include!("BufferedCopySerializer.rs");
include!("PendingRequest.rs");
include!("PendingRequestCallWhenPurged.rs");
include!("PendingRequestHandler.rs");
include!("RemoteEndPoint.rs");
include!("RemoteMemoryAccessRemoteEndPoint.rs");
include!("RendezvousCancellation.rs");
include!("TaggedMessageSendingRemoteEndPoint.rs");
