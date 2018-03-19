// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Attributes of a `CommunicationInterfaceContext`.
#[derive(Debug)]
pub struct CommunicationInterfaceContextAttributes(pub(crate) uct_iface_attr);

impl CommunicationInterfaceContextAttributes
{
	/// Device address length.
	#[inline(always)]
	pub fn device_address_length(&self) -> usize
	{
		self.0.device_addr_len
	}
	
	/// Interface address length.
	#[inline(always)]
	pub fn interface_address_length(&self) -> usize
	{
		self.0.iface_addr_len
	}
	
	/// Get constraints.
	#[inline(always)]
	pub fn put_constraints(&self) -> &PutConstraints
	{
		&self.capabilities().put
	}
	
	/// Get constraints.
	#[inline(always)]
	pub fn get_constraints(&self) -> &GetConstraints
	{
		&self.capabilities().get
	}
	
	/// Active message constraints.
	#[inline(always)]
	pub fn active_message_constraints(&self) -> &ActiveMessageConstraints
	{
		&self.capabilities().am
	}
	
	/// Tagged message constraints.
	#[inline(always)]
	pub fn tagged_message_constraints(&self) -> &TaggedMessageConstraints
	{
		&self.capabilities().tag
	}
	
	/// Does the interface support the `required_to_support` features?
	#[inline(always)]
	pub fn supports_all_of(&self, required_to_support: InterfaceFeaturesSupported) -> bool
	{
		self.interface_features_supported().contains(required_to_support)
	}
	
	/// What features does the interface support?
	#[inline(always)]
	pub fn interface_features_supported(&self) -> InterfaceFeaturesSupported
	{
		InterfaceFeaturesSupported::from_bits_truncate(self.capabilities().flags)
	}
	
	#[inline(always)]
	fn capabilities(&self) -> &uct_iface_attr__bindgen_ty_1
	{
		&self.0.cap
	}
	
	#[inline(always)]
	pub(crate) fn query(iface: NonNull<uct_iface>) -> Self
	{
		let mut attributes = unsafe { uninitialized() };
		
		let transport_interface_operations = &mut unsafe { &mut * iface.as_ptr() }.ops;
		
		let status = unsafe { (transport_interface_operations.iface_query)(iface.as_ptr(), &mut attributes) };
		
		if status.is_ok()
		{
			CommunicationInterfaceContextAttributes(attributes)
		}
		else
		{
			panic!("Unexpected status '{:?}'", status.parse());
		}
	}
}
