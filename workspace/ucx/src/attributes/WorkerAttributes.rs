// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Worker attributes.
#[derive(Debug)]
pub struct WorkerAttributes(ucp_worker_attr_t);

impl PartialEq for WorkerAttributes
{
	#[inline(always)]
	fn eq(&self, other: &WorkerAttributes) -> bool
	{
		self.thread_mode() == other.thread_mode()
	}
}

impl Eq for WorkerAttributes
{
}

impl PartialOrd for WorkerAttributes
{
	#[inline(always)]
	fn partial_cmp(&self, other: &WorkerAttributes) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for WorkerAttributes
{
	#[inline(always)]
	fn cmp(&self, other: &WorkerAttributes) -> Ordering
	{
		self.worker_thread_mode().cmp(&other.worker_thread_mode())
	}
}

impl Hash for WorkerAttributes
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.thread_mode().hash(state);
	}
}

impl WorkerAttributes
{
	/// Worker thread mode; closely related to the value in the `requested_features` of the application context configuration.
	/// NOTE: SerializedOneThreadAtATime is NEVER returned by ucx currently.
	#[inline(always)]
	pub fn worker_thread_mode(&self) -> WorkerThreadMode
	{
		WorkerThreadMode::from_ucs_thread_mode_t(self.thread_mode())
	}
	
	#[inline(always)]
	pub(crate) fn query(&self, handle: ucp_worker_h) -> Self
	{
		let mut attributes: ucp_worker_attr_t = unsafe { uninitialized() };
		attributes.field_mask = ucp_worker_attr_field::THREAD_MODE.0 as u64;
		
		panic_on_error!(ucp_worker_query, handle, &mut attributes);
		WorkerAttributes(attributes)
	}
	
	#[inline(always)]
	fn thread_mode(&self) -> ucs_thread_mode_t
	{
		self.0.thread_mode
	}
}
