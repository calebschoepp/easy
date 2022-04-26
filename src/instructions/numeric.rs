use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, verify},
    sequence::pair,
    IResult,
};

use crate::Decode;

#[derive(Debug, PartialEq)]
pub enum NumericInstruction {
    // const
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    // i32
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32NeU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    // i64
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64NeU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    // f32
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    // f64
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    // i32
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    // i64
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    // f32
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32CopySign,
    // f64
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64CopySign,
    // conversion
    // i32
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    // i64
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    // f32
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64u,
    F32DemoteF64,
    // f64
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64u,
    F64PromoteF32,
    // reinterpretation
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
    // extend
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
}

impl Decode for NumericInstruction {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        // Nested alts used to avoid limitation of only 21 clauses allowed in alt
        alt((
            // const
            alt((
                map(pair(tag([0x41]), i32::decode), |pair| {
                    Self::I32Const(pair.1)
                }),
                map(pair(tag([0x42]), i64::decode), |pair| {
                    Self::I64Const(pair.1)
                }),
                map(pair(tag([0x43]), f32::decode), |pair| {
                    Self::F32Const(pair.1)
                }),
                map(pair(tag([0x44]), f64::decode), |pair| {
                    Self::F64Const(pair.1)
                }),
            )),
            // i32
            alt((
                map(tag([0x45]), |_| Self::I32Eqz),
                map(tag([0x46]), |_| Self::I32Eq),
                map(tag([0x47]), |_| Self::I32Ne),
                map(tag([0x48]), |_| Self::I32LtS),
                map(tag([0x49]), |_| Self::I32NeU),
                map(tag([0x4A]), |_| Self::I32GtS),
                map(tag([0x4B]), |_| Self::I32GtU),
                map(tag([0x4C]), |_| Self::I32LeS),
                map(tag([0x4D]), |_| Self::I32LeU),
                map(tag([0x4E]), |_| Self::I32GeS),
                map(tag([0x4F]), |_| Self::I32GeU),
            )),
            // i64
            alt((
                map(tag([0x50]), |_| Self::I64Eqz),
                map(tag([0x51]), |_| Self::I64Eq),
                map(tag([0x52]), |_| Self::I64Ne),
                map(tag([0x53]), |_| Self::I64LtS),
                map(tag([0x54]), |_| Self::I64NeU),
                map(tag([0x55]), |_| Self::I64GtS),
                map(tag([0x56]), |_| Self::I64GtU),
                map(tag([0x57]), |_| Self::I64LeS),
                map(tag([0x58]), |_| Self::I64LeU),
                map(tag([0x59]), |_| Self::I64GeS),
                map(tag([0x5A]), |_| Self::I64GeU),
            )),
            // f32
            alt((
                map(tag([0x5B]), |_| Self::F32Eq),
                map(tag([0x5C]), |_| Self::F32Ne),
                map(tag([0x5D]), |_| Self::F32Lt),
                map(tag([0x5E]), |_| Self::F32Gt),
                map(tag([0x5F]), |_| Self::F32Le),
                map(tag([0x60]), |_| Self::F32Ge),
            )),
            // f64
            alt((
                map(tag([0x61]), |_| Self::F64Eq),
                map(tag([0x62]), |_| Self::F64Ne),
                map(tag([0x63]), |_| Self::F64Lt),
                map(tag([0x64]), |_| Self::F64Gt),
                map(tag([0x65]), |_| Self::F64Le),
                map(tag([0x66]), |_| Self::F64Ge),
            )),
            // i32
            alt((
                map(tag([0x45]), |_| Self::I32Eqz),
                map(tag([0x67]), |_| Self::I32Clz),
                map(tag([0x68]), |_| Self::I32Ctz),
                map(tag([0x69]), |_| Self::I32Popcnt),
                map(tag([0x6A]), |_| Self::I32Add),
                map(tag([0x6B]), |_| Self::I32Sub),
                map(tag([0x6C]), |_| Self::I32Mul),
                map(tag([0x6D]), |_| Self::I32DivS),
                map(tag([0x6E]), |_| Self::I32DivU),
                map(tag([0x6F]), |_| Self::I32RemS),
                map(tag([0x70]), |_| Self::I32RemU),
                map(tag([0x71]), |_| Self::I32And),
                map(tag([0x72]), |_| Self::I32Or),
                map(tag([0x73]), |_| Self::I32Xor),
                map(tag([0x74]), |_| Self::I32Shl),
                map(tag([0x75]), |_| Self::I32ShrS),
                map(tag([0x76]), |_| Self::I32ShrU),
                map(tag([0x77]), |_| Self::I32Rotl),
                map(tag([0x78]), |_| Self::I32Rotr),
            )),
            // i64
            alt((
                map(tag([0x79]), |_| Self::I64Clz),
                map(tag([0x7A]), |_| Self::I64Ctz),
                map(tag([0x7B]), |_| Self::I64Popcnt),
                map(tag([0x7C]), |_| Self::I64Add),
                map(tag([0x7D]), |_| Self::I64Sub),
                map(tag([0x7E]), |_| Self::I64Mul),
                map(tag([0x7F]), |_| Self::I64DivS),
                map(tag([0x80]), |_| Self::I64DivU),
                map(tag([0x81]), |_| Self::I64RemS),
                map(tag([0x82]), |_| Self::I64RemU),
                map(tag([0x83]), |_| Self::I64And),
                map(tag([0x84]), |_| Self::I64Or),
                map(tag([0x85]), |_| Self::I64Xor),
                map(tag([0x86]), |_| Self::I64Shl),
                map(tag([0x87]), |_| Self::I64ShrS),
                map(tag([0x88]), |_| Self::I64ShrU),
                map(tag([0x89]), |_| Self::I64Rotl),
                map(tag([0x8A]), |_| Self::I64Rotr),
            )),
            // f32
            alt((
                map(tag([0x8B]), |_| Self::F32Abs),
                map(tag([0x8C]), |_| Self::F32Neg),
                map(tag([0x8D]), |_| Self::F32Ceil),
                map(tag([0x8E]), |_| Self::F32Floor),
                map(tag([0x8F]), |_| Self::F32Trunc),
                map(tag([0x90]), |_| Self::F32Nearest),
                map(tag([0x91]), |_| Self::F32Sqrt),
                map(tag([0x92]), |_| Self::F32Add),
                map(tag([0x93]), |_| Self::F32Sub),
                map(tag([0x94]), |_| Self::F32Mul),
                map(tag([0x95]), |_| Self::F32Div),
                map(tag([0x96]), |_| Self::F32Min),
                map(tag([0x97]), |_| Self::F32Max),
                map(tag([0x98]), |_| Self::F32CopySign),
            )),
            // f64
            alt((
                map(tag([0x99]), |_| Self::F64Abs),
                map(tag([0x9A]), |_| Self::F64Neg),
                map(tag([0x9B]), |_| Self::F64Ceil),
                map(tag([0x9C]), |_| Self::F64Floor),
                map(tag([0x9D]), |_| Self::F64Trunc),
                map(tag([0x9E]), |_| Self::F64Nearest),
                map(tag([0x9F]), |_| Self::F64Sqrt),
                map(tag([0xA0]), |_| Self::F64Add),
                map(tag([0xA1]), |_| Self::F64Sub),
                map(tag([0xA2]), |_| Self::F64Mul),
                map(tag([0xA3]), |_| Self::F64Div),
                map(tag([0xA4]), |_| Self::F64Min),
                map(tag([0xA5]), |_| Self::F64Max),
                map(tag([0xA6]), |_| Self::F64CopySign),
            )),
            // conversion
            // i32
            alt((
                map(tag([0xA7]), |_| Self::I32WrapI64),
                map(tag([0xA8]), |_| Self::I32TruncF32S),
                map(tag([0xA9]), |_| Self::I32TruncF32U),
                map(tag([0xAA]), |_| Self::I32TruncF64S),
                map(tag([0xAB]), |_| Self::I32TruncF64U),
            )),
            // i64
            alt((
                map(tag([0xAC]), |_| Self::I64ExtendI32S),
                map(tag([0xAD]), |_| Self::I64ExtendI32U),
                map(tag([0xAE]), |_| Self::I64TruncF32S),
                map(tag([0xAF]), |_| Self::I64TruncF32U),
                map(tag([0xB0]), |_| Self::I64TruncF64S),
                map(tag([0xB1]), |_| Self::I64TruncF64U),
            )),
            // f32
            alt((
                map(tag([0xB2]), |_| Self::F32ConvertI32S),
                map(tag([0xB3]), |_| Self::F32ConvertI32U),
                map(tag([0xB4]), |_| Self::F32ConvertI64S),
                map(tag([0xB5]), |_| Self::F32ConvertI64u),
                map(tag([0xB6]), |_| Self::F32DemoteF64),
            )),
            // f64
            alt((
                map(tag([0xB7]), |_| Self::F64ConvertI32S),
                map(tag([0xB8]), |_| Self::F64ConvertI32U),
                map(tag([0xB9]), |_| Self::F64ConvertI64S),
                map(tag([0xBA]), |_| Self::F64ConvertI64u),
                map(tag([0xBB]), |_| Self::F64PromoteF32),
            )),
            // reinterpretation
            alt((
                map(tag([0xBC]), |_| Self::I32ReinterpretF32),
                map(tag([0xBD]), |_| Self::I64ReinterpretF64),
                map(tag([0xBE]), |_| Self::F32ReinterpretI32),
                map(tag([0xBF]), |_| Self::F64ReinterpretI64),
            )),
            // extend
            alt((
                map(tag([0xC0]), |_| Self::I32Extend8S),
                map(tag([0xC1]), |_| Self::I32Extend16S),
                map(tag([0xC2]), |_| Self::I64Extend8S),
                map(tag([0xC3]), |_| Self::I64Extend16S),
                map(tag([0xC4]), |_| Self::I64Extend32S),
            )),
        ))(input)
    }
}

