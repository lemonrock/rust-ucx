// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A new type to ease access to the underlying end point (which is provided as a combination of handle and user data).
#[derive(Debug)]
pub struct EndPointReadyToConsumeStreamingData(ucp_stream_poll_ep_t);

impl EndPointReadyToConsumeStreamingData
{
	/// Gets the end point.
	#[inline(always)]
	pub fn end_point<E: EndPointPeerFailureErrorHandler, A: TheirRemotelyAccessibleEndPointAddress>(&self) -> Option<Rc<RefCell<EndPoint<E, A>>>>
	{
		EndPoint::end_point_from_user_data(self.0.user_data, self.0.ep)
	}
}
