// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Worker thread mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WorkerThreadMode
{
	#[allow(missing_docs)]
	OnlyEverUsedFromThisThread,
	
	#[allow(missing_docs)]
	SerializedOneThreadAtATime,
	
	#[allow(missing_docs)]
	UsedSimultaneouslyAcrossMoreThanOneThread,
}

impl WorkerThreadMode
{
	/// In UCS form.
	#[inline(always)]
	pub fn as_ucs_thread_mode_t(self) -> ucs_thread_mode_t
	{
		use self::WorkerThreadMode::*;
		use self::ucs_thread_mode_t::*;
		
		match self
		{
			OnlyEverUsedFromThisThread => UCS_THREAD_MODE_SINGLE,
			SerializedOneThreadAtATime => UCS_THREAD_MODE_SERIALIZED,
			UsedSimultaneouslyAcrossMoreThanOneThread => UCS_THREAD_MODE_MULTI,
		}
	}
	
	///From UCS form.
	#[inline(always)]
	pub fn from_ucs_thread_mode_t(value: ucs_thread_mode_t) -> Self
	{
		use ucs_thread_mode_t::*;
		use self::WorkerThreadMode::*;
		
		match value
		{
			UCS_THREAD_MODE_SINGLE => OnlyEverUsedFromThisThread,
			UCS_THREAD_MODE_SERIALIZED => SerializedOneThreadAtATime,
			UCS_THREAD_MODE_MULTI => UsedSimultaneouslyAcrossMoreThanOneThread,
		}
	}
}
