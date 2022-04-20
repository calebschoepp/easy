use nom::IResult;

use crate::{types::MemType, util::Decode};

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Memory {
    mt: MemType,
}

impl Decode for Memory {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}
