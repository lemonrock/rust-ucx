// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_uct_query_md_resources"] pub fn uct_query_md_resources(resources_p: *mut *mut uct_md_resource_desc_t, num_resources_p: *mut c_uint) -> ucs_status_t;
	#[link_name = "\u{1}_uct_release_md_resource_list"] pub fn uct_release_md_resource_list(resources: *mut uct_md_resource_desc_t);
}
