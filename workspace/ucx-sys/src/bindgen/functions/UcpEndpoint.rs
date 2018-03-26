// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	pub fn ucp_disconnect_nb(ep: ucp_ep_h) -> ucs_status_ptr_t;
	pub fn ucp_ep_close_nb(ep: ucp_ep_h, mode: c_uint) -> ucs_status_ptr_t;
	pub fn ucp_ep_flush(ep: ucp_ep_h) -> ucs_status_t;
	pub fn ucp_ep_flush_nb(ep: ucp_ep_h, flags: c_uint, cb: ucp_send_callback_t) -> ucs_status_ptr_t;
	pub fn ucp_ep_modify_nb(ep: ucp_ep_h, params: *const ucp_ep_params_t) -> ucs_status_ptr_t;
	pub fn ucp_ep_print_info(ep: ucp_ep_h, stream: *mut FILE);
	pub fn ucp_ep_rkey_unpack(ep: ucp_ep_h, rkey_buffer: *const c_void, rkey_p: *mut ucp_rkey_h) -> ucs_status_t;
}
