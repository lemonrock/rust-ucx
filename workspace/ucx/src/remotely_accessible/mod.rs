// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.



use super::*;
use ::spin_locks::*;
use ::std::cell::UnsafeCell;
use ::std::collections::HashMap;
use ::std::mem::replace;
use ::std::mem::uninitialized;
use ::std::ops::Deref;
use ::std::sync::atomic::fence;
use ::std::sync::atomic::Ordering::SeqCst;


include!("ApplicationContextName.rs");
include!("MasterTheirRemotelyAccessible.rs");
include!("MemoryName.rs");
include!("ServerName.rs");
include!("TheirRemotelyAccessible.rs");
include!("TheirRemotelyAccessibleApplicationContext.rs");
include!("TheirRemotelyAccessibleGuard.rs");
include!("TheirRemotelyAccessibleThreadLocalEntry.rs");
include!("WorkerName.rs");

