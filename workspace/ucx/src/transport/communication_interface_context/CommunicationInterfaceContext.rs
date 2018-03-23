// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


macro_rules! set_active_message_handler
{
	($_count: ident, $a_count: ident, $count: expr)  =>
	{
		interpolate_idents!
		{
			/// Set an active message handler for active message identifier [$count].
			#[inline(always)]
        	pub fn [set_active_message_handler $_count](&mut self, active_message_handler: [$a_count], flags: uct_cb_flags) -> Result<(), ErrorCode>
        	{
				let former_active_message_handler = self.[active_message_handler $_count].take();
				self.[active_message_handler $_count] = Some(active_message_handler);
				let callback_data = self.[active_message_handler $_count].as_mut().unwrap() as *mut _;
				let result = self.set_active_message_handler_for_active_messages_of_identifier(ActiveMessageIdentifier($count), Self::[callback_on_active_message_receive $_count], callback_data, flags);
				drop(former_active_message_handler);
				result
        	}
        	
        	#[inline(always)]
        	unsafe extern "C" fn [callback_on_active_message_receive $_count](arg: *mut c_void, data: *mut c_void, length: usize, flags: c_uint) -> ucs_status_t
        	{
        		debug_assert!(!arg.is_null(), "arg is null");
        		let raw_handler = NonNull::new_unchecked(arg as *mut $a_count);
        		let handler = raw_handler.as_ref();
        		
        		debug_assert!(!data.is_null(), "data is null");
        		let buffer = UcxAllocatedByteBuffer::new(data, length);
        		
        		match handler.invoke(buffer, uct_cb_param_flags(flags))
        		{
        			true => ucs_status_t::UCS_OK,
        			false => ucs_status_t::UCS_INPROGRESS,
        		}
        	}
    	}
	}
}

/// A communication interface context.
///
/// Simply, an interface; a means of talking to remote end points and receiving data from them.
#[derive(Debug)]
pub struct CommunicationInterfaceContext<SCR=DoNothingServerConnectionRequest, E=DoNothingErrorHandler, UETM=DoNothingUnexpectedTaggedMessageHandler, AT=DoNothingActiveMessageTracer, A0=DoNothingActiveMessageHandler, A1=DoNothingActiveMessageHandler, A2=DoNothingActiveMessageHandler, A3=DoNothingActiveMessageHandler, A4=DoNothingActiveMessageHandler, A5=DoNothingActiveMessageHandler, A6=DoNothingActiveMessageHandler, A7=DoNothingActiveMessageHandler, A8=DoNothingActiveMessageHandler, A9=DoNothingActiveMessageHandler, A10=DoNothingActiveMessageHandler, A11=DoNothingActiveMessageHandler, A12=DoNothingActiveMessageHandler, A13=DoNothingActiveMessageHandler, A14=DoNothingActiveMessageHandler, A15=DoNothingActiveMessageHandler, A16=DoNothingActiveMessageHandler, A17=DoNothingActiveMessageHandler, A18=DoNothingActiveMessageHandler, A19=DoNothingActiveMessageHandler, A20=DoNothingActiveMessageHandler, A21=DoNothingActiveMessageHandler, A22=DoNothingActiveMessageHandler, A23=DoNothingActiveMessageHandler, A24=DoNothingActiveMessageHandler, A25=DoNothingActiveMessageHandler, A26=DoNothingActiveMessageHandler, A27=DoNothingActiveMessageHandler, A28=DoNothingActiveMessageHandler, A29=DoNothingActiveMessageHandler, A30=DoNothingActiveMessageHandler, A31=DoNothingActiveMessageHandler>
where
	SCR: ServerConnectionRequest,
	E: ErrorHandler,
	UETM: UnexpectedTaggedMessageHandler,
	AT: ActiveMessageTracer,
	A0: ActiveMessageHandler,
	A1: ActiveMessageHandler,
	A2: ActiveMessageHandler,
	A3: ActiveMessageHandler,
	A4: ActiveMessageHandler,
	A5: ActiveMessageHandler,
	A6: ActiveMessageHandler,
	A7: ActiveMessageHandler,
	A8: ActiveMessageHandler,
	A9: ActiveMessageHandler,
	A10: ActiveMessageHandler,
	A11: ActiveMessageHandler,
	A12: ActiveMessageHandler,
	A13: ActiveMessageHandler,
	A14: ActiveMessageHandler,
	A15: ActiveMessageHandler,
	A16: ActiveMessageHandler,
	A17: ActiveMessageHandler,
	A18: ActiveMessageHandler,
	A19: ActiveMessageHandler,
	A20: ActiveMessageHandler,
	A21: ActiveMessageHandler,
	A22: ActiveMessageHandler,
	A23: ActiveMessageHandler,
	A24: ActiveMessageHandler,
	A25: ActiveMessageHandler,
	A26: ActiveMessageHandler,
	A27: ActiveMessageHandler,
	A28: ActiveMessageHandler,
	A29: ActiveMessageHandler,
	A30: ActiveMessageHandler,
	A31: ActiveMessageHandler
{
	handle: NonNull<uct_iface>, // Only dangling during construction
	handle_drop_safety: Option<Arc<CommunicationInterfaceContextHandleDropSafety>>, // Only None during construction.
	attributes: CommunicationInterfaceContextAttributes, // Invalid during construction.
	progress_engine: ProgressEngine,
	end_point_address: CommunicationInterfaceContextEndPointAddress<SCR>,
	error_handler: E,
	unexpected_tagged_message_handler: UETM,
	active_message_tracer: Option<AT>,
	active_message_handler_0: Option<A0>,
	active_message_handler_1: Option<A1>,
	active_message_handler_2: Option<A2>,
	active_message_handler_3: Option<A3>,
	active_message_handler_4: Option<A4>,
	active_message_handler_5: Option<A5>,
	active_message_handler_6: Option<A6>,
	active_message_handler_7: Option<A7>,
	active_message_handler_8: Option<A8>,
	active_message_handler_9: Option<A9>,
	active_message_handler_10: Option<A10>,
	active_message_handler_11: Option<A11>,
	active_message_handler_12: Option<A12>,
	active_message_handler_13: Option<A13>,
	active_message_handler_14: Option<A14>,
	active_message_handler_15: Option<A15>,
	active_message_handler_16: Option<A16>,
	active_message_handler_17: Option<A17>,
	active_message_handler_18: Option<A18>,
	active_message_handler_19: Option<A19>,
	active_message_handler_20: Option<A20>,
	active_message_handler_21: Option<A21>,
	active_message_handler_22: Option<A22>,
	active_message_handler_23: Option<A23>,
	active_message_handler_24: Option<A24>,
	active_message_handler_25: Option<A25>,
	active_message_handler_26: Option<A26>,
	active_message_handler_27: Option<A27>,
	active_message_handler_28: Option<A28>,
	active_message_handler_29: Option<A29>,
	active_message_handler_30: Option<A30>,
	active_message_handler_31: Option<A31>,
}

