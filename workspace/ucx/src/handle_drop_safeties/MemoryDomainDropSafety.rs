// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct MemoryDomainDropSafety(NonNull<uct_md>);

impl Drop for MemoryDomainDropSafety
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { uct_md_close(self.0.as_ptr()) }
	}
}

impl MemoryDomainDropSafety
{
	#[inline(always)]
	pub(crate) fn new(value: NonNull<uct_md>) -> Arc<Self>
	{
		Arc::new(MemoryDomainDropSafety(value))
	}
}
