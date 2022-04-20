use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{pair, preceded},
    IResult,
};

use super::util::Decode;

/// Classify numeric values
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

/// Classify vectors of numeric values processed by vector instructions (also known as SIMD instructions, single instruction multiple data)
#[derive(Debug, PartialEq)]
pub enum VecType {
    V128,
}

impl Decode for VecType {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        (map(tag([0x7B]), |_| VecType::V128))(input)
    }
}

/// Classify first-class references to objects in the runtime store
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

/// Classify the individual values that WebAssembly code can compute with and the values that a variable accepts. They are either number types, vector types, or reference types
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

/// Classify the result of executing instructions or functions, which is a sequence of values, written with brackets
type ResultType = Vec<ValType>;

/// Classify the signature of functions, mapping a vector of parameters to a vector of results. They are also used to classify the inputs and outputs of instructions
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

/// Classify the size range of resizable storage associated with memory types and table types
#[derive(Debug, PartialEq)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

impl Decode for Limits {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            map(preceded(tag([0x00]), u32::decode), |min| Limits {
                min,
                max: None,
            }),
            map(
                preceded(tag([0x01]), pair(u32::decode, u32::decode)),
                |pair| Limits {
                    min: pair.0,
                    max: Some(pair.1),
                },
            ),
        ))(input)
    }
}

/// Classify linear memories and their size range
#[derive(Debug, PartialEq)]
pub struct MemType {
    pub lim: Limits,
}

impl Decode for MemType {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(Limits::decode, |limits| Self { lim: limits })(input)
    }
}

/// Classify tables over elements of reference type within a size range
#[derive(Debug, PartialEq)]
pub struct TableType {
    pub lim: Limits,
    pub et: RefType,
}

impl Decode for TableType {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(pair(RefType::decode, Limits::decode), |pair| Self {
            lim: pair.1,
            et: pair.0,
        })(input)
    }
}

/// Classify global variables, which hold a value and can either be mutable or immutable
#[derive(Debug, PartialEq)]
pub struct GlobalType {
    pub m: Mutability,
    pub t: ValType,
}

impl Decode for GlobalType {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(pair(ValType::decode, Mutability::decode), |pair| Self {
            m: pair.1,
            t: pair.0,
        })(input)
    }
}

/// Classify whether something is mutable
#[derive(Debug, PartialEq)]
pub enum Mutability {
    Const,
    Var,
}

impl Decode for Mutability {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            map(tag([0x00]), |_| Self::Const),
            map(tag([0x01]), |_| Self::Var),
        ))(input)
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

    #[test]
    fn test_limits_type() {
        assert_eq!(
            Limits::decode(&[0x00, 0x01]),
            Ok((EMPTY, Limits { min: 1, max: None }))
        );
        assert_eq!(
            Limits::decode(&[0x01, 0x01, 0x02]),
            Ok((
                EMPTY,
                Limits {
                    min: 1,
                    max: Some(2)
                }
            ))
        );
        assert!(Limits::decode(&[0x7A]).is_err());
    }

    #[test]
    fn test_mem_type() {
        assert_eq!(
            MemType::decode(&[0x00, 0x01]),
            Ok((
                EMPTY,
                MemType {
                    lim: Limits { min: 1, max: None }
                }
            ))
        );
        assert_eq!(
            MemType::decode(&[0x01, 0x01, 0x02]),
            Ok((
                EMPTY,
                MemType {
                    lim: Limits {
                        min: 1,
                        max: Some(2)
                    }
                }
            ))
        );
        assert!(MemType::decode(&[0x7A]).is_err());
    }

    #[test]
    fn test_table_type() {
        assert_eq!(
            TableType::decode(&[0x70, 0x00, 0x01]),
            Ok((
                EMPTY,
                TableType {
                    lim: Limits { min: 1, max: None },
                    et: RefType::FuncRef
                }
            ))
        );
        assert_eq!(
            TableType::decode(&[0x70, 0x01, 0x01, 0x02]),
            Ok((
                EMPTY,
                TableType {
                    lim: Limits {
                        min: 1,
                        max: Some(2)
                    },
                    et: RefType::FuncRef
                }
            ))
        );
        assert!(TableType::decode(&[0x7A]).is_err());
    }

    #[test]
    fn test_global_type() {
        assert_eq!(
            GlobalType::decode(&[0x7F, 0x00]),
            Ok((
                EMPTY,
                GlobalType {
                    m: Mutability::Const,
                    t: ValType::NumType(NumType::I32),
                }
            ))
        );
        assert!(GlobalType::decode(&[0x7A]).is_err());
    }

    #[test]
    fn test_mutability_type() {
        assert_eq!(Mutability::decode(&[0x00]), Ok((EMPTY, Mutability::Const)));
        assert_eq!(Mutability::decode(&[0x01]), Ok((EMPTY, Mutability::Var)));
        assert!(Mutability::decode(&[0x7A]).is_err());
    }
}
