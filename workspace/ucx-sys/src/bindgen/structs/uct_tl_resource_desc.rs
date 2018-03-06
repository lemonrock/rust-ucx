// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct uct_tl_resource_desc
{
	pub tl_name: [c_char; 10usize],
	pub dev_name: [c_char; 32usize],
	pub dev_type: uct_device_type_t,
}

impl Default for uct_tl_resource_desc
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for uct_tl_resource_desc
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"uct_tl_resource_desc {{ tl_name: {:?}, dev_name: [{}], dev_type: {:?} }}",
			self.tl_name,
			self.dev_name
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>(),
			self.dev_type
		)
	}
}
