// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// How to open an interface.
#[derive(Debug)]
pub enum CommunicationInterfaceContextEndPointAddress<SCR=DoNothingServerConnectionRequest>
where SCR: ServerConnectionRequest
{
	/// Open a local device `device_name` belonging to transport layer.
	Device
	{
		/// Local device name on a transport layer.
		device_name: CString,
	},
	
	/// Open a client which will connect to a server listening socket.
	ClientSocket,
	
	/// Open a listening server on `socket_address` and get called back with new connections on `connection_callback`.
	ServerSocket
	{
		/// Socket address to listen locally on.
		socket_address: SocketAddress,
		
		/// Callback to process new (inbound) connections that connect to the server's listening socket.
		connection_callback: SCR,
	},
}

impl<SCR: ServerConnectionRequest> CommunicationInterfaceContextEndPointAddress<SCR>
{
	#[inline(always)]
	pub(crate) fn communication_interface_configuration(&self, memory_domain: &MemoryDomain) -> Result<CommunicationInterfaceConfiguration, ErrorCode>
	{
		let transport_layer_name = self.transport_layer_name_for_configuration(memory_domain.transport_layer());
		
		// Going forward, the safe way to do this is probably to use CString for environment variable names and values in JSON.
		let environment_variable_prefix = unsafe { CStr::from_ptr(b"\0" as *const u8 as *const c_char) };
		
		CommunicationInterfaceConfiguration::read_from_environment(transport_layer_name, &environment_variable_prefix, memory_domain)
	}
	
	// `transport_layer_name` can be null for a client or server socket.
	#[inline(always)]
	fn transport_layer_name_for_configuration(&self, transport_layer: &MemoryDomainComponentAndTransportLayer) -> *const c_char
	{
		use self::CommunicationInterfaceContextEndPointAddress::*;
		
		match *self
		{
			Device { .. } => transport_layer.transport_layer_name().as_ptr(),
			
			_ => null(),
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn open_mode(&self, transport_layer: &MemoryDomainComponentAndTransportLayer) -> (uct_iface_open_mode, uct_iface_params__bindgen_ty_1)
	{
		use self::CommunicationInterfaceContextEndPointAddress::*;
		
		let mut open: uct_iface_params__bindgen_ty_1 = unsafe { uninitialized() };
		
		match *self
		{
			Device { ref device_name } =>
			{
				{
					let mut device = unsafe { open.device.as_mut() };
					device.tl_name = transport_layer.transport_layer_name().as_ptr();
					device.dev_name = device_name.as_ptr();
				}
				(uct_iface_open_mode::DEVICE, open)
			}
			
			ClientSocket { .. } => (uct_iface_open_mode::SOCKADDR_CLIENT, open),
			
			ServerSocket { ref socket_address, ref connection_callback } =>
			{
				let (addr, addrlen) = socket_address.suitable_for_ffi();
				{
					let mut socket_address = unsafe { open.sockaddr.as_mut() };
					socket_address.listen_sockaddr = ucs_sock_addr
					{
						addr,
						addrlen,
					};
					socket_address.conn_request_cb = Self::connection_request_callback;
					socket_address.conn_request_arg = connection_callback as *const _ as *mut _;
					socket_address.cb_flags = connection_callback.connection_callback_flags().0;
				}
				(uct_iface_open_mode::SOCKADDR_SERVER, open)
			}
		}
	}
	
	#[inline(always)]
	unsafe extern "C" fn connection_request_callback(arg: *mut c_void, conn_priv_data: *const c_void, length: usize) -> ucs_status_t
	{
		debug_assert!(!arg.is_null(), "arg is null");
		debug_assert!(!conn_priv_data.is_null(), "conn_priv_data is null");
		
		let this = & * (arg as *const c_void as *const SCR);
		let connection_private_data_from_uct_ep_create_sockaddr = UcxAllocatedByteBuffer::new(conn_priv_data as *mut _, length);
		match this.connection_request(connection_private_data_from_uct_ep_create_sockaddr)
		{
			true => ucs_status_t::UCS_OK,
			false => ucs_status_t::UCS_ERR_NO_RESOURCE,
		}
	}
}
