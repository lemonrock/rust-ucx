// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::buffers::*;
use super::status::*;
use ::libc::c_void;
use ::std::mem::forget;
use ::std::mem::uninitialized;
use ::ucx_sys::*;


include!("ContiguousDataTypeDescriptor.rs");
include!("DataTypeDescriptor.rs");
include!("GenericDataTypeDescriptor.rs");
include!("GenericDataTypeDescriptorOperations.rs");
include!("GenericDataTypeDescriptorOperationsDeserializer.rs");
include!("GenericDataTypeDescriptorOperationsSerializer.rs");
include!("StridedDataTypeDescriptor.rs");
include!("TagForLowestThreeBits.rs");
