// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use self::events::*;
use super::configuration::values::UcsGlobalLogLevelSetting;
use super::ffi_helpers::*;
use super::status::*;
use ::libc::c_void;
use ::std::ptr::NonNull;
use ::std::mem::transmute;
use ::ucx_sys::*;


/// Definitions of events.
pub mod events;


/// Functions to map, unmap, etc memory and trigger or not trigger memory events.
///
/// Use `MemoryEventConfiguration` to add and remove memory event handlers.
pub mod memory_functions;


include!("MemoryEventConfiguration.rs");
include!("MemoryEventHandler.rs");
include!("MemoryEventUser.rs");
include!("MemoryEventHandlerPriority.rs");
include!("UcmGlobalConfiguration.rs");
