// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Available communication interfaces.
#[derive(Debug)]
pub struct AvailableCommunicationInterfaces
{
	array: *mut uct_tl_resource_desc,
	length: u32,
}

impl Drop for AvailableCommunicationInterfaces
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if !self.array.is_null()
		{
			unsafe { uct_release_tl_resource_list(self.array) }
		}
	}
}

impl AvailableCommunicationInterfaces
{
	/// Query.
	#[inline(always)]
	pub fn query(memory_domain: &MemoryDomain) -> Result<Self, ErrorCode>
	{
		let mut this = Self
		{
			array: null_mut(),
			length: 0
		};
		
		let status = unsafe { uct_md_query_tl_resources(memory_domain.as_ptr(), &mut this.array, &mut this.length) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(this),
			
			Error(error_code) => Err(error_code),
			
			_ => panic!("Unexpected status '{:?}'", status),
		}
	}
	
	/// As a slice.
	#[inline(always)]
	pub fn as_slice<'a>(&'a self) -> &'a [AvailableCommunicationInterface]
	{
		let slice = unsafe { from_raw_parts(self.array as *const _, self.length as usize) };
		unsafe { transmute(slice) }
	}
}