impl<SCR: ServerConnectionRequest, E : ErrorHandler, UETM: UnexpectedTaggedMessageHandler, AT: ActiveMessageTracer, A0: ActiveMessageHandler, A1: ActiveMessageHandler, A2: ActiveMessageHandler, A3: ActiveMessageHandler, A4: ActiveMessageHandler, A5: ActiveMessageHandler, A6: ActiveMessageHandler, A7: ActiveMessageHandler, A8: ActiveMessageHandler, A9: ActiveMessageHandler, A10: ActiveMessageHandler, A11: ActiveMessageHandler, A12: ActiveMessageHandler, A13: ActiveMessageHandler, A14: ActiveMessageHandler, A15: ActiveMessageHandler, A16: ActiveMessageHandler, A17: ActiveMessageHandler, A18: ActiveMessageHandler, A19: ActiveMessageHandler, A20: ActiveMessageHandler, A21: ActiveMessageHandler, A22: ActiveMessageHandler, A23: ActiveMessageHandler, A24: ActiveMessageHandler, A25: ActiveMessageHandler, A26: ActiveMessageHandler, A27: ActiveMessageHandler, A28: ActiveMessageHandler, A29: ActiveMessageHandler, A30: ActiveMessageHandler, A31: ActiveMessageHandler> HasAttributes for CommunicationInterfaceContext<SCR, E, UETM, AT, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20, A21, A22, A23, A24, A25, A26, A27, A28, A29, A30, A31>
{
	type Attributes = CommunicationInterfaceContextAttributes;
	
	#[inline(always)]
	fn attributes(&self) -> &Self::Attributes
	{
		&self.attributes
	}
}

