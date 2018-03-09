// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::super::status::*;
use super::super::ffi_helpers::*;
use super::super::print_information::PrintInformation;
use super::configuration_settings::*;
use super::values::*;
use ::libc::c_char;
use ::libc::FILE;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::ptr::null;
use ::std::slice::from_raw_parts;
use ::ucx_sys::*;


include!("ConfigurationModifyError.rs");
include!("ConfigurationParseError.rs");
include!("ConfigurationWrapper.rs");
include!("UcmConfigurationWrapper.rs");
include!("UcpConfigurationWrapper.rs");
include!("UcsGlobalConfigurationWrapper.rs");
