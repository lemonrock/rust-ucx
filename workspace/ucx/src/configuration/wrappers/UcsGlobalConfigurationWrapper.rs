// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A wrapper around UCP configuration for an `ApplicationContext`.
/// The configuration is initially populated from environment variables.
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UcsGlobalConfigurationWrapper;

impl Debug for UcsGlobalConfigurationWrapper
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		self.debug_fmt(f)
	}
}

impl PrintInformation for UcsGlobalConfigurationWrapper
{
	const DebugName: &'static str = "UcpConfigurationWrapper";
	
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		let print_flags = ucs_config_print_flags_t::CONFIG | ucs_config_print_flags_t::DOC | ucs_config_print_flags_t::HEADER | ucs_config_print_flags_t::HIDDEN;
		
		unsafe { ucs_global_opts_print(stream, print_flags) };
	}
}

impl ConfigurationWrapper for UcsGlobalConfigurationWrapper
{
	#[inline(always)]
	unsafe fn ucx_config_modify(&self, name: *const c_char, value: *const c_char) -> ucs_status_t
	{
		ucs_global_opts_set_value(name, value)
	}
}

impl UcsGlobalConfigurationWrapper
{
	/// Modify configuration.
	#[inline(always)]
	pub fn modify<Setting: UcmConfigurationSetting>(&self, configuration_setting: &Setting) -> Result<(), ConfigurationModifyError>
	{
		self.modify_(configuration_setting)
	}
	
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
	
	/// None implies output is to standard out.
	/// May contains `%p` and `%h` substitution tokens.
	/// Not thread safe; another thread can cause a de-allocation of the underlying global value.
	#[inline(always)]
	pub fn get_log_file_path_template(&self) -> Option<CString>
	{
		Self::unsafe_string_value(self.values().log_file)
	}
	
	/// Get log buffer size in bytes for a single log message.
	#[inline(always)]
	pub fn get_log_buffer_size(&self) -> usize
	{
		self.values().log_buffer_size
	}
	
	/// Set log buffer size in bytes for a single log message.
	#[inline(always)]
	pub fn set_log_buffer_size(&self, log_buffer_size: usize)
	{
		self.values_mut().log_buffer_size = log_buffer_size
	}
	
	/// Get log data size in bytes.
	/// How much packet payload to print, at most, in data mode.
	#[inline(always)]
	pub fn get_log_data_size(&self) -> usize
	{
		self.values().log_data_size
	}
	
	/// Set log data size.
	/// How much packet payload to print, at most, in data mode.
	#[inline(always)]
	pub fn set_log_data_size(&self, log_data_size: usize)
	{
		self.values_mut().log_data_size = log_data_size
	}
	
	/// Get is log printing enabled?
	/// This option is intended for use by the library developers.
	#[inline(always)]
	pub fn get_is_log_printing_enabled(&self) -> bool
	{
		self.values().log_print_enable != 0
	}
	
	/// Set is log printing enabled?
	/// This option is intended for use by the library developers.
	#[inline(always)]
	pub fn set_log_printing_enabled(&self, log_printing_enabled: bool)
	{
		self.values_mut().log_print_enable = if log_printing_enabled
		{
			1
		}
		else
		{
			0
		}
	}
	
	/// Only available if configured, and only for debugging should the memory pool (mpool) be a FIFO.
	#[inline(always)]
	pub fn get_is_memory_pool_a_fifo_not_a_lifo(&self) -> bool
	{
		self.values().mpool_fifo != 0
	}
	
	/// Default is just backtrace.
	#[inline(always)]
	pub fn get_handle_errors(&self) -> ucs_handle_error_t
	{
		unsafe { transmute(self.values().handle_errors) }
	}
	
	/// Default is just backtrace.
	#[inline(always)]
	pub fn set_handle_errors(&self, handle_errors: ucs_handle_error_t)
	{
		self.values_mut().handle_errors = unsafe { transmute(handle_errors) }
	}
	
	/// Signals which are considered an error indication and trigger error handling.
	/// Not thread safe; another thread can cause a modification of the underlying global value.
	/// SignalNumbers are architecture and Operating System specific.
	#[inline(always)]
	pub fn get_error_signals(&self) -> &[SignalNumber]
	{
		let error_signals = &self.values().error_signals;
		
		let pointer = error_signals.signals as *const _ as *const SignalNumber;
		let count = error_signals.count as usize;
		
		unsafe { from_raw_parts(pointer, count) }
	}
	
	/// Not thread safe; another thread can cause a de-allocation of the underlying global value.
	#[inline(always)]
	pub fn get_error_mail_to(&self) -> Option<CString>
	{
		Self::unsafe_string_value(self.values().error_mail_to)
	}
	
	/// Not thread safe; another thread can cause a de-allocation of the underlying global value.
	#[inline(always)]
	pub fn get_error_mail_footer(&self) -> Option<CString>
	{
		Self::unsafe_string_value(self.values().error_mail_footer)
	}
	
	/// Not thread safe; another thread can cause a de-allocation of the underlying global value.
	#[inline(always)]
	pub fn get_gdb_command(&self) -> Option<CString>
	{
		Self::unsafe_string_value(self.values().gdb_command)
	}
	
	/// Get debug signal.
	/// SignalNumbers are architecture and Operating System specific.
	#[inline(always)]
	pub fn get_debug_signal(&self) -> SignalNumber
	{
		self.values().debug_signo
	}
	
	/// Signal number which causes UCS to enter debug mode.
	/// Set to 0 to disable.
	/// SignalNumbers are architecture and Operating System specific.
	#[inline(always)]
	pub fn set_debug_signal(&self, signal: SignalNumber)
	{
		self.values_mut().debug_signo = signal
	}
	
