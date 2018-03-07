// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Attributes for a hyper thread context.
#[derive(Debug)]
pub struct HyperThreadContextAttributes(pub(crate) ucp_context_attr);

impl PartialEq for HyperThreadContextAttributes
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.thread_mode() == other.thread_mode() && self.reserved_space_in_non_blocking_requests() == other.reserved_space_in_non_blocking_requests()
	}
}

impl Eq for HyperThreadContextAttributes
{
}

impl PartialOrd for HyperThreadContextAttributes
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for HyperThreadContextAttributes
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.worker_thread_mode().cmp(&other.worker_thread_mode()).then(self.reserved_space_in_non_blocking_requests().cmp(&other.reserved_space_in_non_blocking_requests()))
	}
}

impl Hash for HyperThreadContextAttributes
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.thread_mode().hash(state);
		self.reserved_space_in_non_blocking_requests().hash(state);
	}
}

impl HyperThreadContextAttributes
{
	/// Worker thread mode; closely related to the value in the `requested_features` of the application context configuration.
	/// NOTE: SerializedOneThreadAtATime is NEVER returned by ucx currently.
	#[inline(always)]
	pub fn worker_thread_mode(&self) -> WorkerThreadMode
	{
		WorkerThreadMode::from_ucs_thread_mode_t(self.thread_mode())
	}
	
	/// Should be the same value as supplied by `NonBlockingRequestMemoryCustomization`.
	#[inline(always)]
	pub fn reserved_space_in_non_blocking_requests(&self) -> usize
	{
		self.0.request_size
	}
	
	#[inline(always)]
	pub(crate) fn query(handle: ucp_context_h) -> Self
	{
		let mut attributes: ucp_context_attr_t = unsafe { uninitialized() };
		attributes.field_mask = (ucp_context_attr_field::REQUEST_SIZE | ucp_context_attr_field::THREAD_MODE).0 as u64;
		
		panic_on_error!(ucp_context_query, handle, &mut attributes);
		HyperThreadContextAttributes(attributes)
	}
	
	#[inline(always)]
	fn thread_mode(&self) -> ucs_thread_mode_t
	{
		self.0.thread_mode
	}
}
