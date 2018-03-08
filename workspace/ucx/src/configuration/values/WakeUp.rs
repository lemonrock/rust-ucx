// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Additional reasons to wake up.
#[serde(deny_unknown_fields, default)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WakeUp
{
	/// Wake up on any transmit event (tags, RMA, or AMO32/AMO64).
	pub wake_up_on_transmit: bool,
	
	/// If using a tag sender mask, wake up on tag transmit (send).
	pub wake_up_on_tag_transmit: bool,
	
	/// Wake up on any receive event (tags, RMA, or AMO32/AMO64).
	pub wake_up_on_receive: bool,
	
	/// If using a tag sender mask, wake up on tag receive.
	pub wake_up_on_tag_receive: bool,

	/// Use edge-triggered wakeup.
	/// The event file descriptor will be signaled only for new events, rather than existing ones.
	pub edge: bool,
}

impl Default for WakeUp
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			wake_up_on_transmit: true,
			wake_up_on_tag_transmit: true,
			wake_up_on_receive: true,
			wake_up_on_tag_receive: true,
			edge: false,
		}
	}
}

impl WakeUp
{
	#[inline(always)]
	pub(crate) fn wake_up_events(&self, tag_sender_mask: Option<TagSenderMask>, mut flags: ucp_wakeup_event_types) -> ucp_wakeup_event_types
	{
		if self.wake_up_on_transmit
		{
			flags |= ucp_wakeup_event_types::TX;
		}
		
		if self.wake_up_on_tag_transmit && tag_sender_mask.is_some()
		{
			flags |= ucp_wakeup_event_types::TAG_SEND
		}
		
		if self.wake_up_on_receive
		{
			flags |= ucp_wakeup_event_types::RX;
		}
		
		if self.wake_up_on_tag_receive && tag_sender_mask.is_some()
		{
			flags |= ucp_wakeup_event_types::TAG_RECV
		}
		
		if self.edge
		{
			flags |= ucp_wakeup_event_types::EDGE;
		}
		
		flags
	}
}
