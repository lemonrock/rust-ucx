// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::buffers::*;
use super::status::*;
use ::libc::c_void;
use ::std::cell::UnsafeCell;
use ::std::fmt::Debug;
use ::std::mem::forget;
use ::std::mem::size_of;
use ::std::sync::Arc;
use ::ucx_sys::*;


/// Types of message associated with data type descriptors.
///
/// Each data type descriptor has a particular implementation or implementations of a message.
pub mod messages;


include!("ContiguousDataTypeDescriptor.rs");
include!("DataTypeDescriptor.rs");
include!("GenericDataTypeDescriptor.rs");
include!("GenericDataTypeDescriptorOperations.rs");
include!("GenericDataTypeDescriptorOperationsDeserializer.rs");
include!("GenericDataTypeDescriptorOperationsSerializer.rs");
include!("IoVecDataTypeDescriptor.rs");
include!("TagForLowestThreeBits.rs");
