// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A wrapping new-type to make it clear that a value is always in Big-Endian form, regardless of underlying processor architecture.
#[derive(Serialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct BigEndian<I: UnsignedInteger>(I);

impl<'de, I: UnsignedInteger> Deserialize<'de> for BigEndian<I>
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct BigEndianVisitor<I: Integer>(PhantomData<I>);
		
		impl<'de, I: UnsignedInteger> Visitor<'de> for BigEndianVisitor<I>
		{
			type Value = BigEndian<I>;
			
			#[inline(always)]
			fn visit_u64<E: DeserializeError>(self, value: u64) -> Result<Self::Value, E>
			{
				I::from_u64_or_none(value).map(|value| BigEndian(value)).map_err(|message| E::custom(message))
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
			
			#[inline(always)]
			fn visit_u8<E: DeserializeError>(self, value: u8) -> Result<Self::Value, E>
			{
				self.visit_u64(value as u64)
			}
			
			#[inline(always)]
			fn visit_u16<E: DeserializeError>(self, value: u16) -> Result<Self::Value, E>
			{
				self.visit_u64(value as u64)
			}
			
			#[inline(always)]
			fn visit_u32<E: DeserializeError>(self, value: u32) -> Result<Self::Value, E>
			{
				self.visit_u64(value as u64)
			}
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("an unsigned integer")
			}
		}
		
		deserializer.deserialize_u64(BigEndianVisitor(PhantomData))
	}
}

impl<I: UnsignedInteger> BitOr for BigEndian<I>
{
	type Output = Self;
	
	fn bitor(self, rhs: Self) -> Self::Output
	{
		BigEndian(self.0 | rhs.0)
	}
}

impl<I: UnsignedInteger> BigEndian<I>
{
	/// Zero.
	pub const Zero: Self = BigEndian(I::Zero);
	
	/// From an existing big endian value.
	#[inline(always)]
	pub fn from_big_endian_value(big_endian_value: I) -> Self
	{
		BigEndian(big_endian_value)
	}
	
	/// From native endian value.
	#[inline(always)]
	pub fn from_native_endian_value(native_endian_value: I) -> Self
	{
		BigEndian(native_endian_value.to_big_endian())
	}
	
	#[inline(always)]
	pub fn as_raw_big_endian_value(self) -> I
	{
		self.0
	}
	
	#[inline(always)]
	pub fn as_raw_native_endian_value(self) -> I
	{
		I::from_big_endian(self.0)
	}
	
	#[inline(always)]
	pub fn is_zero(self) -> bool
	{
		self.0.is_zero()
	}
}
