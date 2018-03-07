// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An application represents a fully-configured Open UCX.
///
/// Create it using `Configuration.new_application()`.
///
#[derive(Debug)]
pub struct Application<MemoryCustomization = NoNonBlockingRequestMemoryCustomization>
{
	ucx_configuration_wrapper: UcxConfigurationWrapper,
	hyper_threads_application_contexts: Arc<HyperThreadsApplicationContextConfigurations>,
	phantom_data: PhantomData<MemoryCustomization>,
}

impl<MemoryCustomization: NonBlockingRequestMemoryCustomization> Application<MemoryCustomization>
{
	/// Creates new hyper thread context.
	///
	/// Panics if it can not.
	#[inline(always)]
	pub fn new_hyper_thread_context_or_panic(&self, hyper_thread_index: ZeroBasedHyperThreadIndex) -> HyperThreadContext
	{
		self.new_hyper_thread_context(hyper_thread_index).expect("Could not create a hyper thread context with the configuration settings we require")
	}
	
	/// Creates new hyper thread context.
	///
	/// Failure should normally be seen as unrecoverable unless one if trying various configurations to see which is acceptable.
	pub fn new_hyper_thread_context(&self, hyper_thread_index: ZeroBasedHyperThreadIndex) -> Result<HyperThreadContext, HyperThreadContextCreationError>
	{
		let application_context_configuration = self.hyper_threads_application_contexts.application_context_configuration(hyper_thread_index);
		application_context_configuration.per_hyper_thread::<MemoryCustomization>(&self.ucx_configuration_wrapper)
	}
}
