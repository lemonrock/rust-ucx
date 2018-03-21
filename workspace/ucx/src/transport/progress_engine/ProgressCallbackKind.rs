// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Progress callback kind.
///
/// Mimics `ucs_callback_q` flags, but such that invalid combinations are not possible.
#[repr(u32)]
pub enum ProgressCallbackKind
{
	/// Execute on slow-path.
	Slow = 0,
	
	/// Execute on fast-path if at all possible.
	Fast = 1,
	
	/// Execute on slow-path and ?ensure only occurs once?
	OneShot = 2,
}
