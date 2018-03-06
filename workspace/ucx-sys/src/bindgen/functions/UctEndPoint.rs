// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_uct_ep_check"] pub fn uct_ep_check(ep: uct_ep_h, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_t;
	#[link_name = "\u{1}_uct_ep_connect_to_ep"] pub fn uct_ep_connect_to_ep(ep: uct_ep_h, dev_addr: *const uct_device_addr_t, ep_addr: *const uct_ep_addr_t) -> ucs_status_t;
	#[link_name = "\u{1}_uct_ep_create"] pub fn uct_ep_create(iface: uct_iface_h, ep_p: *mut uct_ep_h) -> ucs_status_t;
	#[link_name = "\u{1}_uct_ep_create_connected"] pub fn uct_ep_create_connected(iface: uct_iface_h, dev_addr: *const uct_device_addr_t, iface_addr: *const uct_iface_addr_t, ep_p: *mut uct_ep_h) -> ucs_status_t;
	#[link_name = "\u{1}_uct_ep_create_sockaddr"] pub fn uct_ep_create_sockaddr(iface: uct_iface_h, sockaddr: *const ucs_sock_addr_t, priv_data: *const c_void, length: usize, ep_p: *mut uct_ep_h) -> ucs_status_t;
	#[link_name = "\u{1}_uct_ep_destroy"] pub fn uct_ep_destroy(ep: uct_ep_h);
	#[link_name = "\u{1}_uct_ep_get_address"] pub fn uct_ep_get_address(ep: uct_ep_h, addr: *mut uct_ep_addr_t) -> ucs_status_t;
}
