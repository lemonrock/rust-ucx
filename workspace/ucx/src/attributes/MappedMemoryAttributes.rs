// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Mapped memory attributes.
#[derive(Debug)]
pub struct MappedMemoryAttributes(ucp_mem_attr_t);

impl PartialEq for MappedMemoryAttributes
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.address() == other.address() && self.length() == other.length()
	}
}

impl Eq for MappedMemoryAttributes
{
}

impl PartialOrd for MappedMemoryAttributes
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for MappedMemoryAttributes
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.address().cmp(&other.address()).then(self.length().cmp(&other.length()))
	}
}

impl Hash for MappedMemoryAttributes
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.address().hash(state);
		self.length().hash(state);
	}
}

impl MappedMemoryAttributes
{
	/// Mapped memory address.
	#[inline(always)]
	pub fn address(&self) -> *mut u8
	{
		self.0.address as *mut u8
	}
	
	/// Mapped memory length.
	#[inline(always)]
	pub fn length(&self) -> usize
	{
		self.0.length
	}
	
	#[inline(always)]
	pub(crate) fn query(&self, handle: ucp_mem_h) -> Self
	{
		let mut attributes: ucp_mem_attr_t = unsafe { uninitialized() };
		attributes.field_mask = (ucp_mem_attr_field::ADDRESS | ucp_mem_attr_field::LENGTH).0 as u64;
		
		panic_on_error!(ucp_mem_query, handle, &mut attributes);
		MappedMemoryAttributes(attributes)
	}
}
