// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A sealed item of data.
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Sealed
{
	nonce: [u8; Sealed::MaximumNonceLength],
	cipher_text_and_tag: Vec<u8>,
	additional_authenticated_data: Vec<u8>,
}

impl Sealed
{
	/// All the AEADs we support use 96-bit nonces.
	pub const MaximumNonceLength: usize = 96 / 8;
	
	#[inline(always)]
	fn seal(sealing_key: &SealingKey, random_source: &SystemRandom, address: NonNull<u8>, length: usize) -> Self
	{
		let mut nonce: [u8; Self::MaximumNonceLength] = unsafe { uninitialized() };
		random_source.fill(&mut nonce).expect("Could not fill nonce with random bytes");
		
		let additional_authenticated_data = Vec::new();
		
		// Currently always 128-bit (16 bytes).
		let out_suffix_capacity = sealing_key.algorithm().tag_len();
		
		let mut in_out = Vec::with_capacity(length + out_suffix_capacity);
		unsafe { copy_nonoverlapping(address.as_ptr(), in_out.as_mut_ptr(), length) };
		
		let out_length = seal_in_place(sealing_key, &nonce, &additional_authenticated_data, &mut in_out, out_suffix_capacity).expect("Could not seal");
		unsafe { in_out.set_len(out_length) };
		Self
		{
			nonce,
			cipher_text_and_tag: in_out,
			additional_authenticated_data,
		}
	}
	
	/// Returns plain text bytes.
	/// Returns an error if decryption fails.
	#[inline(always)]
	pub fn open(self, opening_key: &OpeningKey) -> Result<Vec<u8>, ()>
	{
		let mut cipher_text_and_tag = self.cipher_text_and_tag;
		let plain_text_length =
		{
			let plain_text = open_in_place(opening_key, &self.nonce, &self.additional_authenticated_data[..], 0, &mut cipher_text_and_tag).map_err(|_| ())?;
			plain_text.len()
		};
		unsafe { cipher_text_and_tag.set_len(plain_text_length) };
		cipher_text_and_tag.shrink_to_fit();
		Ok(cipher_text_and_tag)
	}
}
