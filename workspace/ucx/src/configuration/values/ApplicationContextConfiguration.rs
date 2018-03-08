// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Application context configuration.
#[serde(deny_unknown_fields, default)]
#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ApplicationContextConfiguration
{
	/// Features which must be provided or initialization will fail.
	pub requested_features: RequestedFeatures,

	/// Mask which specifies particular bits of the tag which can uniquely identify the sender (UCP end point) in tagged operations.
	/// This defaults to 0 if `None`.
	pub tag_sender_mask: Option<TagSenderMask>,

	/// An optimization hint of how many endpoints would be created on this context.
	/// For example, when used from MPI or SHMEM libraries, this number would specify the number of ranks (or processing elements) in the job.
	/// Does not affect semantics, but only transport selection criteria and the resulting performance.
	/// In such a case it will override the number of endpoints specified here.
	pub estimated_number_of_end_points: usize,
}

impl Default for ApplicationContextConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			requested_features: Default::default(),
			tag_sender_mask: None,
			estimated_number_of_end_points: 256,
		}
	}
}

impl ApplicationContextConfiguration
{
	/// Creates a new application context.
	#[inline(always)]
	pub fn new<MemoryCustomization: NonBlockingRequestMemoryCustomization>(self, sealing_key: SealingKey, opening_key: OpeningKey, ucp_configuration_wrapper: UcpConfigurationWrapper) -> Result<ApplicationContext<MemoryCustomization>, ApplicationContextCreationError>
	{
		let parameters = self.parameters_for_new_application_context::<MemoryCustomization>();
		let mut handle = unsafe { uninitialized() };
		let status = unsafe { ucp_init_version(UCP_API_MAJOR, UCP_API_MINOR, &parameters, ucp_configuration_wrapper.handle, &mut handle) };

		match status.parse()?
		{
			IsOk => Ok
			(
				ApplicationContext
				{
					handle,
					application_context_handle_drop_safety: Rc::new(ApplicationContextHandleDropSafety(handle)),
					application_context_configuration: self,
					sealing_key,
					opening_key,
					phantom_data: PhantomData,
				}
			),

			Error(error_code) => match error_code
			{
				FunctionNotImplemented | UnsupportedOperation => Err(ApplicationContextCreationError::FunctionalityNotImplementedOrSupported),

				OutOfMemory => panic!("Out of memory"),

				_ => panic!("Unexpected error for 'ucp_init_version' of '{}'", error_code),
			},

			UnknownErrorCode(_) | OperationInProgress => panic!("Status should not be possible for 'ucp_init_version'"),
		}
	}

	#[inline(always)]
	fn parameters_for_new_application_context<MemoryCustomization: NonBlockingRequestMemoryCustomization>(&self) -> ucp_params_t
	{
		const CBoolTrue: i32 = 1;

		let (initialize, clean_up) = MemoryCustomization::function_pointers();

		ucp_params_t
		{
			field_mask: (ucp_params_field::FEATURES | ucp_params_field::REQUEST_SIZE | ucp_params_field::REQUEST_INIT | ucp_params_field::REQUEST_CLEANUP | ucp_params_field::TAG_SENDER_MASK | ucp_params_field::MT_WORKERS_SHARED | ucp_params_field::ESTIMATED_NUM_EPS).0 as u64,
			features: self.requested_features.ucp_feature(self.tag_sender_mask).0 as u64,

			request_size: MemoryCustomization::reserved_space_in_non_blocking_requests,

			request_init: initialize,

			request_cleanup: clean_up,

			tag_sender_mask: if let Some(tag_sender_mask) = self.tag_sender_mask
			{
				tag_sender_mask.0
			}
			else
			{
				0
			},

			// This flag indicates if this context is shared by multiple workers from different threads.
			// If so, this context needs thread safety support; otherwise, the context does not need to provide thread safety.
			//
			//For example, if the context is used by single worker, and that worker is shared by multiple threads, this context does not need thread safety; if the context is used by worker 1 and worker 2, and worker 1 is used by thread 1 and worker 2 is used by thread 2, then this context needs thread safety.
			mt_workers_shared: CBoolTrue,

			estimated_num_eps: self.estimated_number_of_end_points,
		}
	}

	#[inline(always)]
	pub(crate) fn wake_up_events(&self) -> ucp_wakeup_event_types
	{
		self.requested_features.wake_up_events(self.tag_sender_mask)
	}
}
