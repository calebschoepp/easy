use crate::module::indices::GlobalIdx;
use crate::module::indices::LocalIdx;
use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::pair, IResult};

use crate::{
    module::types::{RefType, ValType},
    Decode,
};

/// TODO: Document
#[derive(Debug, PartialEq)]
pub enum VariableInstruction {
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),
}

impl Decode for VariableInstruction {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            map(pair(tag([0x20]), LocalIdx::decode), |pair| {
                Self::LocalGet(pair.1)
            }),
            map(pair(tag([0x21]), LocalIdx::decode), |pair| {
                Self::LocalSet(pair.1)
            }),
            map(pair(tag([0x22]), LocalIdx::decode), |pair| {
                Self::LocalTee(pair.1)
            }),
            map(pair(tag([0x23]), GlobalIdx::decode), |pair| {
                Self::GlobalGet(pair.1)
            }),
            map(pair(tag([0x24]), GlobalIdx::decode), |pair| {
                Self::GlobalSet(pair.1)
            }),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    #[test]
    fn test_parametric_instruction() {
        assert_eq!(
            VariableInstruction::decode(&[0x20, 0x01]),
            Ok((EMPTY, VariableInstruction::LocalGet(1)))
        );
        assert!(VariableInstruction::decode(&[0xFF]).is_err());
    }
}
