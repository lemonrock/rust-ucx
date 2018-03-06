// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ucs_async_add_timer"] pub fn ucs_async_add_timer(mode: ucs_async_mode_t, interval: ucs_time_t, cb: ucs_async_event_cb_t, arg: *mut c_void, async: *mut ucs_async_context_t, timer_id_p: *mut c_int) -> ucs_status_t;
	#[link_name = "\u{1}_ucs_async_context_create"] pub fn ucs_async_context_create(mode: ucs_async_mode_t, async_p: *mut *mut ucs_async_context_t) -> ucs_status_t;
	#[link_name = "\u{1}_ucs_async_context_destroy"] pub fn ucs_async_context_destroy(async: *mut ucs_async_context_t);
	#[link_name = "\u{1}_ucs_async_modify_handler"] pub fn ucs_async_modify_handler(fd: c_int, events: c_int) -> ucs_status_t;
	#[link_name = "\u{1}_ucs_async_poll"] pub fn ucs_async_poll(async: *mut ucs_async_context_t);
	#[link_name = "\u{1}_ucs_async_remove_handler"] pub fn ucs_async_remove_handler(id: c_int, sync: c_int) -> ucs_status_t;
	#[link_name = "\u{1}_ucs_async_set_event_handler"] pub fn ucs_async_set_event_handler(mode: ucs_async_mode_t, event_fd: c_int, events: c_int, cb: ucs_async_event_cb_t, arg: *mut c_void, async: *mut ucs_async_context_t) -> ucs_status_t;
}
