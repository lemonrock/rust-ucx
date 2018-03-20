// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RemoteAddress(pub u64);

impl RemoteAddress
{
	#[inline(always)]
	pub(crate) fn debug_assert_is_32_bit_aligned(&self)
	{
		debug_assert_eq!(self.0 % 4, 0, "aligned_remote_address '{}' is not 32-bit (4-byte) aligned", self.0)
	}
	
	#[inline(always)]
	pub(crate) fn debug_assert_is_64_bit_aligned(&self)
	{
		debug_assert_eq!(self.0 % 8, 0, "aligned_remote_address '{}' is not 64-bit (8-byte) aligned", self.0)
	}
}
