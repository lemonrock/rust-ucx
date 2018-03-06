// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
pub struct __BindgenUnionField<T>(PhantomData<T>);

impl<T> __BindgenUnionField<T>
{
	
	pub fn new() -> Self
	{
		__BindgenUnionField(PhantomData)
	}
	
	pub unsafe fn as_ref(&self) -> &T
	{
		transmute(self)
	}
	
	pub unsafe fn as_mut(&mut self) -> &mut T
	{
		transmute(self)
	}
}

impl<T> Default for __BindgenUnionField<T>
{
	
	fn default() -> Self
	{
		Self::new()
	}
}

impl<T> Clone for __BindgenUnionField<T>
{
	
	fn clone(&self) -> Self
	{
		Self::new()
	}
}

impl<T> Copy for __BindgenUnionField<T>
{
}

impl<T> Debug for __BindgenUnionField<T>
{
	fn fmt(&self, fmt: &mut Formatter) -> Result
	{
		fmt.write_str("__BindgenUnionField")
	}
}

impl<T> ::std::hash::Hash for __BindgenUnionField<T>
{
	fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H)
	{
	}
}

impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T>
{
	fn eq(&self, _other: &__BindgenUnionField<T>) -> bool
	{
		true
	}
}

impl<T> ::std::cmp::Eq for __BindgenUnionField<T>
{
}
