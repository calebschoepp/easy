use crate::module::util::Decode;
use nom::IResult;

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Expression(Vec<Instruction>);

impl Decode for Expression {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
enum Instruction {}

impl Decode for Instruction {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}
