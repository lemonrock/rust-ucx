// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use self::non_blocking_requests::*;
pub use self::NonBlockingRequestCompletedOrInProgress::*;
use super::TheirRemotelyAccessibleEndPoint;
use super::TheirRemotelyAccessibleEndPointAddress;
use super::Worker;
use self::ucs_status_t::*;
use ::std::cell::RefCell;
use ::std::ffi::CStr;
use ::std::mem::transmute;
use ::std::ops::Deref;
use ::std::ptr::NonNull;
use ::std::rc::Rc;
use ::ucx_sys::*;


/// Models the slightly different kinds of non-blocking request used by UCX.
pub mod non_blocking_requests;


include!("EndPointPeerFailureErrorHandler.rs");
include!("EndPointReadyToConsumeStreamingData.rs");
include!("ErrorCode.rs");
include!("NonBlockingRequestCompletedOrInProgress.rs");
include!("Status.rs");
include!("StatusOrUcxAllocatedNonBlockingRequest.rs");
include!("ucs_status_tExt.rs");
include!("ucs_status_ptr_tExt.rs");
include!("WorkerWithNonBlockingRequest.rs");