impl<SCR: ServerConnectionRequest, E: ErrorHandler, UETM: UnexpectedTaggedMessageHandler, AT: ActiveMessageTracer, A0: ActiveMessageHandler, A1: ActiveMessageHandler, A2: ActiveMessageHandler, A3: ActiveMessageHandler, A4: ActiveMessageHandler, A5: ActiveMessageHandler, A6: ActiveMessageHandler, A7: ActiveMessageHandler, A8: ActiveMessageHandler, A9: ActiveMessageHandler, A10: ActiveMessageHandler, A11: ActiveMessageHandler, A12: ActiveMessageHandler, A13: ActiveMessageHandler, A14: ActiveMessageHandler, A15: ActiveMessageHandler, A16: ActiveMessageHandler, A17: ActiveMessageHandler, A18: ActiveMessageHandler, A19: ActiveMessageHandler, A20: ActiveMessageHandler, A21: ActiveMessageHandler, A22: ActiveMessageHandler, A23: ActiveMessageHandler, A24: ActiveMessageHandler, A25: ActiveMessageHandler, A26: ActiveMessageHandler, A27: ActiveMessageHandler, A28: ActiveMessageHandler, A29: ActiveMessageHandler, A30: ActiveMessageHandler, A31: ActiveMessageHandler> CommunicationInterfaceContext<SCR, E, UETM, AT, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20, A21, A22, A23, A24, A25, A26, A27, A28, A29, A30, A31>
{
	/// Open an interface.
	///
	/// Equivalent to `uct_iface_open`.
	#[inline(always)]
	pub fn open(hyper_thread_index: ZeroBasedHyperThreadIndex, memory_domain: &MemoryDomain, end_point_address: CommunicationInterfaceContextEndPointAddress<SCR>, error_handler: E, unexpected_tagged_message_handler: UETM, progress_engine: &ProgressEngine) -> Result<Box<Self>, ErrorCode>
	{
		let mut this = Box::new
		(
			Self
			{
				handle: NonNull::dangling(),
				handle_drop_safety: None,
				attributes: unsafe { uninitialized() },
				progress_engine: progress_engine.clone(),
				end_point_address,
				error_handler,
				unexpected_tagged_message_handler,
				active_message_tracer: None,
				active_message_handler_0: None,
				active_message_handler_1: None,
				active_message_handler_2: None,
				active_message_handler_3: None,
				active_message_handler_4: None,
				active_message_handler_5: None,
				active_message_handler_6: None,
				active_message_handler_7: None,
				active_message_handler_8: None,
				active_message_handler_9: None,
				active_message_handler_10: None,
				active_message_handler_11: None,
				active_message_handler_12: None,
				active_message_handler_13: None,
				active_message_handler_14: None,
				active_message_handler_15: None,
				active_message_handler_16: None,
				active_message_handler_17: None,
				active_message_handler_18: None,
				active_message_handler_19: None,
				active_message_handler_20: None,
				active_message_handler_21: None,
				active_message_handler_22: None,
				active_message_handler_23: None,
				active_message_handler_24: None,
				active_message_handler_25: None,
				active_message_handler_26: None,
				active_message_handler_27: None,
				active_message_handler_28: None,
				active_message_handler_29: None,
				active_message_handler_30: None,
				active_message_handler_31: None,
			}
		);
		
		let communication_interface_configuration = this.end_point_address.communication_interface_configuration(memory_domain)?;
		let (uct_iface_open_mode(open_mode), mode) = this.end_point_address.open_mode(memory_domain.transport_layer());
		
		let raw_unexpected_tagged_message_handler = (&this.unexpected_tagged_message_handler) as *const UETM as *mut UETM as *mut c_void;
		let parameters = uct_iface_params
		{
			cpu_mask: ucs_cpu_set_t::create_for_hyper_thread(hyper_thread_index),
			
			open_mode: open_mode as u64,
			mode,
			
			stats_root: null_mut(),
			
			rx_headroom: this.unexpected_tagged_message_handler.receive_headroom(),
			
			err_handler_arg: this.deref() as *const Self as *const c_void as *mut _,
			err_handler: Self::error_handler,
			
			eager_arg: raw_unexpected_tagged_message_handler,
			eager_cb: Self::unexpected_eager_tagged_message,
			
			rndv_arg: raw_unexpected_tagged_message_handler,
			rndv_cb: Self::unexpected_rendezvous_tagged_message,
		};
		
		let mut handle = unsafe { uninitialized() };
		let status = unsafe { uct_iface_open(memory_domain.as_ptr(), progress_engine.as_ptr(), &parameters, communication_interface_configuration.as_ptr(), &mut handle) };
		
		if let Err(error_code) = Self::parse_status(status, ())
		{
			Err(error_code)
		}
		else
		{
			let handle = unsafe { NonNull::new_unchecked(handle) };
			this.handle = handle;
			this.handle_drop_safety = Some(CommunicationInterfaceContextHandleDropSafety::new(handle, memory_domain.handle_drop_safety()));
			this.attributes = CommunicationInterfaceContextAttributes::query(handle);
			Ok(this)
		}
	}
	
	unsafe extern "C" fn error_handler(arg: *mut c_void, ep: *mut uct_ep, status: ucs_status_t) -> ucs_status_t
	{
		debug_assert!(!arg.is_null(), "arg is null");
		debug_assert!(!ep.is_null(), "ep is null");
		
		use self::Status::*;
		let status = status.parse();
		if let Error(error_code) = status
		{
			let this = & * (arg as *mut Self as *const Self);
			match this.error_handler.handle(NonNull::new_unchecked(ep), error_code)
			{
				Ok(()) => ucs_status_t::UCS_OK,
				Err(error_code) => error_code.to_ucs_status_t(),
			}
		}
		else
		{
			panic!("Invalid status '{:?}'", status)
		}
	}
	
