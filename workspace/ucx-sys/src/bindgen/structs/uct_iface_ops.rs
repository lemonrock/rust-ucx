// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct uct_iface_ops
{
	pub ep_put_short: unsafe extern "C" fn(ep: uct_ep_h, buffer: *const c_void, length: c_uint, remote_addr: u64, rkey: uct_rkey_t) -> ucs_status_t,
	pub ep_put_bcopy: unsafe extern "C" fn(ep: uct_ep_h, pack_cb: uct_pack_callback_t, arg: *mut c_void, remote_addr: u64, rkey: uct_rkey_t) -> isize,
	pub ep_put_zcopy: unsafe extern "C" fn(ep: uct_ep_h, iov: *const uct_iov_t, iovcnt: usize, remote_addr: u64, rkey: uct_rkey_t, comp: *mut uct_completion_t) -> ucs_status_t,
	pub ep_get_short: unsafe extern "C" fn(ep: uct_ep_h, buffer: *const c_void, length: c_uint, remote_addr: u64, rkey: uct_rkey_t) -> ucs_status_t,
	pub ep_get_bcopy: unsafe extern "C" fn(ep: uct_ep_h, unpack_cb: uct_unpack_callback_t, arg: *mut c_void, length: usize, remote_addr: u64, rkey: uct_rkey_t, comp: *mut uct_completion_t) -> ucs_status_t,
	pub ep_get_zcopy: unsafe extern "C" fn(ep: uct_ep_h, iov: *const uct_iov_t, iovcnt: usize, remote_addr: u64, rkey: uct_rkey_t, comp: *mut uct_completion_t) -> ucs_status_t,
	pub ep_am_short: unsafe extern "C" fn(ep: uct_ep_h, id: u8, header: u64, payload: *const c_void, length: c_uint) -> ucs_status_t,
	pub ep_am_bcopy: unsafe extern "C" fn(ep: uct_ep_h, id: u8, pack_cb: uct_pack_callback_t, arg: *mut c_void, flags: c_uint) -> isize,
	pub ep_am_zcopy: unsafe extern "C" fn(ep: uct_ep_h, id: u8, header: *const c_void, header_length: c_uint, iov: *const uct_iov_t, iovcnt: usize, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_t,
	pub ep_atomic_add64: unsafe extern "C" fn(ep: uct_ep_h, add: u64, remote_addr: u64, rkey: uct_rkey_t) -> ucs_status_t,
	pub ep_atomic_fadd64: unsafe extern "C" fn(ep: uct_ep_h, add: u64, remote_addr: u64, rkey: uct_rkey_t, result: *mut u64, comp: *mut uct_completion_t) -> ucs_status_t,
	pub ep_atomic_swap64: unsafe extern "C" fn(ep: uct_ep_h, swap: u64, remote_addr: u64, rkey: uct_rkey_t, result: *mut u64, comp: *mut uct_completion_t) -> ucs_status_t,
	pub ep_atomic_cswap64: unsafe extern "C" fn(ep: uct_ep_h, compare: u64, swap: u64, remote_addr: u64, rkey: uct_rkey_t, result: *mut u64, comp: *mut uct_completion_t) -> ucs_status_t,
	pub ep_atomic_add32: unsafe extern "C" fn(ep: uct_ep_h, add: u32, remote_addr: u64, rkey: uct_rkey_t) -> ucs_status_t,
	pub ep_atomic_fadd32: unsafe extern "C" fn(ep: uct_ep_h, add: u32, remote_addr: u64, rkey: uct_rkey_t, result: *mut u32, comp: *mut uct_completion_t) -> ucs_status_t,
	pub ep_atomic_swap32: unsafe extern "C" fn(ep: uct_ep_h, swap: u32, remote_addr: u64, rkey: uct_rkey_t, result: *mut u32, comp: *mut uct_completion_t) -> ucs_status_t,
	pub ep_atomic_cswap32: unsafe extern "C" fn(ep: uct_ep_h, compare: u32, swap: u32, remote_addr: u64, rkey: uct_rkey_t, result: *mut u32, comp: *mut uct_completion_t) -> ucs_status_t,
	pub ep_tag_eager_short: unsafe extern "C" fn(ep: uct_ep_h, tag: uct_tag_t, data: *const c_void, length: usize) -> ucs_status_t,
	pub ep_tag_eager_bcopy: unsafe extern "C" fn(ep: uct_ep_h, tag: uct_tag_t, imm: u64, pack_cb: uct_pack_callback_t, arg: *mut c_void, flags: c_uint) -> isize,
	pub ep_tag_eager_zcopy: unsafe extern "C" fn(ep: uct_ep_h, tag: uct_tag_t, imm: u64, iov: *const uct_iov_t, iovcnt: usize, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_t,
	pub ep_tag_rndv_zcopy: unsafe extern "C" fn(ep: uct_ep_h, tag: uct_tag_t, header: *const c_void, header_length: c_uint, iov: *const uct_iov_t, iovcnt: usize, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_ptr_t,
	pub ep_tag_rndv_cancel: unsafe extern "C" fn(ep: uct_ep_h, op: *mut c_void) -> ucs_status_t,
	pub ep_tag_rndv_request: unsafe extern "C" fn(ep: uct_ep_h, tag: uct_tag_t, header: *const c_void, header_length: c_uint, flags: c_uint) -> ucs_status_t,
	pub iface_tag_recv_zcopy: unsafe extern "C" fn(iface: uct_iface_h, tag: uct_tag_t, tag_mask: uct_tag_t, iov: *const uct_iov_t, iovcnt: usize, ctx: *mut uct_tag_context_t) -> ucs_status_t,
	pub iface_tag_recv_cancel: unsafe extern "C" fn(iface: uct_iface_h, ctx: *mut uct_tag_context_t, force: c_int) -> ucs_status_t,
	pub ep_pending_add: unsafe extern "C" fn(ep: uct_ep_h, n: *mut uct_pending_req_t) -> ucs_status_t,
	pub ep_pending_purge: unsafe extern "C" fn(ep: uct_ep_h, cb: uct_pending_purge_callback_t, arg: *mut c_void),
	pub ep_flush: unsafe extern "C" fn(ep: uct_ep_h, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_t,
	pub ep_fence: unsafe extern "C" fn(ep: uct_ep_h, flags: c_uint) -> ucs_status_t,
	pub ep_check: unsafe extern "C" fn(ep: uct_ep_h, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_t,
	pub ep_create: unsafe extern "C" fn(iface: uct_iface_h, ep_p: *mut uct_ep_h) -> ucs_status_t,
	pub ep_create_connected: unsafe extern "C" fn(iface: uct_iface_h, dev_addr: *const uct_device_addr_t, iface_addr: *const uct_iface_addr_t, ep_p: *mut uct_ep_h) -> ucs_status_t,
	pub ep_create_sockaddr: unsafe extern "C" fn(iface: uct_iface_h, sockaddr: *const ucs_sock_addr_t, priv_data: *const c_void, length: usize, ep_p: *mut uct_ep_h) -> ucs_status_t,
	pub ep_destroy: unsafe extern "C" fn(ep: uct_ep_h),
	pub ep_get_address: unsafe extern "C" fn(ep: uct_ep_h, addr: *mut uct_ep_addr_t) -> ucs_status_t,
	pub ep_connect_to_ep: unsafe extern "C" fn(ep: uct_ep_h, dev_addr: *const uct_device_addr_t, ep_addr: *const uct_ep_addr_t) -> ucs_status_t,
	pub iface_flush: unsafe extern "C" fn(iface: uct_iface_h, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_t,
	pub iface_fence: unsafe extern "C" fn(iface: uct_iface_h, flags: c_uint) -> ucs_status_t,
	pub iface_progress_enable: unsafe extern "C" fn(iface: uct_iface_h, flags: c_uint),
	pub iface_progress_disable: unsafe extern "C" fn(iface: uct_iface_h, flags: c_uint),
	pub iface_progress: unsafe extern "C" fn(iface: uct_iface_h) -> c_uint,
	pub iface_event_fd_get: unsafe extern "C" fn(iface: uct_iface_h, fd_p: *mut c_int) -> ucs_status_t,
	pub iface_event_arm: unsafe extern "C" fn(iface: uct_iface_h, events: c_uint) -> ucs_status_t,
	pub iface_close: unsafe extern "C" fn(iface: uct_iface_h),
	pub iface_query: unsafe extern "C" fn(iface: uct_iface_h, iface_attr: *mut uct_iface_attr_t) -> ucs_status_t,
	pub iface_get_device_address: unsafe extern "C" fn(iface: uct_iface_h, addr: *mut uct_device_addr_t) -> ucs_status_t,
	pub iface_get_address: unsafe extern "C" fn(iface: uct_iface_h, addr: *mut uct_iface_addr_t) -> ucs_status_t,
	pub iface_is_reachable: unsafe extern "C" fn(iface: uct_iface_h, dev_addr: *const uct_device_addr_t, iface_addr: *const uct_iface_addr_t) -> c_int,
}

impl Default for uct_iface_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
