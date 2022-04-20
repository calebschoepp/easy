use nom::IResult;

use super::{
    indices::{FuncIdx, GlobalIdx, MemIdx, TableIdx},
    util::Decode,
    values::Name,
};

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Export {
    nm: Name,
    d: ExportDescriptor,
}

impl Decode for Export {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub enum ExportDescriptor {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

impl Decode for ExportDescriptor {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}
