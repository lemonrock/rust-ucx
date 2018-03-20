// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Wraps up part of UCT's approach to configuration.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum MemoryDomainComponentAndTransportLayer
{
	/// CUDA copy.
	CudaCopy,
	
	/// CUDA GDR copy.
	CudaGdrCopy,
	
	/// InfiniBand.
	InfiniBand(InfiniBandVariant),
	// From `ib_md.h`
	
	// REG_METHODS	rcache,odp,direct	List of registration methods in order of preference. Supported methods are:
	//	odp         - implicit on-demand paging
	//	rcache      - userspace registration cache
	//	direct      - direct registration
	// ? Not a setting but for rcache ? RCACHE_ADDR_ALIGN=16
	// MEM_REG_OVERHEAD	16ns	Memory registration overhead
	// MEM_REG_GROWTH	0.06ns	Memory registration growth rate
	// FORK_INIT try Initialize a fork-safe IB library with ibv_fork_init().
	//
	// and lots more...
	
	/// RDMA using unaccelerated libibverbs library (however, it is *not* necessarily over InfiniBand).
	RdmaCommunicationManager,
	// ADDR_RESOLVE_TIMEOUT	500ms	Time to wait for address resolution to complete.
	
	/// RoCM.
	RadeonOpenCompute,
	
	/// Cross-Memory-Attach, aka `process_vm_readv` et al.
	CrossMemoryAttach,
	
	/// KNEM Cross-Memory-Attach (An out-of-tree Linux kernel module from <http://knem.gforge.inria.fr/>).
	CrossMemoryAttach_KNEM,
	
	/// Memory-Mapped Shared Memory.
	MemoryMapped(MemoryMappedVariant),
	
	/// A special form of shared memory in which local and remote is the same.
	///
	/// Also known as 'self'.
	LoopbackToSelf,
	
	/// TCP.
	TCP,
	
	/// Cray userspace generic network interface ('uGNI').
	CrayUserspaceGenericNetworkInterface(CrayUserspaceGenericNetworkInterfaceVariant),
}

impl MemoryDomainComponentAndTransportLayer
{
	#[inline(always)]
	fn memory_domain_configuration(&self) -> Result<MemoryDomainComponentConfiguration, ErrorCode>
	{
		let memory_domain_name = self.memory_domain_component_name();
		
		// Going forward, the safe way to do this is probably to use CString for environment variable names and values in JSON.
		let environment_variable_prefix = unsafe { CStr::from_ptr(b"\0" as *const u8 as *const c_char) };
		
		MemoryDomainComponentConfiguration::read_from_environment(memory_domain_name, &environment_variable_prefix)
	}
	
	/// UCT name for FFI.
	///
	/// No more than 16 characters long (excluding final `\0`).
	#[inline(always)]
	pub fn memory_domain_component_name(&self) -> &'static CStr
	{
		use self::MemoryDomainComponentAndTransportLayer::*;
		
		match *self
		{
			CudaCopy => unsafe { CStr::from_ptr(b"cuda_cpy\0" as *const u8 as *const c_char) },
			
			CudaGdrCopy => unsafe { CStr::from_ptr(b"gdr_copy\0" as *const u8 as *const c_char) },
			
			InfiniBand(..) => unsafe { CStr::from_ptr(b"ib\0" as *const u8 as *const c_char) },
			
			RdmaCommunicationManager => unsafe { CStr::from_ptr(b"rdmacm\0" as *const u8 as *const c_char) },
			
			RadeonOpenCompute => unsafe { CStr::from_ptr(b"rocm\0" as *const u8 as *const c_char) },
			
			CrossMemoryAttach => unsafe { CStr::from_ptr(b"cma\0" as *const u8 as *const c_char) },
			
			CrossMemoryAttach_KNEM => unsafe { CStr::from_ptr(b"knem\0" as *const u8 as *const c_char) },
			
			MemoryMapped(ref variant) => variant.memory_domain_component_name(),
			
			LoopbackToSelf => unsafe { CStr::from_ptr(b"self\0" as *const u8 as *const c_char) },
			
			TCP => unsafe { CStr::from_ptr(b"tcp\0" as *const u8 as *const c_char) },
			
			CrayUserspaceGenericNetworkInterface(..) => unsafe { CStr::from_ptr(b"ugni\0" as *const u8 as *const c_char) },
		}
	}
	
