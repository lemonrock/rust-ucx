// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ucp_mem_advise"] pub fn ucp_mem_advise(context: ucp_context_h, memh: ucp_mem_h, params: *mut ucp_mem_advise_params_t) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_mem_map"] pub fn ucp_mem_map(context: ucp_context_h, params: *const ucp_mem_map_params_t, memh_p: *mut ucp_mem_h) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_mem_unmap"] pub fn ucp_mem_unmap(context: ucp_context_h, memh: ucp_mem_h) -> ucs_status_t;
}
