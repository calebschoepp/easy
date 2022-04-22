use nom::{combinator::map, IResult};

use crate::Decode;

use super::types::TableType;

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Table {
    pub tt: TableType,
}

impl Decode for Table {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(TableType::decode, |tt| Table { tt })(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::module::types::{Limits, RefType};

    use super::*;

    const EMPTY: &[u8] = &[];

    #[test]
    fn test_table() {
        assert_eq!(
            Table::decode(&[0x70, 0x01, 0x00, 0x01]),
            Ok((
                EMPTY,
                Table {
                    tt: TableType {
                        lim: Limits {
                            min: 0,
                            max: Some(1)
                        },
                        et: RefType::FuncRef
                    }
                }
            ))
        );
        assert!(Table::decode(&[0x7A]).is_err());
    }
}