	/// UCT configuration prefix.
	#[inline(always)]
	pub fn memory_domain_configuration_prefix(&self) -> &'static CStr
	{
		use self::MemoryDomainComponentAndTransportLayer::*;
		
		match *self
		{
			CudaCopy => unsafe { CStr::from_ptr(b"CUDA_COPY_\0" as *const u8 as *const c_char) },
			
			CudaGdrCopy => unsafe { CStr::from_ptr(b"GDR_COPY_\0" as *const u8 as *const c_char) },
			
			InfiniBand(..) => unsafe { CStr::from_ptr(b"IB_\0" as *const u8 as *const c_char) },
			
			RdmaCommunicationManager => unsafe { CStr::from_ptr(b"RDMACM_\0" as *const u8 as *const c_char) },
			
			RadeonOpenCompute => unsafe { CStr::from_ptr(b"ROCM_MD_\0" as *const u8 as *const c_char) },
			
			CrossMemoryAttach => unsafe { CStr::from_ptr(b"CMA_\0" as *const u8 as *const c_char) },
			
			CrossMemoryAttach_KNEM => unsafe { CStr::from_ptr(b"KNEM_\0" as *const u8 as *const c_char) },
			
			MemoryMapped(ref variant) => variant.memory_domain_configuration_prefix(),
			
			LoopbackToSelf => unsafe { CStr::from_ptr(b"SELF_\0" as *const u8 as *const c_char) },
			
			TCP => unsafe { CStr::from_ptr(b"TCP_\0" as *const u8 as *const c_char) },
			
			CrayUserspaceGenericNetworkInterface(..) => unsafe { CStr::from_ptr(b"UGNI_\0" as *const u8 as *const c_char) },
		}
	}
	/// UCT name for FFI.
	///
	/// No more than 10 characters long (excluding final `\0`).
	#[inline(always)]
	pub fn transport_layer_name(&self) -> &'static CStr
	{
		use self::MemoryDomainComponentAndTransportLayer::*;
		
		match *self
		{
			// Also UCT_CUDA_DEV_NAME: `cudacopy0`
			CudaCopy => unsafe { CStr::from_ptr(b"cuda_copy\0" as *const u8 as *const c_char) },
			
			// Also UCT_CUDA_DEV_NAME: `gdrcopy0`
			CudaGdrCopy => unsafe { CStr::from_ptr(b"gdr_copy\0" as *const u8 as *const c_char) },
			
			// Device names are of the form `%s:%d` <- uct_ib_device_name(dev), port_num
			InfiniBand(ref variant) => variant.transport_layer_name(),
			
			RdmaCommunicationManager => unsafe { CStr::from_ptr(b"rdmacm\0" as *const u8 as *const c_char) },
			
			RadeonOpenCompute => unsafe { CStr::from_ptr(b"rocm\0" as *const u8 as *const c_char) },
			
			CrossMemoryAttach => unsafe { CStr::from_ptr(b"cma\0" as *const u8 as *const c_char) },
			
			CrossMemoryAttach_KNEM => unsafe { CStr::from_ptr(b"knem\0" as *const u8 as *const c_char) },
			
			MemoryMapped(..) => unsafe { CStr::from_ptr(b"mm\0" as *const u8 as *const c_char) },
			
			LoopbackToSelf => unsafe { CStr::from_ptr(b"self\0" as *const u8 as *const c_char) },
			
			TCP => unsafe { CStr::from_ptr(b"tcp\0" as *const u8 as *const c_char) },
			
			CrayUserspaceGenericNetworkInterface(ref variant) => variant.transport_layer_name(),
		}
	}
	
	/// UCT configuration prefix.
	#[inline(always)]
	pub fn transport_layer_configuration_prefix(&self) -> &'static CStr
	{
		use self::MemoryDomainComponentAndTransportLayer::*;
		
		match *self
		{
			CudaCopy => unsafe { CStr::from_ptr(b"CUDA_COPY_\0" as *const u8 as *const c_char) },
			
			CudaGdrCopy => unsafe { CStr::from_ptr(b"GDR_COPY_\0" as *const u8 as *const c_char) },
			
			InfiniBand(ref variant) => variant.transport_layer_configuration_prefix(),
			
			RdmaCommunicationManager => unsafe { CStr::from_ptr(b"RDMACM_\0" as *const u8 as *const c_char) },
			
			RadeonOpenCompute => unsafe { CStr::from_ptr(b"ROCM_TL_\0" as *const u8 as *const c_char) },
			
			// No prefix.
			CrossMemoryAttach => unsafe { CStr::from_ptr(b"\0" as *const u8 as *const c_char) },
			
			// No prefix.
			CrossMemoryAttach_KNEM => unsafe { CStr::from_ptr(b"\0" as *const u8 as *const c_char) },
			
			MemoryMapped(..) => unsafe { CStr::from_ptr(b"MM_\0" as *const u8 as *const c_char) },
			
			LoopbackToSelf => unsafe { CStr::from_ptr(b"SELF_\0" as *const u8 as *const c_char) },
			
			TCP => unsafe { CStr::from_ptr(b"TCP_\0" as *const u8 as *const c_char) },
			
			CrayUserspaceGenericNetworkInterface(ref variant) => variant.transport_layer_configuration_prefix(),
		}
	}
	
	/// Device kind (network, intra-node, accelerator or loopback).
	#[inline(always)]
	pub fn device_kind(&self) -> DeviceKind
	{
		use self::MemoryDomainComponentAndTransportLayer::*;
		use self::DeviceKind::*;
		
		match *self
		{
			CudaCopy => Accelerator,
			
			CudaGdrCopy => Accelerator,
			
			InfiniBand(..) => Network,
			
			RdmaCommunicationManager => Network,
			
			RadeonOpenCompute => Accelerator,
			
			CrossMemoryAttach => IntraNode,
			
			CrossMemoryAttach_KNEM => IntraNode,
			
			MemoryMapped(..) => IntraNode,
			
			LoopbackToSelf => Loopback,
			
			TCP => Network,
			
			CrayUserspaceGenericNetworkInterface(..) => Network,
		}
	}
}
