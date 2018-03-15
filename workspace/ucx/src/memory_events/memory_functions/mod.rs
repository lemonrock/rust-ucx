// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use ::libc::c_int;
use ::libc::c_void;
use ::libc::intptr_t;
use ::libc::off_t;
use ::libc::size_t;
use ::std::os::unix::io::RawFd;
use ::ucx_sys::*;


include!("AggregatedMemoryFunctionsWhichTriggerEventHandlers.rs");
include!("MemoryFunctionsWhichDoNotTriggerEvents.rs");
include!("MemoryFunctionsWhichTriggerEventHandlers.rs");
