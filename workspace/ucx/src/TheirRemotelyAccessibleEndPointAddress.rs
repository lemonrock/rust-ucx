// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// The address of a remote end point.
pub trait TheirRemotelyAccessibleEndPointAddress
{
	// NOTE: It is important that this instance of `TheirRemotelyAccessibleAddress` is not dropped until after these parameters have been used to create an end point.
	#[doc(hidden)]
	#[inline(always)]
	fn populate_end_point_parameters(&self, end_pointer_parameters: ucp_ep_params_t) -> ucp_ep_params_t;
}
