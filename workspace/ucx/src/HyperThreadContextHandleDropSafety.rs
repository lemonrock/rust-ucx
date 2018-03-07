// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


// We use this rather than force the public API to deal with `Rc<HyperThreadContext>`.
// This also has the benefit of eliminating a pointer dereference to get to `handle: ucp_context_h`, as we do not need to got through `Rc::deref()`.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct HyperThreadContextHandleDropSafety(pub(crate) ucp_context_h);

impl Drop for HyperThreadContextHandleDropSafety
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_cleanup(self.0) };
	}
}
