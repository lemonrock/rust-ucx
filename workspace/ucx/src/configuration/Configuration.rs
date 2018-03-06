// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Configuration suitable for use with Serde and so JSON, HJSON, XML, etc.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[allow(missing_docs)]
pub struct Configuration
{
	pub network_device_names: NetworkDeviceNames,
	pub shared_memory_device_names: SharedMemoryDeviceNames,
	pub accelerated_device_names: AcceleratedDeviceNames,
	pub self_device_names: SelfDeviceNames,
	pub transport_layers_to_use_if_available: TransportLayersToUseIfAvailable,
	pub memory_allocator_priority_set: MemoryAllocatorPrioritySet,
	pub threshold_for_switching_from_short_to_buffer_copy_protocol: ThresholdForSwitchingFromShortToBufferCopyProtocol,
	pub threshold_for_switching_from_eager_to_rendezvous_protocol: ThresholdForSwitchingFromEagerToRendezvousProtocol,
	pub message_size_threshold_to_start_using_the_rendezvous_protocol_is_case_the_calculated_threshold_is_zero_or_negative: MessageSizeThresholdToStartUsingTheRendezvousProtocolInCaseTheCalculatedThresholdIsZeroOrNegative,
	pub rendezvous_protocol_and_eager_zero_copy_protocol_percentage_difference: RendezvousProtocolAndEagerZeroCopyProtocolPercentageDifference,
	pub threshold_for_switching_from_buffer_copy_protocol_to_zero_copy_protocol: ThresholdForSwitchingFromBufferCopyProtocolToZeroCopyProtocol,
	pub estimation_of_buffer_copy_bandwidth: EstimationOfBufferCopyBandwidth,
	pub atomic_operations_synchronization: AtomicOperationsSynchronization,
	pub maximum_length_of_worker_name: MaximumLengthOfWorkerName,
	pub prefer_spin_lock_over_mutex_when_multi_threading: PreferSpinLockOverMutexWhenMultiThreading,
	pub threshold_for_using_tag_matching_offload_capabilities: ThresholdForUsingTagMatchingOffloadCapabilities,
}

/*
	TODO:-

  {"MAX_EAGER_LANES", "1",
   "Maximal number of devices on which an eager operation may be executed in parallel",
   ucs_offsetof(ucp_config_t, ctx.max_eager_lanes), UCS_CONFIG_TYPE_UINT},

  {"MAX_RNDV_LANES", "1",
   "Maximal number of devices on which a rendezvous operation may be executed in parallel",
   ucs_offsetof(ucp_config_t, ctx.max_rndv_lanes), UCS_CONFIG_TYPE_UINT},

  {"RNDV_SCHEME", "auto",
   "Communication scheme in RNDV protocol.\n"
   " get_zcopy - use get_zcopy scheme in RNDV protocol.\n"
   " put_zcopy - use put_zcopy scheme in RNDV protocol.\n"
   " auto      - runtime automatically chooses optimal scheme to use.\n",
   ucs_offsetof(ucp_config_t, ctx.rndv_mode), UCS_CONFIG_TYPE_ENUM(ucp_rndv_modes)},


  {"ADAPTIVE_PROGRESS", "y",
   "Enable apaptive progress mechanism, which turns on polling only on active\n"
   "transport interfaces.",
   ucs_offsetof(ucp_config_t, ctx.adaptive_progress), UCS_CONFIG_TYPE_BOOL},

  {"SEG_SIZE", "8192",
   "Size of a segment in the worker preregistered memory pool.",
   ucs_offsetof(ucp_config_t, ctx.seg_size), UCS_CONFIG_TYPE_MEMUNITS},

  {"TM_THRESH", "1024", /* TODO: calculate automaticlly */
   "Threshold for using tag matching offload capabilities.\n"
   "Smaller buffers will not be posted to the transport.",
   ucs_offsetof(ucp_config_t, ctx.tm_thresh), UCS_CONFIG_TYPE_MEMUNITS},

  {"TM_MAX_BCOPY", "1024", /* TODO: calculate automaticlly */
   "Maximal size for posting \"bounce buffer\" (UCX interal preregistered memory) for\n"
   "tag offload receives. When message arrives, it is copied into the user buffer (similar\n"
   "to eager protocol). The size values has to be equal or less than segment size.\n"
   "Also the value has to be bigger than UCX_TM_THRESH to take an effect." ,
   ucs_offsetof(ucp_config_t, ctx.tm_max_bcopy), UCS_CONFIG_TYPE_MEMUNITS},

  {"TM_FORCE_THRESH", "8192", /* TODO: calculate automaticlly */
   "Threshold for forcing tag matching offload mode. Every tag receive operation\n"
   "with buffer bigger than this threshold would force offloading of all uncompleted\n"
   "non-offloaded receive operations to the transport (e. g. operations with\n"
   "buffers below the UCX_TM_THRESH value). Offloading may be unsuccessful in certain\n"
   "cases (non-contig buffer, or sender wildcard).",
   ucs_offsetof(ucp_config_t, ctx.tm_force_thresh), UCS_CONFIG_TYPE_MEMUNITS},

  {"NUM_EPS", "auto",
   "An optimization hint of how many endpoints would be created on this context.\n"
   "Does not affect semantics, but only transport selection criteria and the\n"
   "resulting performance.\n"
   " If set to a value different from \"auto\" it will override the value passed\n"
   "to ucp_init()",
   ucs_offsetof(ucp_config_t, ctx.estimated_num_eps), UCS_CONFIG_TYPE_ULUNITS},

  {"RNDV_FRAG_SIZE", "256k",
   "RNDV fragment size \n",
   ucs_offsetof(ucp_config_t, ctx.rndv_frag_size), UCS_CONFIG_TYPE_MEMUNITS},
