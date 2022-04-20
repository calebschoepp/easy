use nom::IResult;

use crate::{instructions::Expression, Decode};

use super::types::ValType;

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Code {
    pub size: u32,
    pub code: Func,
}

impl Decode for Code {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Func {
    pub locals: Vec<Local>,
    pub body: Expression,
}

impl Decode for Func {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Local {
    pub count: u32,
    pub value_type: ValType,
}

impl Decode for Local {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO: Write tests
}
