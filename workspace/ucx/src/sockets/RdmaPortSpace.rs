// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Constants are prefixed with `RDMA_PS_` in C-land.
#[repr(u16)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RdmaPortSpace
{
	/// IP over InfiniBand.
	/// Very similar to `UDP` below.
	IPOIB = 0x0002,
	
	/// TCP-alike over InfiniBand.
	TCP = 0x0106,
	
	/// UDP-alike over InfiniBand.
	UDP = 0x0111,
	
	/// Just InfiniBand.
	IB = 0x013F,
}

impl RdmaPortSpace
{
	#[inline(always)]
	fn shifted_to_big_endian(self) -> BigEndian<u64>
	{
		BigEndian::from_native_endian_value(((self as u64) << 16) as u64)
	}
}
