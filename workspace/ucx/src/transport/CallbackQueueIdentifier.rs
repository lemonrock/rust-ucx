// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An unique identifier for a callback.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CallbackQueueIdentifier(i32);

impl CallbackQueueIdentifier
{
	pub(crate) fn new(identifier: i32) -> Self
	{
		debug_assert_ne!(identifier, UCS_CALLBACKQ_ID_NULL, "identifier can not be UCS_CALLBACKQ_ID_NULL");
		CallbackQueueIdentifier(identifier)
	}
}
