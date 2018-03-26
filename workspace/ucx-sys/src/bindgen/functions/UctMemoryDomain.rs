// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	pub fn uct_md_close(md: uct_md_h);
	pub fn uct_md_config_read(name: *const c_char, env_prefix: *const c_char, filename: *const c_char, config_p: *mut *mut uct_md_config_t) -> ucs_status_t;
	pub fn uct_md_iface_config_read(md: uct_md_h, tl_name: *const c_char, env_prefix: *const c_char, filename: *const c_char, config_p: *mut *mut uct_iface_config_t) -> ucs_status_t;
	pub fn uct_md_is_mem_type_owned(md: uct_md_h, addr: *mut c_void, length: usize) -> c_int;
	pub fn uct_md_is_sockaddr_accessible(md: uct_md_h, sockaddr: *const ucs_sock_addr_t, mode: uct_sockaddr_accessibility_t) -> c_int;
	pub fn uct_md_mkey_pack(md: uct_md_h, memh: uct_mem_h, rkey_buffer: *mut c_void) -> ucs_status_t;
	pub fn uct_md_open(md_name: *const c_char, config: *const uct_md_config_t, md_p: *mut uct_md_h) -> ucs_status_t;
	pub fn uct_md_query(md: uct_md_h, md_attr: *mut uct_md_attr_t) -> ucs_status_t;
	pub fn uct_md_query_tl_resources(md: uct_md_h, resources_p: *mut *mut uct_tl_resource_desc_t, num_resources_p: *mut c_uint) -> ucs_status_t;
}
