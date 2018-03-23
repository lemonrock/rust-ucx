// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct MemoryRegionHandleDropSafety(NonNull<c_void>, NonNull<uct_md>, Arc<MemoryDomainHandleDropSafety>);

impl Drop for MemoryRegionHandleDropSafety
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let status = unsafe { uct_md_mem_free(self.1.as_ptr(), self.0.as_ptr()) };
		if !status.is_ok()
		{
			panic!("Unexpected status '{:?}'", status.parse())
		}
	}
}

impl MemoryRegionHandleDropSafety
{
	#[inline(always)]
	pub(crate) fn new(value: NonNull<c_void>, memory_domain: NonNull<uct_md>, memory_domain_handle_drop_safety: Arc<MemoryDomainHandleDropSafety>) -> Arc<Self>
	{
		Arc::new(MemoryRegionHandleDropSafety(value, memory_domain, memory_domain_handle_drop_safety))
	}
}
