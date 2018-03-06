// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use self::configuration_settings::*;
use super::errors::*;
use super::print_information::PrintInformation;
use ::libc::FILE;
use ::std::ffi::CString;
use ::std::mem::uninitialized;
use ::std::ptr::null;
use ::ucx_sys::*;


/// Domain.
pub mod domain;


/// Configuration settings.
pub mod configuration_settings;


include!("Configuration.rs");
include!("ConfigurationModifyError.rs");
include!("ConfigurationParseError.rs");
