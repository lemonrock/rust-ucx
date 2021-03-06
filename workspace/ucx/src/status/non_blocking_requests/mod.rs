// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::ErrorCode;
use super::Status::*;
use super::ucs_status_tExt;
use super::super::Worker;
use super::super::attributes::ApplicationContextAttributes;
use super::super::streams::StreamLengthOfReceivedDataInBytes;
use super::super::tagged_messages::ReceivedTaggedMessageInformation;
use ::libc::c_void;
use ::std::mem::uninitialized;
use ::std::ptr::NonNull;
use ::ucx_sys::*;


include!("NonBlockingRequest.rs");
include!("UserAllocatedNonBlockingRequest.rs");
include!("UcpAllocatedNonBlockingRequest.rs");
