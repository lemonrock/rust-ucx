// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_uct_worker_create"] pub fn uct_worker_create(async: *mut ucs_async_context_t, thread_mode: ucs_thread_mode_t, worker_p: *mut uct_worker_h) -> ucs_status_t;
	#[link_name = "\u{1}_uct_worker_destroy"] pub fn uct_worker_destroy(worker: uct_worker_h);
	#[link_name = "\u{1}_uct_worker_progress_register_safe"] pub fn uct_worker_progress_register_safe(worker: uct_worker_h, func: ucs_callback_t, arg: *mut c_void, flags: c_uint, id_p: *mut uct_worker_cb_id_t);
	#[link_name = "\u{1}_uct_worker_progress_unregister_safe"] pub fn uct_worker_progress_unregister_safe(worker: uct_worker_h, id_p: *mut uct_worker_cb_id_t);
}
