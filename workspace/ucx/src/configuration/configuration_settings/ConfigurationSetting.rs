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
	($struct_name: ident, $configuration_name: expr, $default_type: ty, $default: expr) =>
	{
		#[allow(missing_docs)]
		#[derive(Debug, Clone)]
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
	}
}

configuration_setting!
{
	NetworkDeviceNames,
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
	SharedMemoryDeviceNames,
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
	AcceleratedDeviceNames,
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
	SelfDeviceNames,
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
	TransportLayersToUseIfAvailable,
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
	MemoryAllocatorPrioritySet,
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
	ThresholdForSwitchingFromShortToBufferCopyProtocol,
	"BCOPY_THRESH",
	MemoryUnit,
	{
		UnitLess(0)
	}
}

configuration_setting!
{
	ThresholdForSwitchingFromEagerToRendezvousProtocol,
	"RNDV_THRESH",
	MemoryUnit,
	{
		Automatic
	}
}

configuration_setting!
{
	MessageSizeThresholdToStartUsingTheRendezvousProtocolInCaseTheCalculatedThresholdIsZeroOrNegative,
	"RNDV_THRESH_FALLBACK",
	MemoryUnit,
	{
		Infinity
	}
}

configuration_setting!
{
	RendezvousProtocolAndEagerZeroCopyProtocolPercentageDifference,
	"RNDV_PERF_DIFF",
	f64,
	{
		1.0f64
	}
}

configuration_setting!
{
	ThresholdForSwitchingFromBufferCopyProtocolToZeroCopyProtocol,
	"ZCOPY_THRESH",
	MemoryUnit,
	{
		Automatic
	}
}

configuration_setting!
{
	EstimationOfBufferCopyBandwidth,
	"BCOPY_BW",
	MemoryUnit,
	{
		MegaBytes(5800)
	}
}

configuration_setting!
{
	AtomicOperationsSynchronization,
	"ATOMIC_MODE",
	AtomicOperationsSynchronizationMode,
	{
		guess
	}
}

configuration_setting!
{
	MaximumLengthOfWorkerName,
	"MAX_WORKER_NAME",
	u32,
	{
		32
	}
}

configuration_setting!
{
	PreferSpinLockOverMutexWhenMultiThreading,
	"USE_MT_MUTEX",
	bool,
	{
		false
	}
}

configuration_setting!
{
	ThresholdForUsingTagMatchingOffloadCapabilities,
	"TM_THRESH",
	MemoryUnit,
	{
		UnitLess(1024)
	}
}
