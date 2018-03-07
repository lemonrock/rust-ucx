// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Mask which specifies particular bits of the tag which can uniquely identify the sender (UCP end point) in tagged operations.
#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagSenderMask(u64);

impl TagSenderMask
{
	/// Parses a tag sender mask.
	#[inline(always)]
	pub fn parse(value: u64) -> Option<Self>
	{
		if value == 0
		{
			None
		}
		else
		{
			Some(TagSenderMask(value))
		}
	}
}

impl<'de> Deserialize<'de> for TagSenderMask
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct TagSenderMaskVisitor;
		
		impl<'de> Visitor<'de> for TagSenderMaskVisitor
		{
			type Value = TagSenderMask;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("an integer between -2^31 and 2^31")
			}
			
			#[inline(always)]
			fn visit_i64<E: DeserializeError>(self, value: i64) -> Result<Self::Value, E>
			{
				self.visit_u64(unsafe { transmute(value) })
			}
			
			#[inline(always)]
			fn visit_u64<E: DeserializeError>(self, value: u64) -> Result<Self::Value, E>
			{
				if value == 0
				{
					Err(E::custom(format!("value can not be zero")))
				}
				else
				{
					Ok(TagSenderMask(value))
				}
			}
			
			// Handles hexadecimal and octal strings.
			#[inline(always)]
			fn visit_str<E: DeserializeError>(self, value: &str) -> Result<Self::Value, E>
			{
				if value.starts_with("0x") || value.starts_with("0X")
				{
					let numeric_part = &value[2 .. ];
					if let Ok(value) = u64::from_str_radix(numeric_part, 16)
					{
						self.visit_u64(value)
					}
					else
					{
						Err(E::custom(format!("value was not a hexadecimal string that fitted in an u64")))
					}
				}
				else if value.starts_with("0")
				{
					let numeric_part = &value[1 .. ];
					if let Ok(value) = u64::from_str_radix(numeric_part, 8)
					{
						self.visit_u64(value)
					}
					else
					{
						Err(E::custom(format!("value was not an octal string that fitted in an u64")))
					}
				}
				else
				{
					if let Ok(value) = u64::from_str_radix(value, 10)
					{
						self.visit_u64(value)
					}
					else
					{
						Err(E::custom(format!("value was not a decimal string that fitted in an u64")))
					}
				}
			}
		}
		
		deserializer.deserialize_u64(TagSenderMaskVisitor)
	}
}
