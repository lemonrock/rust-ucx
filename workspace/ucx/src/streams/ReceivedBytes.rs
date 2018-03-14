// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Bytes received from a stream.
#[derive(Debug)]
pub struct ReceivedBytes<E: EndPointPeerFailureErrorHandler>
{
	address: NonNull<u8>,
	length: usize,
	parent_end_point: Rc<RefCell<TheirRemotelyAccessibleEndPoint<E, TheirRemotelyAccessibleServerEndPointAddress>>>,
}

impl<E: EndPointPeerFailureErrorHandler> Drop for ReceivedBytes<E>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_stream_data_release(self.parent_end_point.borrow().debug_assert_handle_is_valid(), self.address.as_ptr() as *mut _) }
	}
}

impl<E: EndPointPeerFailureErrorHandler> ByteBuffer for ReceivedBytes<E>
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

impl<E: EndPointPeerFailureErrorHandler> ReceivedBytes<E>
{
	#[inline(always)]
	pub(crate) fn new(address: *mut u8, length: usize, parent_end_point: &Rc<RefCell<TheirRemotelyAccessibleEndPoint<E, TheirRemotelyAccessibleServerEndPointAddress>>>) -> Self
	{
		debug_assert!(!address.is_null(), "address is null");
		
		Self
		{
			address: unsafe { NonNull::new_unchecked(address) },
			length,
			parent_end_point: parent_end_point.clone(),
		}
	}
}
