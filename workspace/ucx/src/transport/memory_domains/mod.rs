// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::super::handle_drop_safeties::MemoryDomainDropSafety;
use super::super::status::*;
use super::communication_interface_context::AvailableCommunicationInterfaces;
use ::libc::c_char;
use ::libc::c_void;
use ::libc::strnlen;
use ::std::borrow::Cow;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::mem::uninitialized;
use ::std::mem::transmute;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::slice::from_raw_parts;
use ::std::sync::Arc;
use ::ucx_sys::*;


include!("AvailableMemoryDomainComponent.rs");
include!("AvailableMemoryDomainComponents.rs");
include!("MemoryDomain.rs");
include!("MemoryDomainComponent.rs");
include!("MemoryDomainComponentConfiguration.rs");
