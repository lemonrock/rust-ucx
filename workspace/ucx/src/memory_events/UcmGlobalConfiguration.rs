// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// UCM global configuration.
///
/// Can only be modified prior to calls made to memory event functionality.
///
/// The configuration is initially populated from environment variables prefixed `MEM_`.
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UcmGlobalConfiguration;

impl UcmGlobalConfiguration
{
	/// Get log level.
	/// Messages with a level higher or equal to the selected log level will be printed.
	#[inline(always)]
	pub fn get_log_level(&self) -> Result<UcsGlobalLogLevelSetting, ()>
	{
		UcsGlobalLogLevelSetting::from_ucs_log_level_t(self.values().log_level)
	}
	
	/// Set log level.
	/// Messages with a level higher or equal to the selected log level will be printed.
	#[inline(always)]
	pub fn set_log_level(&self, ucs_log_level: UcsGlobalLogLevelSetting)
	{
		self.values_mut().log_level = ucs_log_level.to_ucs_log_level_t()
	}
	
	/// Get if events are enabled.
	#[inline(always)]
	pub fn are_events_enabled(&self) -> bool
	{
		self.values().enable_events.from_c_bool()
	}
	
	/// Set if events are enabled.
	#[inline(always)]
	pub fn set_events_are_enabled(&self, enabled: bool)
	{
		self.values_mut().enable_events = enabled.to_c_bool()
	}
	
	/// Get if mmap relocation is enabled.
	#[inline(always)]
	pub fn are_mmap_relocation_enabled(&self) -> bool
	{
		self.values().enable_mmap_reloc.from_c_bool()
	}
	
	/// Set if mmap relocation is enabled.
	#[inline(always)]
	pub fn set_mmap_relocation_is_enabled(&self, enabled: bool)
	{
		self.values_mut().enable_mmap_reloc = enabled.to_c_bool()
	}
	
	/// Get if malloc hooks are enabled.
	#[inline(always)]
	pub fn are_malloc_hooks_enabled(&self) -> bool
	{
		self.values().enable_malloc_hooks.from_c_bool()
	}
	
	/// Set if malloc hooks are enabled.
	#[inline(always)]
	pub fn set_malloc_hooks_are_enabled(&self, enabled: bool)
	{
		self.values_mut().enable_malloc_hooks = enabled.to_c_bool()
	}
	
	/// Get if malloc relocation is enabled.
	#[inline(always)]
	pub fn is_malloc_reallocation_enabled(&self) -> bool
	{
		self.values().enable_malloc_reloc.from_c_bool()
	}
	
	/// Set if malloc relocation is enabled.
	#[inline(always)]
	pub fn set_malloc_relocation_is_enabled(&self, enabled: bool)
	{
		self.values_mut().enable_malloc_reloc = enabled.to_c_bool()
	}
	
	/// Get if CUDA relocation is enabled.
	#[inline(always)]
	pub fn is_cuda_reallocation_enabled(&self) -> bool
	{
		self.values().enable_cuda_reloc.from_c_bool()
	}
	
	/// Set if CUDA relocation is enabled.
	#[inline(always)]
	pub fn set_cuda_reallocation_is_enabled(&self, enabled: bool)
	{
		self.values_mut().enable_cuda_reloc = enabled.to_c_bool()
	}
	
	/// Get if the dynamic mmap threshold is enabled.
	#[inline(always)]
	pub fn is_dynamic_mmap_threshold_enabled(&self) -> bool
	{
		self.values().enable_dynamic_mmap_thresh.from_c_bool()
	}
	
	/// Set if the dynamic mmap threshold is enabled.
	#[inline(always)]
	pub fn set_dynamic_mmap_threshold_is_enabled(&self, enabled: bool)
	{
		self.values_mut().enable_dynamic_mmap_thresh = enabled.to_c_bool()
	}
	
	/// Get allocation alignment.
	/// Should be a power of two, but nothing enforces this in the UCM logic internally.
	/// Default is 16.
	#[inline(always)]
	pub fn get_allocation_alignment(&self) -> usize
	{
		self.values().alloc_alignment
	}
	
	/// Set allocation alignment.
	/// Must be a power of two.
	#[inline(always)]
	pub fn set_allocation_alignment(&self, allocation_alignment: usize)
	{
		assert!(allocation_alignment.is_power_of_two(), "allocation_alignment '{}' is not power of two", allocation_alignment);
		self.values_mut().alloc_alignment = allocation_alignment
	}
	
	#[inline(always)]
	fn values(&self) -> &'static ucm_global_config
	{
		unsafe { &ucm_global_opts }
	}
	
	#[inline(always)]
	fn values_mut(&self) -> &'static mut ucm_global_config
	{
		unsafe { &mut ucm_global_opts }
	}
}
