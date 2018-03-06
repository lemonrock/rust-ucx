// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ucp_atomic_add32"] pub fn ucp_atomic_add32(ep: ucp_ep_h, add: u32, remote_addr: u64, rkey: ucp_rkey_h) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_atomic_add64"] pub fn ucp_atomic_add64(ep: ucp_ep_h, add: u64, remote_addr: u64, rkey: ucp_rkey_h) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_atomic_cswap32"] pub fn ucp_atomic_cswap32(ep: ucp_ep_h, compare: u32, swap: u32, remote_addr: u64, rkey: ucp_rkey_h, result: *mut u32) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_atomic_cswap64"] pub fn ucp_atomic_cswap64(ep: ucp_ep_h, compare: u64, swap: u64, remote_addr: u64, rkey: ucp_rkey_h, result: *mut u64) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_atomic_fadd32"] pub fn ucp_atomic_fadd32(ep: ucp_ep_h, add: u32, remote_addr: u64, rkey: ucp_rkey_h, result: *mut u32) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_atomic_fadd64"] pub fn ucp_atomic_fadd64(ep: ucp_ep_h, add: u64, remote_addr: u64, rkey: ucp_rkey_h, result: *mut u64) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_atomic_fetch_nb"] pub fn ucp_atomic_fetch_nb(ep: ucp_ep_h, opcode: ucp_atomic_fetch_op_t, value: u64, result: *mut c_void, op_size: usize, remote_addr: u64, rkey: ucp_rkey_h, cb: ucp_send_callback_t) -> ucs_status_ptr_t;
	#[link_name = "\u{1}_ucp_atomic_post"] pub fn ucp_atomic_post(ep: ucp_ep_h, opcode: ucp_atomic_post_op_t, value: u64, op_size: usize, remote_addr: u64, rkey: ucp_rkey_h) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_atomic_swap32"] pub fn ucp_atomic_swap32(ep: ucp_ep_h, swap: u32, remote_addr: u64, rkey: ucp_rkey_h, result: *mut u32) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_atomic_swap64"] pub fn ucp_atomic_swap64(ep: ucp_ep_h, swap: u64, remote_addr: u64, rkey: ucp_rkey_h, result: *mut u64) -> ucs_status_t;
}
