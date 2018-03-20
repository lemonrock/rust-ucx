// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Attributes of a `CommunicationInterfaceContext`.
#[derive(Debug)]
pub struct MemoryDomainAttributes(uct_md_attr);

impl MemoryDomainAttributes
{
	/// Packed memory key length.
	pub fn packed_memory_key_length(&self) -> usize
	{
		self.0.rkey_packed_size
	}
	
	#[inline(always)]
	pub(crate) fn query(memory_domain: NonNull<uct_md>) -> Self
	{
		let mut attributes = unsafe { uninitialized() };
		
		let status = unsafe { uct_md_query(memory_domain.as_ptr(), &mut attributes) };
		
		if status.is_ok()
		{
			MemoryDomainAttributes(attributes)
		}
		else
		{
			panic!("Unexpected status '{:?}'", status.parse());
		}
	}
}
