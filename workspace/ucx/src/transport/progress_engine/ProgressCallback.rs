// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Progress callback.
pub trait ProgressCallback
{
	/// What kind of callback?
	#[inline(always)]
	fn kind(&self) -> ProgressCallbackKind;
	
	/// Called back.
	///
	/// Returns `true` if actually caused progress.
	#[inline(always)]
	fn invoke(&mut self) -> bool;
}
