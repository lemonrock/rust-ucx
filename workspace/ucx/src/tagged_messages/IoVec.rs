// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Similar to, but easier to work with, than `ucp_dt_iov_t`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct IoVec
{
	/// Address
	pub address: NonNull<u8>,
	
	/// Length
	pub length: usize,
}

impl ByteBuffer for IoVec
{
	#[inline(always)]
	fn address(&self) -> NonNull<u8>
	{
		self.address
	}
	
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.length
	}
}

impl IoVec
{
	/// Create a new new instance.
	pub fn new(address: NonNull<u8>, length: usize) -> Self
	{
		Self
		{
			address,
			length,
		}
	}
}
