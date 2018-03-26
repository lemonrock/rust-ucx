// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	pub fn ucp_stream_data_release(ep: ucp_ep_h, data: *mut c_void);
	pub fn ucp_stream_recv_data_nb(ep: ucp_ep_h, length: *mut usize) -> ucs_status_ptr_t;
	pub fn ucp_stream_recv_nb(ep: ucp_ep_h, buffer: *mut c_void, count: usize, datatype: ucp_datatype_t, cb: ucp_stream_recv_callback_t, length: *mut usize, flags: c_uint) -> ucs_status_ptr_t;
	pub fn ucp_stream_recv_request_test(request: *mut c_void, length_p: *mut usize) -> ucs_status_t;
	pub fn ucp_stream_send_nb(ep: ucp_ep_h, buffer: *const c_void, count: usize, datatype: ucp_datatype_t, cb: ucp_send_callback_t, flags: c_uint) -> ucs_status_ptr_t;
	pub fn ucp_stream_worker_poll(worker: ucp_worker_h, poll_eps: *mut ucp_stream_poll_ep_t, max_eps: usize, flags: c_uint) -> isize;
}
