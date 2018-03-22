// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ucp_get"] pub fn ucp_get(ep: ucp_ep_h, buffer: *mut c_void, length: usize, remote_addr: u64, rkey: ucp_rkey_h) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_get_nb"] pub fn ucp_get_nb(ep: ucp_ep_h, buffer: *mut c_void, length: usize, remote_addr: u64, rkey: ucp_rkey_h, cb: ucp_send_callback_t) -> ucs_status_ptr_t;
	#[link_name = "\u{1}_ucp_get_nbi"] pub fn ucp_get_nbi(ep: ucp_ep_h, buffer: *mut c_void, length: usize, remote_addr: u64, rkey: ucp_rkey_h) -> ucs_status_t;
}
