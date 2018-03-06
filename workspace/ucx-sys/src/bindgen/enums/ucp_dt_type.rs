// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ucp_dt_type
{
	UCP_DATATYPE_CONTIG = 0,
	UCP_DATATYPE_STRIDED = 1,
	UCP_DATATYPE_IOV = 2,
	UCP_DATATYPE_GENERIC = 7,
	UCP_DATATYPE_SHIFT = 3,
}

impl ucp_dt_type
{
	pub const CLASS_MASK: Self = ucp_dt_type::UCP_DATATYPE_GENERIC;
}
