// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A message.
pub trait Message: Debug
{
	/// Start address of this message.
	#[inline(always)]
	fn address(&self) -> NonNull<u8>;
	
	/// Count of items in this message.
	/// This is ***not*** the number of bytes.
	#[inline(always)]
	fn count(&self) -> usize;
	
	#[doc(hidden)]
	#[inline(always)]
	fn data_type_descriptor(&self) -> ucp_datatype_t;
}
