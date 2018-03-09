// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


macro_rules! panic_on_error_with_clean_up
{
	($status: ident, $failureBlock: block, $function: path$(,$argument: expr)*) =>
	{
		{
			let $status = unsafe { $function($($argument),*) };
			if $status != $crate::ucx_sys::ucs_status_t::UCS_OK
			{
				$failureBlock
				
				let status = $crate::status::Status::parse_ucs_status_t($status).expect("Invalid status");
				panic!("{} failed with status code '{:?}'", stringify!($function), status);
			}
		}
	}
}
