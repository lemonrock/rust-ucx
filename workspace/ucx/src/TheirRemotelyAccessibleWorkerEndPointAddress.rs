// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// The address of a remotely accessible worker.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TheirRemotelyAccessibleWorkerEndPointAddress(NonNull<ucp_address_t>);

impl TheirRemotelyAccessibleEndPointAddress for TheirRemotelyAccessibleWorkerEndPointAddress
{
	// NOTE: It is important that this instance of `TheirRemoteAddress` is not dropped until after these parameters have been used to create an end point.
	#[inline(always)]
	fn populate_end_point_parameters(&self, mut end_pointer_parameters: ucp_ep_params_t) -> ucp_ep_params_t
	{
		end_pointer_parameters.field_mask |= ucp_ep_params_field::REMOTE_ADDRESS.0 as u64;
		end_pointer_parameters.address = self.0.as_ptr();
		end_pointer_parameters
	}
}