#[derive(Debug, PartialEq)]
pub enum SaturatingTruncationInstruction {
    I32TruncSatF32S,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,
}

impl Decode for SaturatingTruncationInstruction {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, _) = tag([0xFC])(input)?;
        let (input, prefix) = verify(u32::decode, |prefix| *prefix <= 7)(input)?;
        let instruction = match prefix {
            0 => Self::I32TruncSatF32S,
            1 => Self::I32TruncSatF32U,
            2 => Self::I32TruncSatF64S,
            3 => Self::I32TruncSatF64U,
            4 => Self::I64TruncSatF32S,
            5 => Self::I64TruncSatF32U,
            6 => Self::I64TruncSatF64S,
            7 => Self::I64TruncSatF64U,
            _ => unreachable!(),
        };
        Ok((input, instruction))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    #[test]
    fn test_numeric_instruction() {
        assert_eq!(
            NumericInstruction::decode(&[0xC0]),
            Ok((EMPTY, NumericInstruction::I32Extend8S))
        );
        assert_eq!(
            NumericInstruction::decode(&[0x41, 0x08]),
            Ok((EMPTY, NumericInstruction::I32Const(8)))
        );
        assert!(NumericInstruction::decode(&[0xFF]).is_err());
    }

    #[test]
    fn test_saturating_instruction() {
        assert_eq!(
            SaturatingTruncationInstruction::decode(&[0xFC, 0x00]),
            Ok((EMPTY, SaturatingTruncationInstruction::I32TruncSatF32S))
        );
        assert!(SaturatingTruncationInstruction::decode(&[0xFF]).is_err());
    }
}
