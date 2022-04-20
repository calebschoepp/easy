use nom::IResult;

use crate::instructions::Expression;

use super::{indices::MemIdx, util::Decode};

/// TODO: Document
#[derive(Debug, PartialEq)]
pub enum Data {
    Active(Expression, Vec<u8>),
    Passive(Vec<u8>),
    ActiveExplicit(MemIdx, Expression, Vec<u8>),
}

impl Decode for Data {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}
