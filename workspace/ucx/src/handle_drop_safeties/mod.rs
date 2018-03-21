// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use ::std::ptr::NonNull;
use ::std::rc::Rc;
use ::std::sync::Arc;
use ::ucx_sys::*;


include!("ApplicationContextHandleDropSafety.rs");
include!("AsynchronousContextHandleDropSafety.rs");
include!("CommunicationInterfaceContextHandleDropSafety.rs");
include!("MemoryDomainHandleDropSafety.rs");
include!("OurRemotelyAccessibleMemoryHandleDropSafety.rs");
include!("ProgressEngineHandleDropSafety.rs");
include!("WorkerHandleDropSafety.rs");
