use nom::IResult;

use crate::Decode;

use super::types::MemType;

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

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO: Write tests
}
