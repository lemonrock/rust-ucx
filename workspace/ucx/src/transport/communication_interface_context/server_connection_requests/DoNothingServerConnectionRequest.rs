// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Does nothing (actually, always fails connection requests).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DoNothingServerConnectionRequest;

impl ServerConnectionRequest for DoNothingServerConnectionRequest
{
	#[inline(always)]
	fn connection_request(&self, _connection_private_data_from_uct_ep_create_sockaddr: UcxAllocatedByteBuffer) -> bool
	{
		false
	}
	
	/// SYNC, ASYNC or both?
	#[inline(always)]
	fn connection_callback_flags(&self) -> uct_cb_flags
	{
		uct_cb_flags::SYNC | uct_cb_flags::ASYNC
	}
}
