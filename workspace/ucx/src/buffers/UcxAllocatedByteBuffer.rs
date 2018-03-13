// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// The `UcxAllocatedByteBuffer` does not free memory when dropped; it acts as a wrapper around a generic UCX provided buffer.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UcxAllocatedByteBuffer
{
	address: NonNull<u8>,
	length: usize,
}

impl ByteBuffer for UcxAllocatedByteBuffer
{
	#[inline(always)]
	fn address(&self) -> NonNull<u8>
	{
		self.address
	}
	
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.length
	}
}

impl UcxAllocatedByteBuffer
{
	#[inline(always)]
	pub(crate) fn new(address: *mut c_void, length: usize) -> Self
	{
		debug_assert!(!address.is_null(), "address is null");
		
		Self
		{
			address: unsafe { NonNull::new_unchecked(address as *mut u8) },
			length,
		}
	}
}
