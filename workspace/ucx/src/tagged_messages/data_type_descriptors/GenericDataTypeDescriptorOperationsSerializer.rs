// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Trait to abstract away functionality required by UCX.
pub trait GenericDataTypeDescriptorOperationsSerializer
{
	/// Type serialized.
	type Serialized;
	
	/// Serialized size.
	#[inline(always)]
	fn serialized_size(&self) -> usize;
	
	/// Serialize.
	///
	/// Returns number of bytes written; this must not exceed `output_buffer.length()`.
	fn serialize(&self, virtual_offset_in_the_output_stream: usize, output_buffer: UcxAllocatedByteBuffer) -> usize;
}
