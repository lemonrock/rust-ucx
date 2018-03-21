// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Mapped memory attributes.
#[derive(Debug)]
pub struct OurRemotelyAccessibleMemoryAttributes(ucp_mem_attr);

impl Clone for OurRemotelyAccessibleMemoryAttributes
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		unsafe
		{
			let mut clone = OurRemotelyAccessibleMemoryAttributes(uninitialized());
			copy_nonoverlapping(&self.0, &mut clone.0, 1);
			clone
		}
	}
}

impl PartialEq for OurRemotelyAccessibleMemoryAttributes
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.address() == other.address() && self.length() == other.length()
	}
}

impl Eq for OurRemotelyAccessibleMemoryAttributes
{
}

impl PartialOrd for OurRemotelyAccessibleMemoryAttributes
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for OurRemotelyAccessibleMemoryAttributes
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.address().cmp(&other.address()).then(self.length().cmp(&other.length()))
	}
}

impl Hash for OurRemotelyAccessibleMemoryAttributes
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.address().hash(state);
		self.length().hash(state);
	}
}

impl OurRemotelyAccessibleMemoryAttributes
{
	/// Mapped memory address.
	#[inline(always)]
	pub fn address(&self) -> NonNull<u8>
	{
		unsafe { NonNull::new_unchecked(self.0.address as *mut u8) }
	}
	
	/// Mapped memory length.
	#[inline(always)]
	pub fn length(&self) -> usize
	{
		self.0.length
	}
	
	#[inline(always)]
	pub(crate) fn query(handle: ucp_mem_h) -> Self
	{
		let mut attributes: ucp_mem_attr_t = unsafe { uninitialized() };
		attributes.field_mask = (ucp_mem_attr_field::ADDRESS | ucp_mem_attr_field::LENGTH).0 as u64;
		
		panic_on_error!(ucp_mem_query, handle, &mut attributes);
		OurRemotelyAccessibleMemoryAttributes(attributes)
	}
}
