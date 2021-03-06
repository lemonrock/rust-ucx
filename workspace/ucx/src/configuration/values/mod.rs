// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::super::status::Status::*;
use super::super::status::ErrorCode::*;
use super::super::ApplicationContext;
use super::non_blocking_request_memory_customization::NonBlockingRequestMemoryCustomization;
use super::*;
use ::ring::aead::AES_128_GCM;
use ::ring::aead::AES_256_GCM;
use ::ring::aead::CHACHA20_POLY1305;
use ::ring::aead::OpeningKey;
use ::ring::aead::SealingKey;
use ::serde::de::Deserializer;
use ::serde::de::Deserialize;
use ::serde::de::Error as DeserializeError;
use ::serde::de::Visitor;
use ::std::borrow::Cow;
use ::std::ffi::CString;
use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::marker::PhantomData;
use ::std::mem::transmute;
use ::std::mem::uninitialized;


include!("ApplicationContextConfiguration.rs");
include!("AtomicOperationsSynchronizationMode.rs");
include!("DeviceName.rs");
include!("RequestedFeatures.rs");
include!("MemoryAllocatorPriority.rs");
include!("MemoryDomain.rs");
include!("MemoryUnit.rs");
include!("SecretKeyBytes.rs");
include!("SignalNumber.rs");
include!("TagSenderMask.rs");
include!("TransportLayerCollectionName.rs");
include!("UcsGlobalLogLevelSetting.rs");
include!("WakeUp.rs");
