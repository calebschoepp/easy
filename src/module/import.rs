use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{pair, tuple},
    IResult,
};

use crate::Decode;

use super::{
    indices::TypeIdx,
    types::{GlobalType, MemType, TableType},
    values::Name,
};

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Import {
    pub module: Name,
    pub name: Name,
    pub descriptor: ImportDescriptor,
}

impl Decode for Import {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((Name::decode, Name::decode, ImportDescriptor::decode)),
            |tuple| Import {
                module: tuple.0,
                name: tuple.1,
                descriptor: tuple.2,
            },
        )(input)
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub enum ImportDescriptor {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

impl Decode for ImportDescriptor {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            map(pair(tag([0x00]), TypeIdx::decode), |pair| {
                Self::Func(pair.1)
            }),
            map(pair(tag([0x01]), TableType::decode), |pair| {
                Self::Table(pair.1)
            }),
            map(pair(tag([0x02]), MemType::decode), |pair| Self::Mem(pair.1)),
            map(pair(tag([0x03]), GlobalType::decode), |pair| {
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
    fn test_import() {
        assert_eq!(
            Import::decode(&[0x01, 0xAA, 0x01, 0xBB, 0x00, 0x00]),
            Ok((
                EMPTY,
                Import {
                    module: Name(vec!(0xAA)),
                    name: Name(vec!(0xBB)),
                    descriptor: ImportDescriptor::Func(0)
                }
            ))
        );
        assert!(Import::decode(&[0x7A]).is_err());
    }

    #[test]
    fn test_import_descriptor() {
        assert_eq!(
            ImportDescriptor::decode(&[0x00, 0x00]),
            Ok((EMPTY, ImportDescriptor::Func(0)))
        );
        assert!(ImportDescriptor::decode(&[0x7A]).is_err());
    }
}
