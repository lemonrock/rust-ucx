// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// The immediate outcome of a non-blocking request (similar to the either crate's `Either`).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum NonBlockingRequestCompletedOrInProgress<CompletedVariant, InProgressVariant>
{
	/// This non-blocking request has completed.
	Completed(CompletedVariant),
	
	/// This non-blocking request is in progress.
	InProgress(InProgressVariant),
}

impl<CompletedVariant, InProgressVariant> NonBlockingRequestCompletedOrInProgress<CompletedVariant, InProgressVariant>
{
	/// Returns true if the value is the `Completed` variant.
	#[inline(always)]
	pub fn is_completed(&self) -> bool
	{
		match *self
		{
			Completed(_) => true,
			InProgress(_) => false,
		}
	}
	
	/// Returns true if the value is the `InProgress` variant.
	#[inline(always)]
	pub fn is_in_progress(&self) -> bool
	{
		match *self
		{
			Completed(_) => false,
			InProgress(_) => true,
		}
	}
	
	/// Convert the Completed side of `NonBlockingRequestCompletedOrInProgress<CompletedVariant, InProgressVariant>` to an `Option<CompletedVariant>`.
	#[inline(always)]
	pub fn completed(self) -> Option<CompletedVariant>
	{
		match self
		{
			Completed(completed) => Some(completed),
			InProgress(_) => None,
		}
	}
	
	/// Convert the InProgress side of `NonBlockingRequestCompletedOrInProgress<CompletedVariant, InProgressVariant>` to an `Option<InProgressVariant>`.
	#[inline(always)]
	pub fn in_progress(self) -> Option<InProgressVariant>
	{
		match self
		{
			Completed(_) => None,
			InProgress(in_progress) => Some(in_progress),
		}
	}
	
	/// Convert `&NonBlockingRequestCompletedOrInProgress<CompletedVariant, InProgressVariant>` to `NonBlockingRequestCompletedOrInProgress<&CompletedVariant, &InProgressVariant>`.
	#[inline(always)]
	pub fn as_ref(&self) -> NonBlockingRequestCompletedOrInProgress<&CompletedVariant, &InProgressVariant>
	{
		match *self
		{
			Completed(ref completed) => Completed(completed),
			InProgress(ref in_progress) => InProgress(in_progress),
		}
	}
	
	/// Convert `&mut NonBlockingRequestCompletedOrInProgress<CompletedVariant, InProgressVariant>` to `NonBlockingRequestCompletedOrInProgress<&mut CompletedVariant, &mut InProgressVariant>`.
	#[inline(always)]
	pub fn as_mut(&mut self) -> NonBlockingRequestCompletedOrInProgress<&mut CompletedVariant, &mut InProgressVariant>
	{
		match *self
		{
			Completed(ref mut completed) => Completed(completed),
			InProgress(ref mut in_progress) => InProgress(in_progress),
		}
	}
}
