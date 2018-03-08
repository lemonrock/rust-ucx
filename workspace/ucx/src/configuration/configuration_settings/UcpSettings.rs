// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


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
/// UCX specific settings.
#[allow(missing_docs)]
#[serde(deny_unknown_fields, default)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UcpSettings
{
	pub network_device_names: UcpNetworkDeviceNames,
	pub shared_memory_device_names: UcpSharedMemoryDeviceNames,
	pub accelerated_device_names: UcpAcceleratedDeviceNames,
	pub self_device_names: UcpSelfDeviceNames,
	pub transport_layers_to_use_if_available: UcpTransportLayersToUseIfAvailable,
	pub memory_allocator_priority_set: UcpMemoryAllocatorPrioritySet,
	pub threshold_for_switching_from_short_to_buffer_copy_protocol: UcpThresholdForSwitchingFromShortToBufferCopyProtocol,
	pub threshold_for_switching_from_eager_to_rendezvous_protocol: UcpThresholdForSwitchingFromEagerToRendezvousProtocol,
	pub message_size_threshold_to_start_using_the_rendezvous_protocol_is_case_the_calculated_threshold_is_zero_or_negative: UcpMessageSizeThresholdToStartUsingTheRendezvousProtocolInCaseTheCalculatedThresholdIsZeroOrNegative,
	pub rendezvous_protocol_and_eager_zero_copy_protocol_percentage_difference: UcpRendezvousProtocolAndEagerZeroCopyProtocolPercentageDifference,
	pub threshold_for_switching_from_buffer_copy_protocol_to_zero_copy_protocol: UcpThresholdForSwitchingFromBufferCopyProtocolToZeroCopyProtocol,
	pub estimation_of_buffer_copy_bandwidth: UcpEstimationOfBufferCopyBandwidth,
	pub atomic_operations_synchronization: UcpAtomicOperationsSynchronization,
	pub maximum_length_of_worker_name: UcpMaximumLengthOfWorkerName,
	pub prefer_spin_lock_over_mutex_when_multi_threading: UcpPreferSpinLockOverMutexWhenMultiThreading,
	pub threshold_for_using_tag_matching_offload_capabilities: UcpThresholdForUsingTagMatchingOffloadCapabilities,
}

impl UcpSettings
{
	#[inline(always)]
	pub(crate) fn ucp_configuration_wrapper(&self) -> Result<UcpConfigurationWrapper, CouldNotConfigureUcpError>
	{
		let ucp_configuration_wrapper = UcpConfigurationWrapper::parse_environment_variables(None)?;
		ucp_configuration_wrapper.modify(&self.network_device_names)?;
		ucp_configuration_wrapper.modify(&self.accelerated_device_names)?;
		ucp_configuration_wrapper.modify(&self.self_device_names)?;
		ucp_configuration_wrapper.modify(&self.transport_layers_to_use_if_available)?;
		ucp_configuration_wrapper.modify(&self.memory_allocator_priority_set)?;
		ucp_configuration_wrapper.modify(&self.threshold_for_switching_from_short_to_buffer_copy_protocol)?;
		ucp_configuration_wrapper.modify(&self.threshold_for_switching_from_eager_to_rendezvous_protocol)?;
		ucp_configuration_wrapper.modify(&self.message_size_threshold_to_start_using_the_rendezvous_protocol_is_case_the_calculated_threshold_is_zero_or_negative)?;
		ucp_configuration_wrapper.modify(&self.rendezvous_protocol_and_eager_zero_copy_protocol_percentage_difference)?;
		ucp_configuration_wrapper.modify(&self.threshold_for_switching_from_buffer_copy_protocol_to_zero_copy_protocol)?;
		ucp_configuration_wrapper.modify(&self.estimation_of_buffer_copy_bandwidth)?;
		ucp_configuration_wrapper.modify(&self.atomic_operations_synchronization)?;
		ucp_configuration_wrapper.modify(&self.maximum_length_of_worker_name)?;
		ucp_configuration_wrapper.modify(&self.prefer_spin_lock_over_mutex_when_multi_threading)?;
		ucp_configuration_wrapper.modify(&self.threshold_for_using_tag_matching_offload_capabilities)?;
		
		Ok(ucp_configuration_wrapper)
	}
}
