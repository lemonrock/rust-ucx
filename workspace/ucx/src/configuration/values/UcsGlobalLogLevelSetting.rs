// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// UCS Log Level.
/// Defaults to `warn`.
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UcsGlobalLogLevelSetting
{
	/// Fatal.
	fatal,
	
	/// Error.
	error,
	
	/// Warn (the default).
	warn,
	
	/// Information.
	info,
	
	/// Debug.
	debug,
	
	/// Trace.
	trace,
	
	/// Trace Requests (not supported by all UCS API entry points and so replaced with `trace`)
	trace_request,
	
	/// Trace data.
	trace_data,
	
	/// Trace asynchronous operations (not supported by all UCS API entry points and so replaced with `trace`)
	trace_asynchronous_operations,
	
	/// Trace functions.
	trace_function,
	
	/// Trace poll.
	trace_poll,
	
	/// Print (not supported by all UCS API entry points and so replaced with `trace`)
	print,
}

impl Default for UcsGlobalLogLevelSetting
{
	#[inline(always)]
	fn default() -> Self
	{
		UcsGlobalLogLevelSetting::warn
	}
}

impl ConfigurationValueConverter for UcsGlobalLogLevelSetting
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		CString::new(self.to_str()).unwrap()
	}
}

impl UcsGlobalLogLevelSetting
{
	#[inline(always)]
	fn to_str(&self) -> &'static str
	{
		use self::UcsGlobalLogLevelSetting::*;
		
		match *self
		{
			fatal => "fatal",
			
			error => "error",
			
			warn => "warn",
			
			info => "info",
			
			debug => "debug",
			
			trace => "trace",
			
			trace_request => "trace",
			
			trace_data => "data",
			
			trace_asynchronous_operations => "trace",
			
			trace_function => "func",
			
			trace_poll => "poll",
			
			print => "trace",
		}
	}
	
	/// Convert to `ucs_log_level_t`
	#[inline(always)]
	pub fn to_ucs_log_level_t(self) -> ucs_log_level_t
	{
		use self::UcsGlobalLogLevelSetting::*;
		use self::ucs_log_level_t::*;
		
		match self
		{
			fatal => UCS_LOG_LEVEL_FATAL,
			
			error => UCS_LOG_LEVEL_ERROR,
			
			warn => UCS_LOG_LEVEL_WARN,
			
			info => UCS_LOG_LEVEL_INFO,
			
			debug => UCS_LOG_LEVEL_DEBUG,
			
			trace => UCS_LOG_LEVEL_TRACE,
			
			trace_request => UCS_LOG_LEVEL_TRACE_REQ,
			
			trace_data => UCS_LOG_LEVEL_TRACE_DATA,
			
			trace_asynchronous_operations => UCS_LOG_LEVEL_TRACE_ASYNC,
			
			trace_function => UCS_LOG_LEVEL_TRACE_FUNC,
			
			trace_poll => UCS_LOG_LEVEL_TRACE_POLL,
			
			print => UCS_LOG_LEVEL_PRINT,
		}
	}
	
	/// Converts from `ucs_log_level_t`.
	/// Returns an error for an invalid value.
	#[inline(always)]
	pub fn from_ucs_log_level_t(log_level: ucs_log_level_t) -> Result<Self, ()>
	{
		use self::UcsGlobalLogLevelSetting::*;
		use self::ucs_log_level_t::*;
		
		let value = log_level as u32;
		// 11 is 'UCS_LOG_LEVEL_LAST'
		if value == 11 || value > (UCS_LOG_LEVEL_PRINT as u32)
		{
			Err(())
		}
		else
		{
			let matched = match log_level
			{
				UCS_LOG_LEVEL_FATAL => fatal,
				
				UCS_LOG_LEVEL_ERROR => error,
				
				UCS_LOG_LEVEL_WARN => warn,
				
				UCS_LOG_LEVEL_INFO => info,
				
				UCS_LOG_LEVEL_DEBUG => debug,
				
				UCS_LOG_LEVEL_TRACE => trace,
				
				UCS_LOG_LEVEL_TRACE_REQ => trace_request,
				
				UCS_LOG_LEVEL_TRACE_DATA => trace_data,
				
				UCS_LOG_LEVEL_TRACE_ASYNC => trace_asynchronous_operations,
				
				UCS_LOG_LEVEL_TRACE_FUNC => trace_function,
				
				UCS_LOG_LEVEL_TRACE_POLL => trace_poll,
				
				UCS_LOG_LEVEL_PRINT => print,
			};
			Ok(matched)
		}
	}
}
