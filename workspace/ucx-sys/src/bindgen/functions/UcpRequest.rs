// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	pub fn ucp_request_check_status(request: *mut c_void) -> ucs_status_t;
	pub fn ucp_request_free(request: *mut c_void);
	pub fn ucp_request_test(request: *mut c_void, info: *mut ucp_tag_recv_info_t) -> ucs_status_t;
}
