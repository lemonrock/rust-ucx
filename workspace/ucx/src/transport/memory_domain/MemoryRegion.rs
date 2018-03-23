// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A memory region allocated using a `MemoryDomain`.
#[derive(Debug)]
pub struct MemoryRegion
{
	handle: NonNull<c_void>,
	handle_drop_safety: Arc<MemoryRegionHandleDropSafety>,
	address: NonNull<u8>,
	length: usize,
	packed_memory_key: Vec<u8>,
	memory_domain_handle: NonNull<uct_md>,
	#[cfg(debug_assertions)] was_allocated_non_blocking: bool,
	#[cfg(debug_assertions)] memory_advice_is_supported: bool,
}

impl HasMemoryKey for MemoryRegion
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

impl ByteBuffer for MemoryRegion
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

impl MemoryRegion
{
	/// Advise UCT how to handle this part of the memory region for performance.
	///
	/// Equivalent to `uct_md_mem_advise`.
	///
	/// Advice can be ignored.
	///
	/// Advice is currently limited to telling the OS that non-blocking memory not yet page-mapped will be needed.
	/// (This is being enforced during debug).
	///
	/// Advice requires that the memory domain support `_bindgen_ty_1::ADVISE`; currently only InfiniBand and RdmaConnectionManager based memory domains do.
	///
	/// `length` can not be zero (0).
	#[inline(always)]
	pub fn advise(&self, offset: usize, length: usize, will_be_needed: bool) -> Result<(), ErrorCode>
	{
		#[cfg(debug_assertions)]
		{
			debug_assert!(self.memory_advice_is_supported, "memory advice is not supported by the underlying memory domain");
			
			if will_be_needed && !self.was_allocated_non_blocking
			{
				panic!("This memory was allocated non-blocking, and so can not be advised to be 'will_be_needed'");
			}
		}
		
		debug_assert_ne!(length, 0, "length is zero (0)");
		debug_assert!(offset < self.length, "offset '{}' equals or exceeds length '{}'", offset, self.length);
		
		let address_as_usize = self.address.as_ptr() as usize;
		let address = address_as_usize + offset;
		debug_assert!(address < address_as_usize + self.length, "offset exceeds memory region addresses");
		
		let advice = if will_be_needed
		{
			uct_mem_advice_t::UCT_MADV_WILLNEED
		}
		else
		{
			uct_mem_advice_t::UCT_MADV_NORMAL
		};
		
		let status = unsafe { uct_md_mem_advise(self.memory_domain_handle.as_ptr(), self.handle.as_ptr(), address as *mut c_void, length, advice) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(()),
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
}
