use nom::{bytes::complete::tag, IResult};
use types::FuncType;
use util::Decode;

mod types;
mod util;
mod values;

/// A Wasm module
#[derive(Debug)]
pub struct Module {
    /// Module types
    _types: Vec<FuncType>,
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

fn magic_header<'a>(input: &'a [u8]) -> IResult<&[u8], &[u8]> {
    // The magic header that every Wasm module begins with
    let magic_header = [0x00, 0x61, 0x73, 0x6D];
    tag(magic_header)(input)
}

fn wasm_version<'a>(input: &'a [u8]) -> IResult<&[u8], &[u8]> {
    // Currently only supporting binary format version 1
    let magic_header = [0x01, 0x00, 0x00, 0x00];
    tag(magic_header)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    #[test]
    fn test_module_decode() {
        let bytes = include_bytes!("../examples/module.wasm");
        assert!(Module::decode(bytes).is_ok());
    }

    #[test]
    fn test_module_new() {
        let bytes = include_bytes!("../examples/module.wasm");
        assert!(Module::new(bytes).is_some());
    }

    #[test]
    fn test_magic_header() {
        let magic: &[u8] = &[0x00, 0x61, 0x73, 0x6D];
        let not_magic: &[u8] = &[0x01, 0x61, 0x73, 0x6D];
        assert_eq!(magic_header(magic), Ok((EMPTY, magic)));
        assert!(magic_header(not_magic).is_err());
    }

    #[test]
    fn test_wasm_version() {
        let version: &[u8] = &[0x01, 0x00, 0x00, 0x00];
        let not_version: &[u8] = &[0x02, 0x00, 0x00, 0x00];
        assert_eq!(wasm_version(version), Ok((EMPTY, version)));
        assert!(wasm_version(not_version).is_err());
    }
}
