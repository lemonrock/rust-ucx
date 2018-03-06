// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use configuration::domain::DeviceName;
use configuration::domain::DeviceName::all as AllDevices;
use configuration::domain::TransportLayerCollectionName;
use configuration::domain::TransportLayerCollectionName::all as AllTransportLayers;
use configuration::domain::MemoryAllocatorPriority;
use configuration::domain::MemoryAllocatorPriority::*;
use configuration::domain::MemoryDomain;
use configuration::domain::MemoryUnit;
use configuration::domain::MemoryUnit::*;
use configuration::domain::AtomicOperationsSynchronizationMode;
use configuration::domain::AtomicOperationsSynchronizationMode::*;
use ::indexmap::IndexSet;
use ::std::collections::HashSet;
use ::std::hash::Hash;
use ::std::ffi::CStr;
use ::std::ffi::CString;


include!("ConfigurationSetting.rs");
include!("ConfigurationValueConverter.rs");
include!("ConfigurationValueJoin.rs");
