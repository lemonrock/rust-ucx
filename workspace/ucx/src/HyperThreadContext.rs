// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An application context is suitable for use on a particular hyper thread.
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct HyperThreadContext
{
	handle: ucp_context_h,
	hyper_thread_context_drop_safety: HyperThreadContextDropSafety,
}

impl Debug for HyperThreadContext
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		self.debug_fmt(f)
	}
}

impl PrintInformation for HyperThreadContext
{
	const DebugName: &'static str = "ApplicationContext";
	
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		unsafe { ucp_context_print_info(self.handle, stream) };
	}
}

impl HasAttributes for HyperThreadContext
{
	type Attributes = HyperThreadContextAttributes;
	
	#[inline(always)]
	fn attributes(&self) -> Self::Attributes
	{
		Self::Attributes::query(self.handle)
	}
}
