// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// The address of a remotely accessible server.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TheirRemotelyAccessibleServerEndPointAddress(SocketAddress);

impl TheirRemotelyAccessibleEndPointAddress for TheirRemotelyAccessibleServerEndPointAddress
{
	// NOTE: It is important that this instance of `TheirRemoteAddress` is not dropped until after these parameters have been used to create an end point.
	#[inline(always)]
	fn populate_end_point_parameters(&self, mut end_pointer_parameters: ucp_ep_params_t) -> ucp_ep_params_t
	{
		let (socket_address, length) = self.0.suitable_for_ffi();
		
		end_pointer_parameters.field_mask |= (ucp_ep_params_field::FLAGS | ucp_ep_params_field::SOCK_ADDR).0 as u64;
		end_pointer_parameters.sockaddr = ucs_sock_addr_t
		{
			addr: socket_address,
			addrlen: length,
		};
		end_pointer_parameters.flags |= ucp_ep_params_flags_field::CLIENT_SERVER.0;
		
		end_pointer_parameters
	}
}
