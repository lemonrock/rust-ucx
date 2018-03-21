// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Memory region address allocation request.
///
/// Default is to allocate anywhere.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MemoryRegionAddressAllocationRequest
{
	/// Allocate wherever is possible.
	Any,
	
	/// Try to allocate at this address.
	Hint
	{
		/// The hinted at address.
		address: NonNull<u8>,
	},
	
	/// Allocate at this address or fail.
	Fixed
	{
		/// The fixed address.
		address: NonNull<u8>,
	}
}

impl Default for MemoryRegionAddressAllocationRequest
{
	#[inline(always)]
	fn default() -> Self
	{
		MemoryRegionAddressAllocationRequest::Any
	}
}

impl MemoryRegionAddressAllocationRequest
{
	#[inline(always)]
	fn for_allocate(&self, flags: uct_md_mem_flags) -> (*mut c_void, uct_md_mem_flags)
	{
		use self::MemoryRegionAddressAllocationRequest::*;
		
		match *self
		{
			Any => (null_mut(), flags),
			
			Hint { address } => (address.as_ptr() as *mut _, flags),
			
			Fixed { address } => (address.as_ptr() as *mut _, flags | uct_md_mem_flags::FIXED),
		}
	}
	
	#[inline(always)]
	fn is_fixed(&self) -> bool
	{
		use self::MemoryRegionAddressAllocationRequest::*;
		
		match *self
		{
			Fixed { .. } => true,
			
			_ => false,
		}
	}
}
