// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


// Used in `static mut uct_md_components_list`.
/// Wraps up part of a ghastly approach used for configuration.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum MemoryDomainComponent
{
	/// CUDA copy.
	CudaCopy,
	
	/// CUDA GDR copy.
	CudaGdrCopy,
	
	/// InfiniBand.
	InfiniBand,
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
	
	/// InfiniBand RDMA CM.
	InfiniBandRdmaConnectionManager,
	// ADDR_RESOLVE_TIMEOUT	500ms	Time to wait for address resolution to complete.
	
	/// RoCM.
	RadeonOpenCompute,
	
	/// Cross-Memory-Attach, aka `process_vm_readv` et al.
	CrossMemoryAttach,
	
	/// KNEM Cross-Memory-Attach
	CrossMemoryAttach_KNEM,
	
	/// mm POSIX
	SharedMemory_POSIX,
	
	/// mm SysV
	SharedMemory_SysV,
	
	/// mm XPMEM
	SharedMemory_XPMEM,
	
	/// Self
	Self_,
	
	/// TCP
	TCP,
	
	/// Cray userspace Genesis interconnect.
	Cray_uGNI,
}

impl MemoryDomainComponent
{
	#[inline(always)]
	fn memory_domain_configuration(&self) -> Result<MemoryDomainComponentConfiguration, ErrorCode>
	{
		let name = self.name();
		
		// Going forward, the safe way to do this is probably to use CString for environment variable names and values in JSON.
		let environment_variable_prefix = unsafe { CStr::from_ptr(b"\0" as *const u8 as *const c_char) };
		
		MemoryDomainComponentConfiguration::read_from_environment(name, &environment_variable_prefix)
	}
	
	/// UCT name for FFI.
	#[inline(always)]
	pub fn name(&self) -> &'static CStr
	{
		use self::MemoryDomainComponent::*;
		
		match *self
		{
			CudaCopy => unsafe { CStr::from_ptr(b"cuda_cpy\0" as *const u8 as *const c_char) },
			
			CudaGdrCopy => unsafe { CStr::from_ptr(b"gdr_copy\0" as *const u8 as *const c_char) },
			
			InfiniBand => unsafe { CStr::from_ptr(b"ib\0" as *const u8 as *const c_char) },
			
			InfiniBandRdmaConnectionManager => unsafe { CStr::from_ptr(b"rdmacm\0" as *const u8 as *const c_char) },
			
			RadeonOpenCompute => unsafe { CStr::from_ptr(b"rocm\0" as *const u8 as *const c_char) },
			
			CrossMemoryAttach => unsafe { CStr::from_ptr(b"cma\0" as *const u8 as *const c_char) },
			
			CrossMemoryAttach_KNEM => unsafe { CStr::from_ptr(b"knem\0" as *const u8 as *const c_char) },
			
			SharedMemory_POSIX => unsafe { CStr::from_ptr(b"posix\0" as *const u8 as *const c_char) },
			
			SharedMemory_SysV => unsafe { CStr::from_ptr(b"sysv\0" as *const u8 as *const c_char) },
			
			SharedMemory_XPMEM => unsafe { CStr::from_ptr(b"xpmem\0" as *const u8 as *const c_char) },
			
			Self_ => unsafe { CStr::from_ptr(b"self\0" as *const u8 as *const c_char) },
			
			TCP => unsafe { CStr::from_ptr(b"tcp\0" as *const u8 as *const c_char) },
			
			Cray_uGNI => unsafe { CStr::from_ptr(b"ugni\0" as *const u8 as *const c_char) },
		}
	}
	
	/// UCT configuration prefix.
	#[inline(always)]
	pub fn uct_configuration_prefix(&self) -> &'static CStr
	{
		use self::MemoryDomainComponent::*;
		
		match *self
		{
			CudaCopy => unsafe { CStr::from_ptr(b"CUDA_COPY_\0" as *const u8 as *const c_char) },
			
			CudaGdrCopy => unsafe { CStr::from_ptr(b"GDR_COPY_\0" as *const u8 as *const c_char) },
			
			InfiniBand => unsafe { CStr::from_ptr(b"IB_\0" as *const u8 as *const c_char) },
			
			InfiniBandRdmaConnectionManager => unsafe { CStr::from_ptr(b"RDMACM_\0" as *const u8 as *const c_char) },
			
			RadeonOpenCompute => unsafe { CStr::from_ptr(b"ROCM_MD_\0" as *const u8 as *const c_char) },
			
			CrossMemoryAttach => unsafe { CStr::from_ptr(b"CMA_\0" as *const u8 as *const c_char) },
			
			CrossMemoryAttach_KNEM => unsafe { CStr::from_ptr(b"KNEM_\0" as *const u8 as *const c_char) },
			
			SharedMemory_POSIX => unsafe { CStr::from_ptr(b"POSIX_\0" as *const u8 as *const c_char) },
			
			SharedMemory_SysV => unsafe { CStr::from_ptr(b"SYSV_\0" as *const u8 as *const c_char) },
			
			SharedMemory_XPMEM => unsafe { CStr::from_ptr(b"XPMEM_\0" as *const u8 as *const c_char) },
			
			Self_ => unsafe { CStr::from_ptr(b"SELF_\0" as *const u8 as *const c_char) },
			
			TCP => unsafe { CStr::from_ptr(b"TCP_\0" as *const u8 as *const c_char) },
			
			Cray_uGNI => unsafe { CStr::from_ptr(b"UGNI_\0" as *const u8 as *const c_char) },
		}
	}
}
