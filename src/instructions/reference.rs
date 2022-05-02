use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::pair, IResult};

use crate::{
    module::{indices::FuncIdx, types::RefType},
    Decode,
};

/// TODO: Document
#[derive(Debug, PartialEq)]
pub enum ReferenceInstruction {
    Null(RefType),
    IsNull,
    Func(FuncIdx),
}

impl Decode for ReferenceInstruction {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            map(pair(tag([0xD0]), RefType::decode), |pair| {
                Self::Null(pair.1)
            }),
            map(tag([0xD1]), |_| Self::IsNull),
            map(pair(tag([0xD0]), FuncIdx::decode), |pair| {
                Self::Func(pair.1)
            }),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    #[test]
    fn test_reference_instruction() {
        assert_eq!(
            ReferenceInstruction::decode(&[0xD1]),
            Ok((EMPTY, ReferenceInstruction::IsNull))
        );
        assert_eq!(
            ReferenceInstruction::decode(&[0xD0, 0x70]),
            Ok((EMPTY, ReferenceInstruction::Null(RefType::FuncRef)))
        );
        assert!(ReferenceInstruction::decode(&[0xFF]).is_err());
    }
}
