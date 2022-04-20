use nom::IResult;

use crate::{
    indices::TypeIdx,
    types::{GlobalType, MemType, TableType},
    util::Decode,
    values::Name,
};

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Import {
    module: Name,
    name: Name,
    descriptor: ImportDescriptor,
}

impl Decode for Import {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub enum ImportDescriptor {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

impl Decode for ImportDescriptor {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}
