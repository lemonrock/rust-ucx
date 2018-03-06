// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


quick_error!
{
	/// Invalid status error.
	#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
	pub enum InvalidStatusError
	{
		/// Invalid status.
		InvalidStatus(invalid_status_code: i8)
		{
			display("Status is invalid code '{}'", invalid_status_code)
		}
	}
}