	/// This callback is invoked when a tagged message sent by the eager protocol has arrived and no corresponding tag has been posted.
	///
	/// The callback is always invoked from the context (thread, process) that called `progress()`.
	///
	/// It is allowed to call other communication routines from the callback.
	unsafe extern "C" fn unexpected_eager_tagged_message(arg: *mut c_void, data: *mut c_void, length: usize, flags: c_uint, stag: uct_tag_t, imm: u64) -> ucs_status_t
	{
		debug_assert!(!arg.is_null(), "arg is null");
		debug_assert!(!data.is_null(), "data is null");
		
		let this = & * (arg as *mut UETM as *const UETM);
		
		let sender_tag = TagValue(stag);
		let tagged_message_data = UcxAllocatedByteBuffer::new(data, length);
		let immediate_data = imm;
		
		let callback_flags = uct_cb_param_flags(flags);
		let has_descriptor_with_user_defined_receive_headroom = callback_flags & uct_cb_param_flags::DESC == uct_cb_param_flags::DESC;
		if has_descriptor_with_user_defined_receive_headroom
		{
			match this.unexpected_eager_tagged_message_with_descriptor_with_user_defined_receive_headroom(sender_tag, tagged_message_data, immediate_data)
			{
				true => ucs_status_t::UCS_OK,
				false => ucs_status_t::UCS_INPROGRESS,
			}
		}
		else
		{
			this.unexpected_eager_tagged_message(sender_tag, tagged_message_data, immediate_data);
			ucs_status_t::UCS_OK
		}
	}
	
	/// This callback is invoked when a tagged message send notifications by the rendezvous protocol has arrived and no corresponding tag has been posted.
	unsafe extern "C" fn unexpected_rendezvous_tagged_message(arg: *mut c_void, flags: c_uint, stag: u64, header: *const c_void, header_length: c_uint, remote_addr: u64, length: usize, rkey_buf: *const c_void) -> ucs_status_t
	{
		debug_assert!(!arg.is_null(), "arg is null");
		debug_assert!(!header.is_null(), "arg is null");
		debug_assert!(!rkey_buf.is_null(), "rkey_buf is null");
		
		let this = & * (arg as *mut UETM as *const UETM);
		
		let sender_tag = TagValue(stag);
		let header = UcxAllocatedByteBuffer::new(header as *mut _, header_length as usize);
		let remote_memory_address = RemoteMemoryAddress(remote_addr);
		let remote_length = length;
		let remote_key = NonNull::new_unchecked(rkey_buf as *mut u8);
		
		let callback_flags = uct_cb_param_flags(flags);
		let has_descriptor_with_user_defined_receive_headroom = callback_flags & uct_cb_param_flags::DESC == uct_cb_param_flags::DESC;
		if has_descriptor_with_user_defined_receive_headroom
		{
			match this.unexpected_rendezvous_tagged_message_with_descriptor_with_user_defined_receive_headroom(sender_tag, header, remote_memory_address, remote_length, remote_key)
			{
				true => ucs_status_t::UCS_OK,
				false => ucs_status_t::UCS_INPROGRESS,
			}
		}
		else
		{
			this.unexpected_rendezvous_tagged_message(sender_tag, header, remote_memory_address, remote_length, remote_key);
			ucs_status_t::UCS_OK
		}
	}
	
	/// Explicit progress.
	///
	/// Equivalent to `ProgressEngine.progress()` and `uct_worker_progress`.
	///
	/// Not thread safe.
	///
	/// This routine explicitly progresses any outstanding communication operations and active message requests.
	///
	/// Returns `true` if communication progressed, `false` otherwise.
	#[inline(always)]
	pub fn progress_thread_unsafe(&self) -> bool
	{
		self.progress_engine.progress_thread_unsafe()
	}
	
