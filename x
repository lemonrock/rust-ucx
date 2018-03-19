#!/bin/sh

set -e

count=0
while [ $count -lt 32 ]
do
	cat <<EOF

	/// Set an active message handler for active message identifier $count.
	#[inline(always)]
	fn set_active_message_handler_$count(&mut self, active_message_handler: A$count, flags: uct_cb_flags) -> Result<(), ErrorCode>
	{
		let former_active_message_handler = self.active_message_handler_$count.take();
		self.active_message_handler_$count = Some(active_message_handler);
		self.set_active_message_handler_for_active_messages_of_identifier(ActiveMessageIdentifier($count), Self::callback_on_active_message_receive_$count, &mut self.active_message_handler_$count as *mut _, flags);
		drop(former_active_message_handler)
	}
EOF
	count=$((count+1))
done