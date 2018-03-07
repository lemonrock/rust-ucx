// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A sealing key's bytes suitable for use in configuration.
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SealingKeyBytes
{
	/// Bytes for a ChaCha-20 Poly-1305 sealing key (256-bit).
	ChaCha20Poly1305([u8; 32]),
	
	/// Bytes for an AES-128 GCM sealing key (128-bit).
	Aes128Gcm([u8; 16]),
	
	/// Bytes for an AES-256 GCM sealing key (256-bit).
	Aes256Gcm([u8; 32]),
}

impl SealingKeyBytes
{
	/// Creates a new sealing key from the bytes.
	#[inline(always)]
	pub fn new_sealing_key(&self) -> SealingKey
	{
		use self::SealingKeyBytes::*;
		
		match *self
		{
			ChaCha20Poly1305(ref key_material) => SealingKey::new(&CHACHA20_POLY1305, key_material).expect("Invalid key material"),
			Aes128Gcm(ref key_material) => SealingKey::new(&AES_128_GCM, key_material).expect("Invalid key material"),
			Aes256Gcm(ref key_material) => SealingKey::new(&AES_256_GCM, key_material).expect("Invalid key material"),
		}
	}
}
