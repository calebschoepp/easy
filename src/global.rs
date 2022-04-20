use nom::IResult;

use crate::{instructions::Expression, types::GlobalType, util::Decode};

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Global {
    gt: GlobalType,
    init: Expression,
}

impl Decode for Global {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}
