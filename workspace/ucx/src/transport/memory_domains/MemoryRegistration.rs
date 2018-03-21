// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A memory region registered using a `MemoryDomain` for zero-copy get, put and atomic memory access.
#[derive(Debug)]
pub struct MemoryRegistration
{
	memory_domain_handle: NonNull<uct_md>,
	memory_domain_drop_safety: Arc<MemoryDomainDropSafety>,
	address: NonNull<u8>,
	length: usize,
	memory_region_handle: uct_mem_h,
}

impl Drop for MemoryRegistration
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if !self.memory_region_handle.is_null()
		{
			let status = unsafe { uct_md_mem_dereg(self.memory_domain_handle.as_ptr(), self.memory_region_handle) };
			if !status.is_ok()
			{
				panic!("Unexpected status '{:?}'", status.parse())
			}
		}
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

