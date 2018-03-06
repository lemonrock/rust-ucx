// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ucp_listener_create"] pub fn ucp_listener_create(worker: ucp_worker_h, params: *const ucp_listener_params_t, listener_p: *mut ucp_listener_h) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_listener_destroy"] pub fn ucp_listener_destroy(listener: ucp_listener_h);
}
