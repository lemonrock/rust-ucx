// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ucp_cleanup"] pub fn ucp_cleanup(context_p: ucp_context_h);
	#[link_name = "\u{1}_ucp_context_print_info"] pub fn ucp_context_print_info(context: ucp_context_h, stream: *mut FILE);
	#[link_name = "\u{1}_ucp_context_query"] pub fn ucp_context_query(context_p: ucp_context_h, attr: *mut ucp_context_attr_t) -> ucs_status_t;
}
