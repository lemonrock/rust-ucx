// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use self::ucs_status_t::*;
use ::std::ffi::CStr;
use ::std::mem::transmute;
use ::ucx_sys::ucs_status_ptr_t;
use ::ucx_sys::ucs_status_string;
use ::ucx_sys::ucs_status_t;


include!("ErrorCode.rs");
include!("InvalidStatusError.rs");
include!("Status.rs");
include!("StatusOrPointer.rs");
include!("ucs_status_tExt.rs");
include!("ucs_status_ptr_tExt.rs");
