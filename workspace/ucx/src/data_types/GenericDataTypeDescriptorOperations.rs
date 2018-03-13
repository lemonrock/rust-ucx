// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Trait to abstract away functionality required by UCX.
pub trait GenericDataTypeDescriptorOperations
{
	/// Serializer type.
	type Serializer: GenericDataTypeDescriptorOperationsSerializer;
	
	/// Deserializer type.
	type Deserializer: GenericDataTypeDescriptorOperationsDeserializer;
	
	/// Starts serializing ("packing").
	///
	/// The `UcxAllocatedByteBuffer` does not free memory when dropped.
	#[inline(always)]
	fn start_serialization(&self, buffer: UcxAllocatedByteBuffer) -> Box<Self::Serializer>;
	
	/// Starts serializing ("packing").
	///
	/// The `UcxAllocatedByteBuffer` does not free memory when dropped.
	#[inline(always)]
	fn start_deserialization(&self, buffer: UcxAllocatedByteBuffer) -> Box<Self::Deserializer>;
}
