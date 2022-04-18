use nom::{bytes::complete::tag, combinator::map, error::VerboseError, IResult};

/// A Wasm module
#[derive(Debug)]
pub struct Module<'a> {
    /// Raw bytes of a module
    bytes: &'a [u8],

    /// Module types
    types: Vec<Type>,
}

/// A unique function signature
#[derive(Debug)]
struct Type {}

fn magic_header<'a, E>(input: &'a [u8]) -> IResult<&[u8], (), E>
where
    E: nom::error::ParseError<&'a [u8]>,
{
    // The magic header that every Wasm module begins with
    let magic_header = [0x00, 0x61, 0x73, 0x6D];
    map(tag(magic_header), |_| ())(input)
}

impl<'a> Module<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        // In memory representation of module
        let m = Self {
            bytes,
            types: Vec::new(),
        };

        // Check for magic header
        magic_header::<VerboseError<&[u8]>>(&bytes);

        // // Check Wasm version
        // // TODO
        // pos += 4;

        // while pos < m.bytes.len() {
        //     // First byte is segment ID which marks the type of segment
        //     let id = 1; // TODO

        //     // Next four bytes are the size of the section
        //     let section_size = 1; // TODO

        //     // TODO: Consider marking start_pos before parsing section
        // }

        m
    }
}
