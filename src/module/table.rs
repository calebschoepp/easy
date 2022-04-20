use nom::IResult;

use super::{types::TableType, util::Decode};

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
