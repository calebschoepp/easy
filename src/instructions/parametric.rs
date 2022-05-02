use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::pair, IResult};

use crate::{
    module::types::{RefType, ValType},
    Decode,
};

/// TODO: Document
#[derive(Debug, PartialEq)]
pub enum ParametricInstruction {
    Drop,
    Select,
    SelectTyped(Vec<ValType>),
}

impl Decode for ParametricInstruction {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            map(tag([0x1A]), |_| Self::Drop),
            map(tag([0x1B]), |_| Self::Select),
            map(pair(tag([0x1C]), Vec::<ValType>::decode), |pair| {
                Self::SelectTyped(pair.1)
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
            ParametricInstruction::decode(&[0x1A]),
            Ok((EMPTY, ParametricInstruction::Drop))
        );
        assert_eq!(
            ParametricInstruction::decode(&[0x1C, 0x01, 0x70]),
            Ok((
                EMPTY,
                ParametricInstruction::SelectTyped(vec!(ValType::RefType(RefType::FuncRef)))
            ))
        );
        assert!(ParametricInstruction::decode(&[0xFF]).is_err());
    }
}
