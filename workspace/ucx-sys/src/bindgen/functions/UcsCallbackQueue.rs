// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	pub fn ucs_callbackq_add(cbq: *mut ucs_callbackq_t, cb: ucs_callback_t, arg: *mut c_void, flags: c_uint) -> c_int;
	pub fn ucs_callbackq_add_safe(cbq: *mut ucs_callbackq_t, cb: ucs_callback_t, arg: *mut c_void, flags: c_uint) -> c_int;
	pub fn ucs_callbackq_cleanup(cbq: *mut ucs_callbackq_t);
	pub fn ucs_callbackq_init(cbq: *mut ucs_callbackq_t) -> ucs_status_t;
	pub fn ucs_callbackq_remove(cbq: *mut ucs_callbackq_t, id: c_int);
	pub fn ucs_callbackq_remove_if(cbq: *mut ucs_callbackq_t, pred: ucs_callbackq_predicate_t, arg: *mut c_void);
	pub fn ucs_callbackq_remove_safe(cbq: *mut ucs_callbackq_t, id: c_int);
}
