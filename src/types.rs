use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

use crate::util::Decode;

/// Types of numeric values
#[derive(Debug, PartialEq)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

impl Decode for NumType {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            map(tag([0x7F]), |_| NumType::I32),
            map(tag([0x7E]), |_| NumType::I64),
            map(tag([0x7D]), |_| NumType::F32),
            map(tag([0x7C]), |_| NumType::F64),
        ))(input)
    }
}

/// Vector of numeric values processed with SIMD instructions
#[derive(Debug, PartialEq)]
pub enum VecType {
    V128,
}

impl Decode for VecType {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        (map(tag([0x7B]), |_| VecType::V128))(input)
    }
}

/// First-class references to objects in the runtime store
#[derive(Debug, PartialEq)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

impl Decode for RefType {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            map(tag([0x70]), |_| RefType::FuncRef),
            map(tag([0x6F]), |_| RefType::ExternRef),
        ))(input)
    }
}

/// The individual values that Wasm can compute with
#[derive(Debug, PartialEq)]
pub enum ValType {
    NumType(NumType),
    VecType(VecType),
    RefType(RefType),
}

impl Decode for ValType {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            map(NumType::decode, |num_type| ValType::NumType(num_type)),
            map(VecType::decode, |vec_type| ValType::VecType(vec_type)),
            map(RefType::decode, |ref_type| ValType::RefType(ref_type)),
        ))(input)
    }
}

/// The result of executing instructions or functions
type ResultType = Vec<ValType>;

/// A unique function signature
#[derive(Debug, PartialEq)]
pub struct FuncType {
    pub rt1: ResultType,
    pub rt2: ResultType,
}

impl Decode for FuncType {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, _) = tag([0x60])(input)?;
        let (input, rt1) = ResultType::decode(input)?;
        let (input, rt2) = ResultType::decode(input)?;
        Ok((input, FuncType { rt1, rt2 }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    #[test]
    fn test_num_type() {
        assert_eq!(NumType::decode(&[0x7F]), Ok((EMPTY, NumType::I32)));
        assert_eq!(NumType::decode(&[0x7E]), Ok((EMPTY, NumType::I64)));
        assert_eq!(NumType::decode(&[0x7D]), Ok((EMPTY, NumType::F32)));
        assert_eq!(NumType::decode(&[0x7C]), Ok((EMPTY, NumType::F64)));
        assert!(NumType::decode(&[0x7B]).is_err());
    }

    #[test]
    fn test_vec_type() {
        assert_eq!(VecType::decode(&[0x7B]), Ok((EMPTY, VecType::V128)));
        assert!(VecType::decode(&[0x7A]).is_err());
    }

    #[test]
    fn test_ref_type() {
        assert_eq!(RefType::decode(&[0x70]), Ok((EMPTY, RefType::FuncRef)));
        assert_eq!(RefType::decode(&[0x6F]), Ok((EMPTY, RefType::ExternRef)));
        assert!(RefType::decode(&[0x7A]).is_err());
    }

    #[test]
    fn test_val_type() {
        assert_eq!(
            ValType::decode(&[0x7F]),
            Ok((EMPTY, ValType::NumType(NumType::I32)))
        );
        assert_eq!(
            ValType::decode(&[0x7B]),
            Ok((EMPTY, ValType::VecType(VecType::V128)))
        );
        assert_eq!(
            ValType::decode(&[0x6F]),
            Ok((EMPTY, ValType::RefType(RefType::ExternRef)))
        );
        assert!(ValType::decode(&[0x1A]).is_err());
    }

    #[test]
    fn test_result_type() {
        assert_eq!(
            ResultType::decode(&[0x02, 0x7F, 0x7F]),
            Ok((
                EMPTY,
                vec!(
                    ValType::NumType(NumType::I32),
                    ValType::NumType(NumType::I32)
                )
            ))
        );
        let not: &[u8] = &[0x02, 0xDD, 0x7F];
        assert!(ResultType::decode(not).is_err());
    }

    #[test]
    fn test_func_type() {
        assert_eq!(
            FuncType::decode(&[0x60, 0x02, 0x7F, 0x7F, 0x02, 0x7C, 0x7F]),
            Ok((
                EMPTY,
                FuncType {
                    rt1: vec!(
                        ValType::NumType(NumType::I32),
                        ValType::NumType(NumType::I32)
                    ),
                    rt2: vec!(
                        ValType::NumType(NumType::F64),
                        ValType::NumType(NumType::I32)
                    ),
                }
            ))
        );
    }
}
