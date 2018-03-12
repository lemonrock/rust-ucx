// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A simple trait to make it possible to target just unsigned integers.
pub trait UnsignedInteger: Integer
{
	#[inline(always)]
	fn from_u8_or_none(value: u8) -> Result<Self, &'static str>;
	
	#[inline(always)]
	fn from_u16_or_none(value: u16) -> Result<Self, &'static str>;
	
	#[inline(always)]
	fn from_u32_or_none(value: u32) -> Result<Self, &'static str>;
	
	#[inline(always)]
	fn from_u64_or_none(value: u64) -> Result<Self, &'static str>;
	
	#[inline(always)]
	fn from_usize_or_none(value: usize) -> Result<Self, &'static str>;
}

macro_rules! unsigned_integer
{
    ($type: ty) =>
    {
        impl UnsignedInteger for $type
        {
			#[inline(always)]
			fn from_u8_or_none(value: u8) -> Result<Self, &'static str>
			{
				if value <= Self::Maximum as u8
				{
					Ok(value as Self)
				}
				else
				{
					Err("Exceeds maximum value")
				}
			}
			
			#[inline(always)]
			fn from_u16_or_none(value: u16) -> Result<Self, &'static str>
			{
				if value <= Self::Maximum as u16
				{
					Ok(value as Self)
				}
				else
				{
					Err("Exceeds maximum value")
				}
			}
			
			#[inline(always)]
			fn from_u32_or_none(value: u32) -> Result<Self, &'static str>
			{
				if value <= Self::Maximum as u32
				{
					Ok(value as Self)
				}
				else
				{
					Err("Exceeds maximum value")
				}
			}
			
			#[inline(always)]
			fn from_u64_or_none(value: u64) -> Result<Self, &'static str>
			{
				if value <= Self::Maximum as u64
				{
					Ok(value as Self)
				}
				else
				{
					Err("Exceeds maximum value")
				}
			}
			
			#[inline(always)]
			fn from_usize_or_none(value: usize) -> Result<Self, &'static str>
			{
				if value <= Self::Maximum as usize
				{
					Ok(value as Self)
				}
				else
				{
					Err("Exceeds maximum value")
				}
			}
        }
    }
}

unsigned_integer!(u8);
unsigned_integer!(u16);
unsigned_integer!(u32);
unsigned_integer!(u64);
unsigned_integer!(usize);
