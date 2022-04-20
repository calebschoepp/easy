use nom::IResult;

use crate::{instructions::Expression, Decode};

use super::types::GlobalType;

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

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO: Write tests
}
