// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A callback for UCT to invoke when it is ready.
///
/// Either `invoke` or `purged` is called just once.
pub trait PendingRequestHandler
{
	/// Called back when UCT is ready.
	#[inline(always)]
	fn invoke(&self) -> Result<(), ErrorCode>;
	
	/// Called if purged
	#[inline(always)]
	fn purged(&self);
}

impl PendingRequestHandler for ()
{
	#[inline(always)]
	fn invoke(&self) -> Result<(), ErrorCode>
	{
		Ok(())
	}
	
	#[inline(always)]
	fn purged(&self)
	{
	}
}
