// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Configuration suitable for use with Serde and so JSON, HJSON, XML, etc.
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration
{
	/// Used to control encryption of AEAD-encrypted messages sent out-of-band to other peers.
	pub secret_key_bytes: SecretKeyBytes,
	
	/// UCX settings.
	#[serde(default)] pub ucx_settings: UcpSettings,
	
	/// Application context configuration details.
	#[serde(default)] pub application_context: ApplicationContextConfiguration,
}

impl Configuration
{
	/// Creates a new application context.
	#[inline(always)]
	pub fn new_application_context<MemoryCustomization: NonBlockingRequestMemoryCustomization>(self) -> Result<ApplicationContext<MemoryCustomization>, ApplicationContextCreationError>
	{
		let sealing_key = self.secret_key_bytes.new_sealing_key();
		let opening_key = self.secret_key_bytes.new_opening_key();
		
		let ucp_configuration_wrapper = self.ucx_settings.ucp_configuration_wrapper()?;
		
		self.application_context.new(sealing_key, opening_key, ucp_configuration_wrapper)
	}
}
