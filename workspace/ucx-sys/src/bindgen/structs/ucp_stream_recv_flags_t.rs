// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ucp_stream_recv_flags_t(pub u32);

impl BitOr<ucp_stream_recv_flags_t> for ucp_stream_recv_flags_t
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		ucp_stream_recv_flags_t(self.0 | other.0)
	}
}

impl BitOrAssign for ucp_stream_recv_flags_t
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: ucp_stream_recv_flags_t)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<ucp_stream_recv_flags_t> for ucp_stream_recv_flags_t
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		ucp_stream_recv_flags_t(self.0 & other.0)
	}
}

impl BitAndAssign for ucp_stream_recv_flags_t
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: ucp_stream_recv_flags_t)
	{
		self.0 &= rhs.0;
	}
}

impl ucp_stream_recv_flags_t
{
	pub const WAITALL: Self = ucp_stream_recv_flags_t(1);
}
