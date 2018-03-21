// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Attributes of a `MemoryDomain`.
#[derive(Debug)]
pub struct MemoryDomainAttributes(uct_md_attr);

impl Clone for MemoryDomainAttributes
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		unsafe
		{
			let mut clone = MemoryDomainAttributes(uninitialized());
			copy_nonoverlapping(&self.0, &mut clone.0, 1);
			clone
		}
	}
}

impl MemoryDomainAttributes
{
	/// Packed memory key length.
	pub fn packed_memory_key_length(&self) -> usize
	{
		self.0.rkey_packed_size
	}
	
	/// Is this feature supported?
	#[inline(always)]
	pub fn supports_feature(&self, feature: _bindgen_ty_1) -> bool
	{
		_bindgen_ty_1(self.0.cap.flags as u32) & feature == feature
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
