// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A trait that some objects that wrap UCX handles implement.
/// Allows access to the attributes (if you like, read-only fields) of a UCX object instance.
pub trait HasAttributes
{
	/// The type of Attributes.
	type Attributes: Sized;
	
	/// Obtain attributes.
	#[inline(always)]
	fn attributes(&self) -> &Self::Attributes;
}
