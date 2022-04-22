use nom::{combinator::map, IResult};

use crate::Decode;

use super::types::MemType;

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Memory {
    pub mt: MemType,
}

impl Decode for Memory {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        map(MemType::decode, |mt| Self { mt })(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::module::types::Limits;

    use super::*;

    const EMPTY: &[u8] = &[];

    #[test]
    fn test_memory() {
        assert_eq!(
            Memory::decode(&[0x00, 0x00]),
            Ok((
                EMPTY,
                Memory {
                    mt: MemType {
                        lim: Limits { min: 0, max: None }
                    }
                }
            ))
        );
        assert!(Memory::decode(&[0x7A]).is_err());
    }
}
