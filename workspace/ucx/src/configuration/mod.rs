// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use self::configuration_settings::*;
use self::non_blocking_request_memory_customization::*;
use self::values::*;
use super::*;
use super::print_information::PrintInformation;
use ::libc::FILE;
use ::std::ffi::CString;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::mem::uninitialized;
use ::std::ptr::null;

/// Domain.
pub mod values;


/// Configuration settings.
pub mod configuration_settings;


/// Non-blocking request memory customization.
pub mod non_blocking_request_memory_customization;


include!("ApplicationContextCreationError.rs");
include!("ConfigurationModifyError.rs");
include!("ConfigurationParseError.rs");
include!("CouldNotConfigureUcxError.rs");
include!("Configuration.rs");
include!("UcxConfigurationWrapper.rs");
