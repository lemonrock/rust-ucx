// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ucp_worker_arm"] pub fn ucp_worker_arm(worker: ucp_worker_h) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_worker_destroy"] pub fn ucp_worker_destroy(worker: ucp_worker_h);
	#[link_name = "\u{1}_ucp_worker_fence"] pub fn ucp_worker_fence(worker: ucp_worker_h) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_worker_flush"] pub fn ucp_worker_flush(worker: ucp_worker_h) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_worker_flush_nb"] pub fn ucp_worker_flush_nb(worker: ucp_worker_h, flags: c_uint, cb: ucp_send_callback_t) -> ucs_status_ptr_t;
	#[link_name = "\u{1}_ucp_worker_get_address"] pub fn ucp_worker_get_address(worker: ucp_worker_h, address_p: *mut *mut ucp_address_t, address_length_p: *mut usize) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_worker_get_efd"] pub fn ucp_worker_get_efd(worker: ucp_worker_h, fd: *mut c_int) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_worker_print_info"] pub fn ucp_worker_print_info(worker: ucp_worker_h, stream: *mut FILE);
	#[link_name = "\u{1}_ucp_worker_progress"] pub fn ucp_worker_progress(worker: ucp_worker_h) -> c_uint;
	#[link_name = "\u{1}_ucp_worker_query"] pub fn ucp_worker_query(worker: ucp_worker_h, attr: *mut ucp_worker_attr_t) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_worker_release_address"] pub fn ucp_worker_release_address(worker: ucp_worker_h, address: *mut ucp_address_t);
	#[link_name = "\u{1}_ucp_worker_signal"] pub fn ucp_worker_signal(worker: ucp_worker_h) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_worker_wait"] pub fn ucp_worker_wait(worker: ucp_worker_h) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_worker_wait_mem"] pub fn ucp_worker_wait_mem(worker: ucp_worker_h, address: *mut c_void);
}
