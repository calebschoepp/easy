use nom::combinator::map;
use nom::sequence::pair;
use nom::IResult;

use crate::{instructions::Expression, Decode};

use super::types::{NumType, ValType};

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Code {
    pub size: u32,
    pub code: Func,
}

impl Decode for Code {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(pair(u32::decode, Func::decode), |pair| Self {
            size: pair.0,
            code: pair.1,
        })(input)
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Func {
    pub locals: Vec<Local>,
    pub body: Expression,
}

impl Decode for Func {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(pair(Vec::<Local>::decode, Expression::decode), |pair| {
            Self {
                locals: pair.0,
                body: pair.1,
            }
        })(input)
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Local {
    pub count: u32,
    pub value_type: ValType,
}

impl Decode for Local {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(pair(u32::decode, ValType::decode), |pair| Self {
            count: pair.0,
            value_type: pair.1,
        })(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    #[test]
    fn test_code() {
        assert!(Code::decode(&[0x06, 0x01, 0x02, 0x7F, 0xC0, 0xC0, 0x0B]).is_ok(),);
        assert!(Code::decode(&[0xFF]).is_err());
    }

    #[test]
    fn test_func() {
        assert!(Func::decode(&[0x01, 0x02, 0x7F, 0xC0, 0xC0, 0x0B]).is_ok(),);
        assert!(Func::decode(&[0xFF]).is_err());
    }

    #[test]
    fn test_local() {
        assert_eq!(
            Local::decode(&[0x02, 0x7F]),
            Ok((
                EMPTY,
                Local {
                    count: 2,
                    value_type: ValType::NumType(NumType::I32)
                }
            ))
        );
        assert!(Local::decode(&[0xFF]).is_err());
    }
}
