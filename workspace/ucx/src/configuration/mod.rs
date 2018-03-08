// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use self::non_blocking_request_memory_customization::*;
use self::configuration_settings::*;
use self::values::*;
use self::wrappers::*;
use super::*;

/// Domain.
pub mod values;


/// Configuration settings.
pub mod configuration_settings;


/// Non-blocking request memory customization.
pub mod non_blocking_request_memory_customization;


/// Configuration wrappers.
pub mod wrappers;


include!("ApplicationContextCreationError.rs");
include!("CouldNotConfigureUcpError.rs");
include!("Configuration.rs");
