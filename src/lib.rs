use nom::{bytes::complete::tag, combinator::map, IResult};

/*
Types
Funcs
Tables
Mems
Globals
Elems
Datas
Start?
Imports
Exports
*/

/// A Wasm module
#[derive(Debug)]
pub struct Module {
    /// Module types
    _types: Vec<Type>,
}

/// A unique function signature
#[derive(Debug)]
struct Type {}

pub trait Decode
where
    Self: Sized,
{
    fn decode<'a>(input: &'a [u8]) -> IResult<&[u8], Self, nom::error::Error<&'a [u8]>>;
}

fn magic_header<'a>(input: &'a [u8]) -> IResult<&[u8], ()> {
    // The magic header that every Wasm module begins with
    let magic_header = [0x00, 0x61, 0x73, 0x6D];
    map(tag(magic_header), |_| ())(input)
}

fn wasm_version<'a>(input: &'a [u8]) -> IResult<&[u8], ()> {
    // Currently only supporting binary format version 1
    let magic_header = [0x01, 0x00, 0x00, 0x00];
    map(tag(magic_header), |_| ())(input)
}

impl Decode for Module {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        // Check for the magic header at the beginning of all Wasm modules
        let (input, _) = magic_header(input)?;

        // Check that it is a Wasm version we support
        let (_input, _) = wasm_version(input)?;

        Ok((&[], Self { _types: Vec::new() }))
    }
}

impl Module {
    pub fn new(bytes: &[u8]) -> Option<Self> {
        // TODO: Return result so that errors are explicit
        match Module::decode(bytes) {
            Ok((_, module)) => Some(module),
            Err(_) => None,
        }
    }
}
