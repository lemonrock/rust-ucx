// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
pub struct ucm_event
{
	pub mmap: __BindgenUnionField<ucm_event__bindgen_ty_1>,
	pub munmap: __BindgenUnionField<ucm_event__bindgen_ty_2>,
	pub mremap: __BindgenUnionField<ucm_event__bindgen_ty_3>,
	pub shmat: __BindgenUnionField<ucm_event__bindgen_ty_4>,
	pub shmdt: __BindgenUnionField<ucm_event__bindgen_ty_5>,
	pub sbrk: __BindgenUnionField<ucm_event__bindgen_ty_6>,
	pub vm_mapped: __BindgenUnionField<ucm_event__bindgen_ty_7>,
	pub vm_unmapped: __BindgenUnionField<ucm_event__bindgen_ty_7>,
	pub bindgen_union_field: [u64; 6usize],
}

impl Default for ucm_event
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ucm_event
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ucm_event {{ union }}")
	}
}
