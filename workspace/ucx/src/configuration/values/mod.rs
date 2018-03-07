// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::super::ucx_services::ZeroBasedHyperThreadIndex;
use super::super::errors::Status::*;
use super::super::errors::ErrorCode::*;
use super::super::HyperThreadContext;
use super::non_blocking_request_memory_customization::NonBlockingRequestMemoryCustomization;
use super::configuration_settings::*;
use super::*;
use ::serde::de::Deserializer;
use ::serde::de::Deserialize;
use ::serde::de::Error as DeserializeError;
use ::serde::de::Visitor;
use ::std::borrow::Cow;
use ::std::collections::HashMap;
use ::std::ffi::CString;
use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::mem::transmute;
use ::std::mem::uninitialized;


include!("ApplicationContextConfiguration.rs");
include!("AtomicOperationsSynchronizationMode.rs");
include!("DeviceName.rs");
include!("HyperThreadsApplicationContextConfigurations.rs");
include!("RequestedFeatures.rs");
include!("MemoryAllocatorPriority.rs");
include!("MemoryDomain.rs");
include!("MemoryUnit.rs");
include!("TagSenderMask.rs");
include!("TransportLayerCollectionName.rs");
