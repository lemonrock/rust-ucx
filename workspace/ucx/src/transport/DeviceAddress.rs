// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DeviceAddress(Vec<u8>);

impl DeviceAddress
{
	#[inline(always)]
	pub(crate) fn new(length: usize) -> Self
	{
		let mut bytes = Vec::with_capacity(length);
		unsafe { bytes.set_len(length) };
		DeviceAddress(bytes)
	}
}

impl ByteBuffer for DeviceAddress
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

impl DeviceAddress
{
	#[inline(always)]
	pub(crate) fn is_reachable_address(&self) -> *const uct_device_addr
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
