// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	pub fn uct_rkey_ptr(rkey_ob: *mut uct_rkey_bundle_t, remote_addr: u64, addr_p: *mut *mut c_void) -> ucs_status_t;
	pub fn uct_rkey_release(rkey_ob: *const uct_rkey_bundle_t) -> ucs_status_t;
	pub fn uct_rkey_unpack(rkey_buffer: *const c_void, rkey_ob: *mut uct_rkey_bundle_t) -> ucs_status_t;
}
