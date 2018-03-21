// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An end point address.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EndPointAddress(Vec<u8>);

impl ByteBuffer for EndPointAddress
{
	#[inline(always)]
	fn address(&self) -> NonNull<u8>
	{
		unsafe { NonNull::new_unchecked(self.0.as_ptr() as *mut _) }
	}
	
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.0.len()
	}
}

impl EndPointAddress
{
	#[inline(always)]
	pub(crate) fn new(length: usize) -> Self
	{
		let mut bytes = Vec::with_capacity(length);
		unsafe { bytes.set_len(length) };
		EndPointAddress(bytes)
	}
	
	#[inline(always)]
	pub(crate) fn is_reachable_address(&self) -> *const uct_ep_addr
	{
		if self.0.is_empty()
		{
			null()
		}
		else
		{
			self.0.as_ptr() as *const _
		}
	}
}
