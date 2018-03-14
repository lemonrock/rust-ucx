// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A configuration setting.
pub trait ConfigurationSetting
{
	#[doc(hidden)]
	#[inline(always)]
	fn name_and_value(&self) -> (&CStr, CString);
}

macro_rules! configuration_setting
{
	($configuration_wrapper: ident, $struct_name: ident, $configuration_name: expr, $default_type: ty, $default: expr) =>
	{
		#[allow(missing_docs)]
		#[derive(Serialize, Deserialize, Debug, Clone)]
		pub struct $struct_name(pub $default_type);

		impl Default for $struct_name
		{
			#[inline(always)]
			fn default() -> Self
			{
				$struct_name
				(
					$default
				)
			}
		}

		impl ConfigurationSetting for $struct_name
		{
			#[inline(always)]
			fn name_and_value(&self) -> (&CStr, CString)
			{
				(c_str!($configuration_name), self.0.convert())
			}
		}

		impl $configuration_wrapper for $struct_name
		{
		}
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpNetworkDeviceNames,
	"NET_DEVICES",
	HashSet<DeviceName>,
	{
		let mut set = HashSet::with_capacity(1);
		set.insert(AllDevices);
		set
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpSharedMemoryDeviceNames,
	"SHM_DEVICES",
	HashSet<DeviceName>,
	{
		let mut set = HashSet::with_capacity(1);
		set.insert(AllDevices);
		set
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpAcceleratedDeviceNames,
	"ACC_DEVICES",
	HashSet<DeviceName>,
	{
		let mut set = HashSet::with_capacity(1);
		set.insert(AllDevices);
		set
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpSelfDeviceNames,
	"SELF_DEVICES",
	HashSet<DeviceName>,
	{
		let mut set = HashSet::with_capacity(1);
		set.insert(AllDevices);
		set
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpTransportLayersToUseIfAvailable,
	"TLS",
	HashSet<TransportLayerCollectionName>,
	{
		let mut set = HashSet::with_capacity(1);
		set.insert(AllTransportLayers);
		set
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpMemoryAllocatorPrioritySet,
	"ALLOC_PRIO",
	IndexSet<MemoryAllocatorPriority>,
	{
		let mut set = IndexSet::with_capacity(7);
		set.insert(md(MemoryDomain::sysv));
		set.insert(md(MemoryDomain::posix));
		set.insert(huge);
		set.insert(thp);
		set.insert(md(MemoryDomain::Wildcard));
		set.insert(mmap);
		set.insert(heap);
		set
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpThresholdForSwitchingFromShortToBufferCopyProtocol,
	"BCOPY_THRESH",
	MemoryUnit,
	{
		UnitLess(0)
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpThresholdForSwitchingFromEagerToRendezvousProtocol,
	"RNDV_THRESH",
	MemoryUnit,
	{
		Automatic
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpMessageSizeThresholdToStartUsingTheRendezvousProtocolInCaseTheCalculatedThresholdIsZeroOrNegative,
	"RNDV_THRESH_FALLBACK",
	MemoryUnit,
	{
		Infinity
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpRendezvousProtocolAndEagerZeroCopyProtocolPercentageDifference,
	"RNDV_PERF_DIFF",
	f64,
	{
		1.0f64
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpThresholdForSwitchingFromBufferCopyProtocolToZeroCopyProtocol,
	"ZCOPY_THRESH",
	MemoryUnit,
	{
		Automatic
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpEstimationOfBufferCopyBandwidth,
	"BCOPY_BW",
	MemoryUnit,
	{
		MegaBytes(5800)
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpAtomicOperationsSynchronization,
	"ATOMIC_MODE",
	AtomicOperationsSynchronizationMode,
	{
		guess
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpMaximumLengthOfWorkerName,
	"MAX_WORKER_NAME",
	u32,
	{
		32
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpPreferSpinLockOverMutexWhenMultiThreading,
	"USE_MT_MUTEX",
	bool,
	{
		true
	}
}

configuration_setting!
{
	UcpConfigurationSetting,
	UcpThresholdForUsingTagMatchingOffloadCapabilities,
	"TM_THRESH",
	MemoryUnit,
	{
		UnitLess(1024)
	}
}

configuration_setting!
{
	UcmConfigurationSetting,
	UcmLogLevel,
	"LOG_LEVEL",
	UcsGlobalLogLevelSetting,
	{
		UcsGlobalLogLevelSetting::default()
	}
}

configuration_setting!
{
	UcmConfigurationSetting,
	UcmAllocationAlignment,
	"ALLOC_ALIGN",
	usize,
	{
		16
	}
}

configuration_setting!
{
	UcmConfigurationSetting,
	UcmEnableEvents,
	"EVENTS",
	bool,
	{
		true
	}
}

configuration_setting!
{
	UcmConfigurationSetting,
	UcmEnableMmapRelocate,
	"MMAP_RELOC",
	bool,
	{
		true
	}
}

configuration_setting!
{
	UcmConfigurationSetting,
	UcmEnableMallocHooks,
	"MALLOC_HOOKS",
	bool,
	{
		true
	}
}

configuration_setting!
{
	UcmConfigurationSetting,
	UcmEnableMallocRelocate,
	"MALLOC_RELOC",
	bool,
	{
		false
	}
}

configuration_setting!
{
	UcmConfigurationSetting,
	UcmEnableDynamicMMapThreshold,
	"DYNAMIC_MMAP_THRESH",
	bool,
	{
		true
	}
}

// This setting should not be used if ucx was built without CUDA support.
configuration_setting!
{
	UcmConfigurationSetting,
	UcmEnableCudaHooks,
	"CUDA_HOOKS",
	bool,
	{
		true
	}
}

configuration_setting!
{
	UcsGlobalConfigurationSetting,
	UcsGlobalLogLevel,
	"LOG_LEVEL",
	UcsGlobalLogLevelSetting,
	{
		UcsGlobalLogLevelSetting::default()
	}
}

//// The following substitutions are performed on this string:-
//// * `%p` - Replaced with process id.
//// * `%h` - Replaced with host name.
//// If empty (not null) then log tagged_messages are printed to standard out.
//configuration_setting!
//{
//	UcsGlobalConfigurationSetting,
//	UcsGlobalLogFile,
//	"LOG_FILE",
//	Option<String>,
//	{
//		None
//	}
//}

configuration_setting!
{
	UcsGlobalConfigurationSetting,
	UcsGlobalLogBuffer,
	"LOG_BUFFER",
	MemoryUnit,
	{
		MemoryUnit::Bytes(1024)
	}
}

configuration_setting!
{
	UcsGlobalConfigurationSetting,
	UcsGlobalLogDataSize,
	"LOG_DATA_SIZE",
	usize,
	{
		0
	}
}

configuration_setting!
{
	UcsGlobalConfigurationSetting,
	UcsGlobalEnableLogPrinting,
	"LOG_DATA_SIZE",
	bool,
	{
		false
	}
}

// Only available if configured with debugging data.
configuration_setting!
{
	UcsGlobalConfigurationSetting,
	UcsGlobalMemoryPoolFifoNotLifo,
	"MPOOL_FIFO",
	bool,
	{
		false
	}
}

/*
 {"HANDLE_ERRORS",
#if ENABLE_DEBUG_DATA
  "bt,freeze",
#else
  "bt",
#endif
  "Error handling mode. A combination of: 'bt' (print backtrace),\n"
  "'freeze' (freeze and wait for a debugger), 'debug' (attach debugger)",
  ucs_offsetof(ucs_global_opts_t, handle_errors),
  UCS_CONFIG_TYPE_BITMAP(ucs_handle_error_modes)},

 {"ERROR_SIGNALS", "SIGILL,SIGSEGV,SIGBUS,SIGFPE",
  "Signals which are considered an error indication and trigger error handling.",
  ucs_offsetof(ucs_global_opts_t, error_signals), UCS_CONFIG_TYPE_ARRAY(signo)},

 {"ERROR_MAIL_TO", "",
  "If non-empty, send mail notification for fatal errors.",
  ucs_offsetof(ucs_global_opts_t, error_mail_to), UCS_CONFIG_TYPE_STRING},

 {"ERROR_MAIL_FOOTER", "",
  "Footer for error report email",
  ucs_offsetof(ucs_global_opts_t, error_mail_footer), UCS_CONFIG_TYPE_STRING},

 {"GDB_COMMAND", "gdb -quiet",
  "If non-empty, attaches a gdb to the process in case of error, using the provided command.",
  ucs_offsetof(ucs_global_opts_t, gdb_command), UCS_CONFIG_TYPE_STRING},

 {"DEBUG_SIGNO", "SIGHUP",
  "Signal number which causes UCS to enter debug mode. Set to 0 to disable.",
  ucs_offsetof(ucs_global_opts_t, debug_signo), UCS_CONFIG_TYPE_SIGNO},

 {"LOG_LEVEL_TRIGGER", "fatal",
  "Log level to trigger error handling.",
  ucs_offsetof(ucs_global_opts_t, log_level_trigger), UCS_CONFIG_TYPE_ENUM(ucs_log_level_names)},

 {"ASYNC_MAX_EVENTS", "1024", /* TODO remove this; resize mpmc */
  "Maximal number of events which can be handled from one context",
  ucs_offsetof(ucs_global_opts_t, async_max_events), UCS_CONFIG_TYPE_UINT},

 {"ASYNC_SIGNO", "SIGALRM",
  "Signal number used for async signaling.",
  ucs_offsetof(ucs_global_opts_t, async_signo), UCS_CONFIG_TYPE_SIGNO},

#if ENABLE_STATS
 {"STATS_DEST", "",
  "Destination to send statistics to. If the value is empty, statistics are\n"
  "not reported. Possible values are:\n"
  "  udp:<host>[:<port>]   - send over UDP to the given host:port.\n"
  "  stdout                - print to standard output.\n"
  "  stderr                - print to standard error.\n"
  "  file:<filename>[:bin] - save to a file (%h: host, %p: pid, %c: cpu, %t: time, %u: user, %e: exe)",
  ucs_offsetof(ucs_global_opts_t, stats_dest), UCS_CONFIG_TYPE_STRING},

 {"STATS_TRIGGER", "exit",
  "Trigger to dump statistics:\n"
  "  exit              - dump just before program exits.\n"
  "  signal:<signo>    - dump when process is signaled.\n"
  "  timer:<interval>  - dump in specified intervals (in seconds).",
  ucs_offsetof(ucs_global_opts_t, stats_trigger), UCS_CONFIG_TYPE_STRING},

  {"STATS_FILTER", "*",
   "Used for filter counters summary.\n"
   "Comma-separated list of glob patterns specifying counters.\n"
   "Statistics summary will contain only the matching counters.\n"
   "The order is not meaningful.\n"
   "Each expression in the list may contain any of the following wildcard:\n"
   "  *     - matches any number of any characters including none.\n"
   "  ?     - matches any single character.\n"
   "  [abc] - matches one character given in the bracket.\n"
   "  [a-z] - matches one character from the range given in the bracket.",
   ucs_offsetof(ucs_global_opts_t, stats_filter), UCS_CONFIG_TYPE_STRING_ARRAY},

  {"STATS_FORMAT", "full",
   "Statistics format parameter:\n"
   "  full    - each counter will be displayed in a separate line \n"
   "  agg     - like full but there will also be an aggregation between similar counters\n"
   "  summary - all counters will be printed in the same line.",
   ucs_offsetof(ucs_global_opts_t, stats_format), UCS_CONFIG_TYPE_ENUM(ucs_stats_formats_names)},

#endif

#if ENABLE_MEMTRACK
 {"MEMTRACK_DEST", "",
  "Destination to output memory tracking report to. If the value is empty,\n"
  "results are not reported. Possible values are:\n"
  "  file:<filename>   - save to a file (%h: host, %p: pid, %c: cpu, %t: time, %u: user, %e: exe)\n"
  "  stdout            - print to standard output.\n"
  "  stderr            - print to standard error.\n",
  ucs_offsetof(ucs_global_opts_t, memtrack_dest), UCS_CONFIG_TYPE_STRING},
#endif

  {"PROFILE_MODE", "",
   "Profile collection modes. If none is specified, profiling is disabled.\n"
   " - log   - Record all timestamps.\n"
   " - accum - Accumulate measurements per location.\n",
   ucs_offsetof(ucs_global_opts_t, profile_mode),
   UCS_CONFIG_TYPE_BITMAP(ucs_profile_mode_names)},

  {"PROFILE_FILE", "",
   "File name to dump profiling data to.\n"
   "Substitutions: %h: host, %p: pid, %c: cpu, %t: time, %u: user, %e: exe.\n",
   ucs_offsetof(ucs_global_opts_t, profile_file), UCS_CONFIG_TYPE_STRING},

  {"PROFILE_LOG_SIZE", "4m",
   "Maximal size of profiling log. New records will replace old records.",
   ucs_offsetof(ucs_global_opts_t, profile_log_size), UCS_CONFIG_TYPE_MEMUNITS},


*/