	/// Add a progress callback function.
	///
	/// Equivalent to `ProgressEngine.progress_register_thread_safe()` and `uct_worker_progress_register_safe`.
	///
	/// The `CallbackQueueIdentifier` can be used to remove this callback if necessary.
	#[inline(always)]
	pub fn progress_register_thread_safe<'progress_engine, PC: ProgressCallback>(&'progress_engine self, progress_callback: Box<PC>) -> ProgressCallbackCancel<'progress_engine, PC>
	{
		self.progress_engine.progress_register_thread_safe(progress_callback)
	}
	
	/// Set an active message tracer.
	#[inline(always)]
	pub fn set_active_message_tracer(&mut self, active_message_tracer: AT) -> Result<(), ErrorCode>
	{
		let former_active_message_tracer = self.active_message_tracer.take();
		self.active_message_tracer = Some(active_message_tracer);
		let callback_data = self.active_message_tracer.as_mut().unwrap() as *mut _;
		let result = self.set_active_message_tracer_ffi(Some(Self::callback_on_active_message_trace), callback_data);
		drop(former_active_message_tracer);
		result
	}
	
	#[inline(always)]
	unsafe extern "C" fn callback_on_active_message_trace(arg: *mut c_void, type_: uct_am_trace_type_t, id: u8, data: *const c_void, length: usize, buffer: *mut c_char, max: usize)
	{
		debug_assert!(!arg.is_null(), "arg is null");
		let raw_handler = NonNull::new_unchecked(arg as *mut AT);
		let handler = raw_handler.as_ref();
		
		debug_assert!(!data.is_null(), "data is null");
		let read_only = UcxAllocatedByteBuffer::new(data as *mut c_void, length);
		
		handler.trace(type_, ActiveMessageIdentifier(id), read_only, from_raw_parts_mut(buffer, max))
	}
	
	set_active_message_handler!(_0, A0, 0);
	set_active_message_handler!(_1, A1, 1);
	set_active_message_handler!(_2, A2, 2);
	set_active_message_handler!(_3, A3, 3);
	set_active_message_handler!(_4, A4, 4);
	set_active_message_handler!(_5, A5, 5);
	set_active_message_handler!(_6, A6, 6);
	set_active_message_handler!(_7, A7, 7);
	set_active_message_handler!(_8, A8, 8);
	set_active_message_handler!(_9, A9, 9);
	set_active_message_handler!(_10, A10, 10);
	set_active_message_handler!(_11, A11, 11);
	set_active_message_handler!(_12, A12, 12);
	set_active_message_handler!(_13, A13, 13);
	set_active_message_handler!(_14, A14, 14);
	set_active_message_handler!(_15, A15, 15);
	set_active_message_handler!(_16, A16, 16);
	set_active_message_handler!(_17, A17, 17);
	set_active_message_handler!(_18, A18, 18);
	set_active_message_handler!(_19, A19, 19);
	set_active_message_handler!(_20, A20, 20);
	set_active_message_handler!(_21, A21, 21);
	set_active_message_handler!(_22, A22, 22);
	set_active_message_handler!(_23, A23, 23);
	set_active_message_handler!(_24, A24, 24);
	set_active_message_handler!(_25, A25, 25);
	set_active_message_handler!(_26, A26, 26);
	set_active_message_handler!(_27, A27, 27);
	set_active_message_handler!(_28, A28, 28);
	set_active_message_handler!(_29, A29, 29);
	set_active_message_handler!(_30, A30, 30);
	set_active_message_handler!(_31, A31, 31);
	
	/// Can active messages be received as duplicates?
	///
	/// Equivalent to `self.attributes().supports_all_of(InterfaceFeaturesSupported::AM_DUP)`.
	///
	/// Only true for the `CM` transport as of the time of writing.
	#[inline(always)]
	pub fn can_active_messages_be_received_as_duplicates(&self) -> bool
	{
		self.interface_supports_feature(InterfaceFeaturesSupported::AM_DUP)
	}
	
	/// Post a tag to a transport interface.
	///
	/// Equivalent to `uct_iface_tag_recv_zcopy`.
	///
	/// This routine posts a tag to be matched on a transport interface.
	/// When a message with the corresponding tag arrives it is stored in the user buffer (described by `pointer_to_an_array_of_io_vec_structures` and `array_of_io_vec_structures_length`) directly.
	/// The operation completion is reported using callbacks on the `ctx` structure.
	///
	/// * `tag`: Tag to expect.
	/// * `tag_mask`: Mask which specifies what bits of the tag to compare.
	/// * `io_vec`: May be empty. Maximum length is uct_iface_attr.max_iov`.
	/// * `ctx`: Context associated with this particular tag. The `priv_` field in this structure is used to track the state internally.
	///
	/// Returns:-
	///
	/// * `UCS_OK`: The tag is posted to the transport.
	/// * `UCS_ERR_NO_RESOURCE`: Could not start the operation due to lack of resources.
	/// * `UCS_ERR_EXCEEDS_LIMIT`: No more room for tags in the transport.
	#[inline(always)]
	pub fn tagged_message_receive_zero_copy(&self, tag_matcher: TagMatcher, io_vec: &[uct_iov_t], ctx: &mut uct_tag_context) -> Result<(), ErrorCode>
	{
		debug_assert!(self.interface_supports_feature(InterfaceFeaturesSupported::TAG_EAGER_ZCOPY), "Tag matching not supported");
		
		let tag = tag_matcher.value.0;
		let tag_mask = tag_matcher.bit_mask.0;
		
		let status = unsafe { (self.transport_interface_operations().iface_tag_recv_zcopy)(self.as_ptr(), tag, tag_mask, io_vec.as_ptr(), io_vec.len(), ctx) };
		
		Self::parse_status(status, ())
	}
	
	/// Cancel a posted tag.
	///
	/// Equivalent to `uct_iface_tag_recv_cancel`.
	///
	/// This routine cancels tag: a, which was previously posted by `tagged_message_receive_zcopy`.
	/// The tag would be either matched canceled: or, in a time: bounded, regardless of the peer actions.
	/// The original completion callback of the tag would be called with the status if force is not set.
	///
	/// The tag would be either matched or canceled in a bounded time, regardless of the peer actions.
	/// The original completion callback of the tag would be called with the status if `force` is not set (`false`).
	///
	/// * `ctx`: Tag context which was used for posting the tag. If `force` is `false`, `ctx->completed_cb` will be called with either `UCS_OK` which means the tag was matched and data received despite the cancel request, or `UCS_ERR_CANCELED` which means the tag was successfully cancelled before it was matched.
	/// * `force`: Whether to report completions to `ctx.completed_cb`.
	///
	/// Returns:-
	/// * `UCS_OK`: If the tag is cancelled in the transport.
	#[inline(always)]
	pub fn tagged_message_receive_cancel(&self, ctx: &mut uct_tag_context, force: bool) -> Result<(), ErrorCode>
	{
		debug_assert!(self.interface_supports_feature(InterfaceFeaturesSupported::TAG_EAGER_ZCOPY), "Tag matching not supported");
		
		let status = unsafe { (self.transport_interface_operations().iface_tag_recv_cancel)(self.as_ptr(), ctx, force.to_c_bool()) };
		
		Self::parse_status(status, ())
	}
	
	/// Is interface reachable?
	///
	/// Equivalent to `uct_iface_is_reachable`.
	///
	/// This function checks if a remote address can be reached from a local interface.
	/// If the function returns true, it does not necessarily mean a connection or data transfer would succeed, since the 'reachable check' is a local operation; it does not detect issues such as network mis-configuration or lack of connectivity.
	///
	/// * `device_address` Device address to check if is reachable to. It is NULL if `iface_attr.dev_addr_len == 0`, and must be non-NULL otherwise.
	/// * `interface_address`  Interface address to check if is reachable to. It is NULL if `iface_attr.iface_addr_len == 0`, and must be non-NULL otherwise.
	#[inline(always)]
	pub fn is_reachable_check(&self, device_address: &DeviceAddress, interface_address: &InterfaceAddress) -> bool
	{
		debug_assert_eq!(self.attributes().device_address_length(), device_address.length(), "device address length mismatch");
		debug_assert_eq!(self.attributes().interface_address_length(), interface_address.length(), "interface address length mismatch");
		
		let result = unsafe { (self.transport_interface_operations().iface_is_reachable)(self.as_ptr(), device_address.is_reachable_address(), interface_address.is_reachable_address()) };
		result.from_c_bool()
	}
	
	/// Get address of the interface.
	///
	/// Equivalent to `uct_iface_get_address`.
	#[inline(always)]
	pub fn get_interface_address(&self) -> Result<InterfaceAddress, ErrorCode>
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::CONNECT_TO_IFACE);
		
		let interface_address = InterfaceAddress::new(self.attributes().interface_address_length());
		
		let status = unsafe { (self.transport_interface_operations().iface_get_address)(self.as_ptr(), interface_address.address().as_ptr() as *mut _) };
		
		Self::parse_status(status, interface_address)
	}
	
	/// Get underlying address of the device the interface is using.
	///
	/// Equivalent to `uct_iface_get_device_address`.
	///
	/// All interfaces using the same device would return the same address.
	#[inline(always)]
	pub fn get_device_address(&self) -> Result<DeviceAddress, ErrorCode>
	{
		let device_address = DeviceAddress::new(self.attributes().device_address_length());
		
		let status = unsafe { (self.transport_interface_operations().iface_get_device_address)(self.as_ptr(), device_address.address().as_ptr() as *mut _) };
		
		Self::parse_status(status, device_address)
	}
	
	/// Get an event file descriptor for polling with, say, `epoll`.
	///
	/// Equivalent to `uct_iface_event_fd_get`.
	///
	// Only interfaces supporting the flag `UCT_IFACE_FLAG_EVENT_FD` implement this function.
	#[inline(always)]
	pub fn get_events_file_descriptor(&self) -> Result<RawFd, ErrorCode>
	{
		self.debug_assert_interface_supports_events();
		
		let mut events_file_descriptor = unsafe { uninitialized() };
		
		let status = unsafe { (self.transport_interface_operations().iface_event_fd_get)(self.as_ptr(), &mut events_file_descriptor) };
		
		Self::parse_status(status, events_file_descriptor)
	}
	
	/// Turn on event notification for the next event.
	///
	/// Equivalent to `uct_iface_event_arm`.
	///
	/// This routine needs to be called before waiting on each notification on this interface, so will typically be called once the processing of the previous event is over.
	///
	/// Used in conjunction with `get_events_file_descriptor()`.
	///
	/// Returns:-
	/// * `UCS_OK`: The operation completed successfully. File descriptor (`get_events_file_descriptor()`) will be signaled by new events.
	/// * `UCS_ERR_BUSY`: There are unprocessed events which prevent the file descriptor (`get_events_file_descriptor()`) from being armed. The operation is not completed. File descriptor will not be signaled by new events.
	/// * Other: Genuine failure.
	///
	/// The events supported are:-
	/// * `SEND_COMP`: Send completion event.
	/// * `RECV`: Tag or active message received.
	/// * `RECV_SIG`: Signaled\* tag or active message received (only valid if interface supports Signalled Receives, eg `InterfaceFeaturesSupported::EVENT_RECV_SIG` is present in object returned from `attributes()`).
	///
	/// \* Message was sent with `uct_msg_flags::SIGNALED`.
	#[inline(always)]
	pub fn arm_events_file_descriptor(&self, event_types: uct_iface_event_types) -> Result<(), ErrorCode>
	{
		self.debug_assert_interface_supports_events();
		
		fn is_supported(event_types: uct_iface_event_types, supported_event_type: uct_iface_event_types, interface_feature: InterfaceFeaturesSupported, attributes: &CommunicationInterfaceContextAttributes) -> bool
		{
			if event_types & supported_event_type == supported_event_type
			{
				attributes.supports_all_of(interface_feature)
			}
			else
			{
				true
			}
		}
		debug_assert!(is_supported(event_types, uct_iface_event_types::SEND_COMP, InterfaceFeaturesSupported::EVENT_SEND_COMP, &self.attributes()), "SEND_COMP is not supported");
		debug_assert!(is_supported(event_types, uct_iface_event_types::RECV, InterfaceFeaturesSupported::EVENT_RECV, &self.attributes()), "RECV is not supported");
		debug_assert!(is_supported(event_types, uct_iface_event_types::RECV_SIG, InterfaceFeaturesSupported::EVENT_RECV_SIG, &self.attributes()), "RECV_SIG is not supported");
		
		let status = unsafe { (self.transport_interface_operations().iface_event_arm)(self.as_ptr(), event_types.0) };
		
		Self::parse_status(status, ())
	}
	
	/// Enable synchronous progress for the interface.
	///
	/// Equivalent to `uct_iface_progress_enable`.
	///
	/// Notify the transport that it should actively progress communications during `Worker.progress()`.
	///
	/// NOTE: This function is not thread safe with respect to `Worker.progress()` unless the flag `uct_progress_types::THREAD_SAFE` is set in `flags`.
	///
	/// NOTE: The initial state is for progressing to be disabled.
	#[inline(always)]
	pub fn enable_progressing(&self, flags: uct_progress_types)
	{
		unsafe { (self.transport_interface_operations().iface_progress_enable)(self.as_ptr(), flags.0) }
	}
	
	/// Disable synchronous progress for the interface.
	///
	/// Equivalent to `uct_iface_progress_disable`.
	///
	/// Notify the transport that it should not progress its communications during `Worker.progress()`.
	///
	/// This may improve the latency of other transports.
	///
	/// NOTE: This function is not thread safe with respect to `Worker.progress()` unless the flag `uct_progress_types::THREAD_SAFE` is set in `flags`.
	///
	/// NOTE: The initial state is for progressing to be disabled.
	#[inline(always)]
	pub fn disable_progressing(&self, flags: uct_progress_types)
	{
		unsafe { (self.transport_interface_operations().iface_progress_disable)(self.as_ptr(), flags.0) }
	}
	
	/// Perform a progress on an interface.
	///
	/// Returns the number of events that occurred.
	///
	/// Equivalent to `uct_iface_progress`.
	#[inline(always)]
	pub fn progress(&self) -> u32
	{
		unsafe { (self.transport_interface_operations().iface_progress)(self.as_ptr()) }
	}
	
	/// Flush outstanding communication operations on an interface.
	///
	/// Equivalent to `uct_iface_flush`.
	///
	/// Flushes all outstanding communications issued on the interface prior to this call.
	/// The operations are completed at the origin or at the target as well. The exact completion semantic depends on flags parameter; currently, only `uct_flush_flags::LOCAL` is supported.
	/// This flag guarantees that the data transfer is completed but the target buffer may not be updated yet.
	#[inline(always)]
	pub fn flush<C: CompletionHandler>(&self, flags: uct_flush_flags, completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		debug_assert_eq!(flags, uct_flush_flags::LOCAL, "Only LOCAL is supported currently");
		
		let status = unsafe { (self.transport_interface_operations().iface_flush)(self.as_ptr(), flags.0, completion.to_raw_pointer()) };
		
		completion.parse_status(status)
	}
	
	/// Ensures ordering of outstanding communications on the interface.
	///
	/// Equivalent to `uct_iface_fence`.
	///
	/// Operations issued on the interface prior to this call are guaranteed to be completed before any subsequent communication operations to the same interface which follow the call to `fence`.
	///
	/// Should only ever return `UCS_OK`.
	#[inline(always)]
	pub fn fence(&self) -> Result<(), ErrorCode>
	{
		let status = unsafe { (self.transport_interface_operations().iface_fence)(self.as_ptr(), ReservedForFutureUseFlags) };
		
		Self::parse_status(status, ())
	}
	
	#[inline(always)]
	fn parse_status<R>(status: ucs_status_t, ok: R) -> Result<R, ErrorCode>
	{
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(ok),
			
			Error(error_code) => Err(error_code),
			
			_ => panic!("Unexpected status '{:?}'", status),
		}
	}
	
	#[inline(always)]
	fn parse_status_with_in_progress<R>(status: ucs_status_t, ok: R) -> Result<NonBlockingRequestCompletedOrInProgress<R, R>, ErrorCode>
	{
		use self::Status::*;
		
		use self::NonBlockingRequestCompletedOrInProgress::*;
		
		match status.parse()
		{
			IsOk => Ok(Completed(ok)),
			
			OperationInProgress => Ok(InProgress(ok)),
			
			Error(error_code) => Err(error_code),
			
			_ => panic!("Unexpected status '{:?}'", status),
		}
	}
	
	#[inline(always)]
	pub(crate) fn transport_interface_operations(&self) -> &mut uct_iface_ops
	{
		&mut unsafe { &mut * self.handle.as_ptr() }.ops
	}
	
	#[inline(always)]
	pub(crate) fn handle_drop_safety(&self) -> Arc<CommunicationInterfaceContextHandleDropSafety>
	{
		self.handle_drop_safety.as_ref().unwrap().clone()
	}
	
	#[inline(always)]
	pub(crate) fn as_ptr(&self) -> *mut uct_iface
	{
		self.handle.as_ptr()
	}
	
	#[inline(always)]
	pub(crate) fn debug_interface_supports_feature(&self, required_to_support: InterfaceFeaturesSupported)
	{
		debug_assert!(self.interface_supports_feature(required_to_support), "Unsupported");
	}
	
	#[inline(always)]
	fn debug_assert_interface_supports_events(&self)
	{
		debug_assert!(self.interface_supports_feature(InterfaceFeaturesSupported::EVENT_SEND_COMP) | self.interface_supports_feature(InterfaceFeaturesSupported::EVENT_RECV) | self.interface_supports_feature(InterfaceFeaturesSupported::EVENT_RECV_SIG), "Interface does not support events")
	}
	
	#[inline(always)]
	pub(crate) fn interface_supports_feature(&self, required_to_support: InterfaceFeaturesSupported) -> bool
	{
		self.attributes().supports_all_of(required_to_support)
	}
	
	/// A callback which handler all incoming active messages which match the `active_message_identifier`.
	///
	/// Equivalent to `uct_iface_set_am_handler`.
	///
	/// `callback_on_receive`, when synchronous, is invoked from the same thread that called `self.progress()`.
	#[inline(always)]
	fn set_active_message_handler_for_active_messages_of_identifier<T>(&self, active_message_identifier: ActiveMessageIdentifier, callback_on_receive: uct_am_callback_t, callback_on_receive_data: *mut T, flags: uct_cb_flags) -> Result<(), ErrorCode>
	{
		if cfg!(debug_assertions)
		{
			let mut has_at_least_sync_or_async = false;
			if flags & uct_cb_flags::SYNC == uct_cb_flags::SYNC
			{
				self.debug_interface_supports_feature(InterfaceFeaturesSupported::CB_SYNC);
				has_at_least_sync_or_async = true;
			}
			if flags & uct_cb_flags::ASYNC == uct_cb_flags::ASYNC
			{
				self.debug_interface_supports_feature(InterfaceFeaturesSupported::CB_ASYNC);
				has_at_least_sync_or_async = true;
			}
			debug_assert!(has_at_least_sync_or_async, "flags must contain either SYNC or ASYNC");
		}
		
		let status = unsafe { uct_iface_set_am_handler(self.as_ptr(), active_message_identifier.0, callback_on_receive, callback_on_receive_data as *mut _, flags.0) };
		
		Self::parse_status(status, ())
	}
	
	/// A tracer of active messages.
	///
	/// Equivalent to `uct_iface_set_am_tracer`.
	///
	/// Sets a function which dumps active message debug information to a buffer, which is printed every time an active message is sent or received, when data tracing is on. Without the tracer, only transport-level information is printed.
	///
	/// Pass `None` to remove the tracer function.
	#[inline(always)]
	fn set_active_message_tracer_ffi<T>(&self, tracer_callback: uct_am_tracer_t, tracer_data: *mut T) -> Result<(), ErrorCode>
	{
		debug_assert!(self.interface_supports_feature(InterfaceFeaturesSupported::CB_SYNC) | self.interface_supports_feature(InterfaceFeaturesSupported::CB_ASYNC), "Interface must support CB_SYNC or CB_ASYNC");
		
		let status = unsafe { uct_iface_set_am_tracer(self.as_ptr(), tracer_callback, tracer_data as *mut _) };
		
		Self::parse_status(status, ())
	}
}
