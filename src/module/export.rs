use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::pair, IResult};

use crate::Decode;

use super::{
    indices::{FuncIdx, GlobalIdx, MemIdx, TableIdx},
    values::Name,
};

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Export {
    pub name: Name,
    pub descriptor: ExportDescriptor,
}

impl Decode for Export {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(pair(Name::decode, ExportDescriptor::decode), |pair| Self {
            name: pair.0,
            descriptor: pair.1,
        })(input)
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub enum ExportDescriptor {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

impl Decode for ExportDescriptor {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            map(pair(tag([0x00]), FuncIdx::decode), |pair| {
                Self::Func(pair.1)
            }),
            map(pair(tag([0x01]), TableIdx::decode), |pair| {
                Self::Table(pair.1)
            }),
            map(pair(tag([0x02]), MemIdx::decode), |pair| Self::Mem(pair.1)),
            map(pair(tag([0x03]), GlobalIdx::decode), |pair| {
                Self::Global(pair.1)
            }),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    #[test]
    fn test_export() {
        assert_eq!(
            Export::decode(&[0x01, 0xAA, 0x00, 0x00]),
            Ok((
                EMPTY,
                Export {
                    name: Name(vec!(0xAA)),
                    descriptor: ExportDescriptor::Func(0)
                }
            ))
        );
        assert!(Export::decode(&[0x7A]).is_err());
    }

    #[test]
    fn test_export_descriptor() {
        assert_eq!(
            ExportDescriptor::decode(&[0x00, 0x00]),
            Ok((EMPTY, ExportDescriptor::Func(0)))
        );
        assert!(ExportDescriptor::decode(&[0x7A]).is_err());
    }
}
