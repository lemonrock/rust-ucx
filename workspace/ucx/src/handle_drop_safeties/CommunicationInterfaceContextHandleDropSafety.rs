// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct CommunicationInterfaceContextHandleDropSafety(NonNull<uct_iface>, Arc<MemoryDomainHandleDropSafety>);

impl Drop for CommunicationInterfaceContextHandleDropSafety
{
	/// Close and destroy an interface.
	///
	/// Equivalent to `uct_iface_close`.
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { (self.transport_interface_operations().iface_close)(self.0.as_ptr()) }
	}
}

impl CommunicationInterfaceContextHandleDropSafety
{
	#[inline(always)]
	pub(crate) fn new(value: NonNull<uct_iface>, memory_domain_handle_drop_safety: Arc<MemoryDomainHandleDropSafety>) -> Arc<Self>
	{
		Arc::new(CommunicationInterfaceContextHandleDropSafety(value, memory_domain_handle_drop_safety))
	}
	
	#[inline(always)]
	fn transport_interface_operations(&self) -> &mut uct_iface_ops
	{
		&mut unsafe { &mut * self.0.as_ptr() }.ops
	}
}
