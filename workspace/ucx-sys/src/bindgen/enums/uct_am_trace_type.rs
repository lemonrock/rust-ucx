// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum uct_am_trace_type
{
	UCT_AM_TRACE_TYPE_SEND = 0,
	UCT_AM_TRACE_TYPE_RECV = 1,
	UCT_AM_TRACE_TYPE_SEND_DROP = 2,
	UCT_AM_TRACE_TYPE_RECV_DROP = 3,
	UCT_AM_TRACE_TYPE_LAST = 4,
}

impl BitOr<uct_cb_param_flags> for uct_cb_param_flags
{
	type Output = Self;
	
	fn bitor(self, other: Self) -> Self
	{
		uct_cb_param_flags(self.0 | other.0)
	}
}
