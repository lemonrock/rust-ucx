// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
pub struct ucs_global_opts_t
{
	pub log_level: ucs_log_level_t,
	pub log_file: *mut c_char,
	pub log_buffer_size: usize,
	pub log_data_size: usize,
	pub log_print_enable: c_int,
	pub mpool_fifo: c_int,
	pub handle_errors: c_uint,
	pub error_signals: ucs_global_opts_t__bindgen_ty_1,
	pub error_mail_to: *mut c_char,
	pub error_mail_footer: *mut c_char,
	pub gdb_command: *mut c_char,
	pub debug_signo: c_uint,
	pub log_level_trigger: ucs_log_level_t,
	pub async_max_events: c_uint,
	pub stats_dest: *mut c_char,
	pub stats_trigger: *mut c_char,
	pub tuning_path: *mut c_char,
	pub perf_stall_loops: usize,
	pub async_signo: c_uint,
	pub memtrack_dest: *mut c_char,
	pub profile_mode: c_uint,
	pub profile_file: *mut c_char,
	pub profile_log_size: usize,
	pub stats_filter: ucs_config_names_array_t,
	pub stats_format: ucs_stats_formats_t,
}

impl Default for ucs_global_opts_t
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ucs_global_opts_t
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ucs_global_opts_t {{ log_level: {:?}, log_file: {:?}, error_signals: {:?}, error_mail_to: {:?}, error_mail_footer: {:?}, gdb_command: {:?}, log_level_trigger: {:?}, stats_dest: {:?}, stats_trigger: {:?}, tuning_path: {:?}, memtrack_dest: {:?}, profile_file: {:?}, stats_filter: {:?}, stats_format: {:?} }}", self.log_level, self.log_file, self.error_signals, self.error_mail_to, self.error_mail_footer, self.gdb_command, self.log_level_trigger, self.stats_dest, self.stats_trigger, self.tuning_path, self.memtrack_dest, self.profile_file, self.stats_filter, self.stats_format)
	}
}
