use nom::IResult;

// TODO: This file shouldn't exist

/// TODO: Document
pub trait Decode
where
    Self: Sized,
{
    /// Decode the given bytes into the type Self
    fn decode<'a>(input: &'a [u8]) -> IResult<&[u8], Self, nom::error::Error<&'a [u8]>>;
}
