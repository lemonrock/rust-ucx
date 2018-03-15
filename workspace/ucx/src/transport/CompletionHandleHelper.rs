// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


trait CompletionHandleHelper
{
	#[inline(always)]
	fn mutable_reference(self) -> *mut uct_completion;
}

impl<'a> CompletionHandleHelper for Option<&'a mut uct_completion>
{
	#[inline(always)]
	fn mutable_reference(self) -> *mut uct_completion
	{
		match self
		{
			None => null_mut(),
			Some(mutable_reference) => mutable_reference as *mut _
		}
	}
}
