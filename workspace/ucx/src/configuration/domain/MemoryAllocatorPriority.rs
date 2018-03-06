// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Memory allocator priority.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryAllocatorPriority
{
	/// Transparent Huge Pages.
	thp,
	
	/// Memory domain.
	md(MemoryDomain),
	
	/// Heap.
	heap,
	
	/// mmap.
	mmap,
	
	/// Huge pages.
	huge,
}

impl ConfigurationValueJoin for MemoryAllocatorPriority
{
	#[inline(always)]
	fn push(&self, string: &mut String)
	{
		string.push_str(self.to_str())
	}
}

impl ConfigurationValueConverter for MemoryAllocatorPriority
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		CString::new(self.to_str()).unwrap()
	}
}

impl MemoryAllocatorPriority
{
	#[inline(always)]
	fn to_str(&self) -> &'static str
	{
		use self::MemoryAllocatorPriority::*;
		
		match *self
		{
			thp => "thp",
			md(ref memory_domain) => memory_domain.to_memory_allocator_priority_str(),
			heap => "heap",
			mmap => "mmap",
			huge => "huge",
		}
	}
}
