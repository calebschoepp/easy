use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::many_till, IResult};

use crate::Decode;

use self::{
    numeric::{NumericInstruction, SaturatingTruncationInstruction},
    parametric::ParametricInstruction,
    reference::ReferenceInstruction,
};

mod numeric;
mod parametric;
mod reference;

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Expression(Vec<Instruction>);

impl Decode for Expression {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(many_till(Instruction::decode, tag([0x0B])), |pair| {
            Expression(pair.0)
        })(input)
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
enum Instruction {
    //     Control(ControlInstruction),
    Reference(ReferenceInstruction),
    Parametric(ParametricInstruction),
    //     Variable(VariableInstruction),
    //     Table(TableInstruction),
    //     Memory(MemoryInstruction),
    Numeric(NumericInstruction),
    Saturating(SaturatingTruncationInstruction),
}

impl Decode for Instruction {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            // map(ControlInstruction::decode, |instruction| {
            //     Self::Control(instruction)
            // }),
            map(ReferenceInstruction::decode, |instruction| {
                Self::Reference(instruction)
            }),
            map(ParametricInstruction::decode, |instruction| {
                Self::Parametric(instruction)
            }),
            // map(VariableInstruction::decode, |instruction| {
            //     Self::Variable(instruction)
            // }),
            // map(TableInstruction::decode, |instruction| {
            //     Self::Table(instruction)
            // }),
            // map(MemoryInstruction::decode, |instruction| {
            //     Self::Memory(instruction)
            // }),
            map(NumericInstruction::decode, |instruction| {
                Self::Numeric(instruction)
            }),
            map(SaturatingTruncationInstruction::decode, |instruction| {
                Self::Saturating(instruction)
            }),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    #[test]
    fn test_expression() {
        assert_eq!(
            Expression::decode(&[0xC0, 0xC0, 0x0B]),
            Ok((
                EMPTY,
                Expression(vec!(
                    Instruction::Numeric(NumericInstruction::I32Extend8S),
                    Instruction::Numeric(NumericInstruction::I32Extend8S)
                ))
            ))
        );
        assert!(Expression::decode(&[0xFF]).is_err());
    }

    #[test]
    fn test_instruction() {
        assert_eq!(
            Instruction::decode(&[0xC0]),
            Ok((EMPTY, Instruction::Numeric(NumericInstruction::I32Extend8S)))
        );
        assert!(Instruction::decode(&[0xFF]).is_err());
    }
}
