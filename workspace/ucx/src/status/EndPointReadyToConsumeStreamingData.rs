// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A new type wrapper to ease access to the underlying remotely accessible end point.
#[derive(Debug)]
pub struct EndPointReadyToConsumeStreamingData(ucp_stream_poll_ep_t);

impl EndPointReadyToConsumeStreamingData
{
	/// Gets their remotely accessible end point that is ready to consume streaming data.
	#[inline(always)]
	pub fn their_remotely_accessible_end_point<E: EndPointPeerFailureErrorHandler, A: TheirRemotelyAccessibleEndPointAddress>(&self) -> Option<Rc<RefCell<TheirRemotelyAccessibleEndPoint<E, A>>>>
	{
		TheirRemotelyAccessibleEndPoint::end_point_from_user_data(self.0.user_data, self.0.ep)
	}
}