*/
impl Default for Configuration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			network_device_names: Default::default(),
			shared_memory_device_names: Default::default(),
			accelerated_device_names: Default::default(),
			self_device_names: Default::default(),
			transport_layers_to_use_if_available: Default::default(),
			memory_allocator_priority_set: Default::default(),
			threshold_for_switching_from_short_to_buffer_copy_protocol: Default::default(),
			threshold_for_switching_from_eager_to_rendezvous_protocol: Default::default(),
			message_size_threshold_to_start_using_the_rendezvous_protocol_is_case_the_calculated_threshold_is_zero_or_negative: Default::default(),
			rendezvous_protocol_and_eager_zero_copy_protocol_percentage_difference: Default::default(),
			threshold_for_switching_from_buffer_copy_protocol_to_zero_copy_protocol: Default::default(),
			estimation_of_buffer_copy_bandwidth: Default::default(),
			atomic_operations_synchronization: Default::default(),
			maximum_length_of_worker_name: Default::default(),
			prefer_spin_lock_over_mutex_when_multi_threading: Default::default(),
			threshold_for_using_tag_matching_offload_capabilities: Default::default(),
		}
	}
}

impl Configuration
{
	/// Creates a new Open UCX configuration wrapper.
	#[inline(always)]
	pub fn ucx_configuration_wrapper(&self) -> Result<UcxConfigurationWrapper, CouldNotConfigureUcxError>
	{
		let ucx_configuration_wrapper = UcxConfigurationWrapper::parse_environment_variables("")?;
		ucx_configuration_wrapper.modify(&self.network_device_names)?;
		ucx_configuration_wrapper.modify(&self.accelerated_device_names)?;
		ucx_configuration_wrapper.modify(&self.self_device_names)?;
		ucx_configuration_wrapper.modify(&self.transport_layers_to_use_if_available)?;
		ucx_configuration_wrapper.modify(&self.memory_allocator_priority_set)?;
		ucx_configuration_wrapper.modify(&self.threshold_for_switching_from_short_to_buffer_copy_protocol)?;
		ucx_configuration_wrapper.modify(&self.threshold_for_switching_from_eager_to_rendezvous_protocol)?;
		ucx_configuration_wrapper.modify(&self.message_size_threshold_to_start_using_the_rendezvous_protocol_is_case_the_calculated_threshold_is_zero_or_negative)?;
		ucx_configuration_wrapper.modify(&self.rendezvous_protocol_and_eager_zero_copy_protocol_percentage_difference)?;
		ucx_configuration_wrapper.modify(&self.threshold_for_switching_from_buffer_copy_protocol_to_zero_copy_protocol)?;
		ucx_configuration_wrapper.modify(&self.estimation_of_buffer_copy_bandwidth)?;
		ucx_configuration_wrapper.modify(&self.atomic_operations_synchronization)?;
		ucx_configuration_wrapper.modify(&self.maximum_length_of_worker_name)?;
		ucx_configuration_wrapper.modify(&self.prefer_spin_lock_over_mutex_when_multi_threading)?;
		ucx_configuration_wrapper.modify(&self.threshold_for_using_tag_matching_offload_capabilities)?;
		
		Ok(ucx_configuration_wrapper)
	}
}
