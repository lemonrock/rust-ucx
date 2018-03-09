// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// The address of a remotely accessible server.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TheirRemotelyAccessibleServerAddress(NixSockAddr);

impl TheirRemotelyAccessibleEndPointAddress for TheirRemotelyAccessibleServerAddress
{
	// NOTE: It is important that this instance of `TheirRemoteAddress` is not dropped until after these parameters have been used to create an end point.
	#[inline(always)]
	fn populate_end_point_parameters(&self, mut end_pointer_parameters: ucp_ep_params_t) -> ucp_ep_params_t
	{
		let (socket_address, length) = unsafe { self.0.as_ffi_pair() };
		
		end_pointer_parameters.field_mask |= (ucp_ep_params_field::FLAGS | ucp_ep_params_field::SOCK_ADDR).0 as u64;
		end_pointer_parameters.sockaddr = ucs_sock_addr_t
		{
			addr: socket_address,
			addrlen: length,
		};
		end_pointer_parameters.flags |= UCP_EP_PARAMS_FLAGS_CLIENT_SERVER;
		
		end_pointer_parameters
	}
}