	/// Get Log level to trigger error handling.
	#[inline(always)]
	pub fn get_log_level_to_trigger_error_handling(&self) -> Result<UcsGlobalLogLevelSetting, ()>
	{
		UcsGlobalLogLevelSetting::from_ucs_log_level_t(self.values().log_level_trigger)
	}
	
	/// Set Log level to trigger error handling.
	#[inline(always)]
	pub fn set_log_level_to_trigger_error_handling(&self, ucs_log_level: UcsGlobalLogLevelSetting)
	{
		self.values_mut().log_level_trigger = ucs_log_level.to_ucs_log_level_t()
	}
	
//	/// Get maximum asynchronous events.
//	/// Maximal number of events which can be handled from one context.
//	/// Will be removed in future release.
//	#[inline(always)]
//	pub fn get_maximum_asynchronous_events(&self) -> u32
//	{
//		self.values().async_max_events
//	}
//
//	/// Set maximum asynchronous events.
//	/// Maximal number of events which can be handled from one context.
//	/// Will be removed in future release.
//	#[inline(always)]
//	pub fn set_maximum_asynchronous_events(&self, maximum_asynchronous_events: u32)
//	{
//		self.values_mut().async_max_events = maximum_asynchronous_events
//	}
//
//	/// Only valid if ucx was compiled with statistics.
//	/// Not thread safe; another thread can cause a de-allocation of the underlying global value.
//	#[inline(always)]
//	pub fn get_statistics_destination(&self) -> Option<CString>
//	{
//		Self::unsafe_string_value(self.values().stats_dest)
//	}
//
//	/// Only valid if ucx was compiled with statistics.
//	/// Not thread safe; another thread can cause a de-allocation of the underlying global value.
//	#[inline(always)]
//	pub fn get_statistics_trigger(&self) -> Option<CString>
//	{
//		Self::unsafe_string_value(self.values().stats_trigger)
//	}
	
	/// Not thread safe; another thread can cause a de-allocation of the underlying global value.
	#[inline(always)]
	pub fn get_tuning_file_path(&self) -> Option<CString>
	{
		Self::unsafe_string_value(self.values().tuning_path)
	}
	
	/// Get number of performance stall loops to perform.
	#[inline(always)]
	pub fn get_performance_stall_loops(&self) -> usize
	{
		self.values().perf_stall_loops
	}
	
	/// Set number of performance stall loops to perform.
	#[inline(always)]
	pub fn set_performance_stall_loops(&self, performance_stall_loops: usize)
	{
		self.values_mut().perf_stall_loops = performance_stall_loops
	}
	
	/// Get asynchronous signal.
	/// SignalNumbers are architecture and Operating System specific.
	#[inline(always)]
	pub fn get_asynchronous_signal(&self) -> SignalNumber
	{
		self.values().async_signo
	}
	
	/// Set asynchronous signal.
	/// SignalNumbers are architecture and Operating System specific.
	#[inline(always)]
	pub fn set_asynchronous_signal(&self, signal: SignalNumber)
	{
		self.values_mut().async_signo = signal
	}
	
//	/// Only valid if ucx was compiled with memory tracking.
//	/// Not thread safe; another thread can cause a de-allocation of the underlying global value.
//	/// If prefixed with `file:` then a file path with subtitutions %h: host, %p: pid, %c: cpu, %t: time, %u: user, %e: exe
//	/// Otherwise can be `stdout` or `stderr`.
//	#[inline(always)]
//	pub fn get_memtrack_destination_file_path_template(&self) -> Option<CString>
//	{
//		Self::unsafe_string_value(self.values().memtrack_dest)
//	}
	
	/// Profile collection modes.
	/// If none is specified, profiling is disabled.
	/// Supposedly a bit field, but seems to be an anonymous enum.
	#[inline(always)]
	pub fn get_profile_mode(&self) -> u32
	{
		self.values().profile_mode
	}
	
	/// Only valid if ucx was compiled with profiling.
	/// Not thread safe; another thread can cause a de-allocation of the underlying global value.
	/// Substitutions: %h: host, %p: pid, %c: cpu, %t: time, %u: user, %e: exe.
	#[inline(always)]
	pub fn get_profile_file_path_template(&self) -> Option<CString>
	{
		Self::unsafe_string_value(self.values().profile_file)
	}
	
	/// Get maximal size of profiling log.
	/// New records will replace old records.
	pub fn get_profile_log_maximal_size(&self) -> usize
	{
		self.values().profile_log_size
	}
	
	/// Set maximal size of profiling log.
	/// New records will replace old records.
	pub fn set_profile_log_maximal_size(&self, profile_log_maximal_size: usize)
	{
		self.values_mut().profile_log_size = profile_log_maximal_size
	}
	
//	pub stats_filter: ucs_config_names_array_t,
//	pub stats_format: ucs_stats_formats_t,
	
	#[inline(always)]
	fn unsafe_string_value(raw: *mut c_char) -> Option<CString>
	{
		if raw.is_null()
		{
			return None;
		}
		let c_str = unsafe { CStr::from_ptr(raw) };
		if c_str.to_bytes().is_empty()
		{
			None
		}
		else
		{
			Some(c_str.to_owned())
		}
	}
	
	#[inline(always)]
	fn values(&self) -> &'static ucs_global_opts_t
	{
		unsafe { &ucs_global_opts }
	}
	
	#[inline(always)]
	fn values_mut(&self) -> &'static mut ucs_global_opts_t
	{
		unsafe { &mut ucs_global_opts }
	}
	
	// Get a value: ucs_global_opts_get_value
}
