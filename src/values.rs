use crate::util::Decode;
use nom::{combinator::opt, multi::count, IResult};
use nom_leb128::{leb128_i32, leb128_i64, leb128_u32};

impl Decode for u32 {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        leb128_u32(input)
    }
}

impl Decode for u8 {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        nom::number::complete::u8(input)
    }
}

impl Decode for f32 {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        nom::number::complete::le_f32(input)
    }
}

impl Decode for f64 {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        nom::number::complete::le_f64(input)
    }
}

impl Decode for i32 {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        leb128_i32(input)
    }
}

impl Decode for i64 {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        leb128_i64(input)
    }
}

impl<T> Decode for Vec<T>
where
    T: Decode,
{
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, length) = u32::decode(input)?;
        count(T::decode, length as usize)(input)
    }
}

impl<T> Decode for Option<T>
where
    T: Decode,
{
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        opt(T::decode)(input)
    }
}

/// A UTF-8 character sequence
#[derive(Debug, PartialEq)]
pub struct Name(Vec<u8>);

impl Decode for Name {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, name) = <Vec<u8>>::decode(input)?;
        Ok((input, Name(name)))
    }
}
