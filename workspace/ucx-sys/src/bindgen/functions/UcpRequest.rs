// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ucp_request_check_status"] pub fn ucp_request_check_status(request: *mut c_void) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_request_free"] pub fn ucp_request_free(request: *mut c_void);
	#[link_name = "\u{1}_ucp_request_test"] pub fn ucp_request_test(request: *mut c_void, info: *mut ucp_tag_recv_info_t) -> ucs_status_t;
}
