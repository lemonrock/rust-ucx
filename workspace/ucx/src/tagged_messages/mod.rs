// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use self::data_type_descriptors::*;
use super::Worker;
use super::buffers::ByteBuffer;
use super::status::*;
use super::status::non_blocking_requests::*;
use ::std::error::Error;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::mem::size_of;
use ::std::ops::Deref;
use ::std::ptr::NonNull;
use ::std::sync::Arc;
use ::ucx_sys::*;


/// Data types descriptors are used for describing data sent in tagged tagged_messages and streams.
///
/// A data type descriptor lets UCX manage serialization and deserialization.
pub mod data_type_descriptors;


include!("ContiguousMessage.rs");
include!("ErrorCodeWithMessage.rs");
include!("GenericMessage.rs");
include!("IoVec.rs");
include!("IoVecMessage.rs");
include!("Message.rs");
include!("SendingTaggedMessageNonBlockingRequest.rs");
