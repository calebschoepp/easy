use nom::IResult;

use crate::Decode;

use super::types::TableType;

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Table {
    tt: TableType,
}

impl Decode for Table {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO: Write tests
}
