use nom::{bytes::complete::tag, combinator::map, IResult};

/// A Wasm module
#[derive(Debug)]
pub struct Module<'a> {
    /// Raw bytes of a module
    bytes: &'a [u8],

    /// Module types
    _types: Vec<Type>,
}

/// A unique function signature
#[derive(Debug)]
struct Type {}

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

impl<'a> Module<'a> {
    // TODO: Bubble up errors with ?
    pub fn new(bytes: &'a [u8]) -> Result<Self, ()> {
        // In memory representation of module
        let m = Self {
            bytes,
            _types: Vec::new(),
        };

        let parse_result = || -> IResult<&'a [u8], ()> {
            let input = m.bytes;

            // Check for magic header
            let (input, _) = magic_header(input)?;

            // Check Wasm version
            let (_input, _) = wasm_version(input)?;

            Ok((&[], ()))
        }();

        match parse_result {
            Ok(_) => println!("Parsed well"),
            Err(err) => println!("{:?}", err),
        };

        Ok(m)
    }
}
