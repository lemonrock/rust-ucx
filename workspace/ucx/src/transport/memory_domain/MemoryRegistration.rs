// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A memory region registered using a `MemoryDomain` for zero-copy get, put and atomic memory access.
#[derive(Debug)]
pub struct MemoryRegistration
{
	handle: NonNull<c_void>,
	handle_drop_safety: Arc<MemoryRegistrationHandleDropSafety>,
	address: NonNull<u8>,
	length: usize,
	packed_memory_key: Vec<u8>,
}

impl HasMemoryKey for MemoryRegistration
{
	#[inline(always)]
	fn packed_memory_key(&self) -> &[u8]
	{
		self.packed_memory_key.as_slice()
	}
	
	#[inline(always)]
	fn zero_copy_io_vector(&self, offset: usize, length: usize) -> ZeroCopyIoVector
	{
		debug_assert_ne!(length, 0, "length can not be zero");
		debug_assert!(offset <= self.length, "offset '{}' is equal to or greater than self.length '{}'", offset, self.length);
		debug_assert!(offset + length <= self.length, "offset '{}' + length '{}' is equal to or greater than self.length '{}'", offset, length, self.length);
		
		let address = unsafe { NonNull::new_unchecked(((self.address.as_ptr() as usize) + offset) as *mut u8) };
		ZeroCopyIoVector::new(address, length, self.handle)
	}
}

impl ByteBuffer for MemoryRegistration
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
