use nom::bytes::complete::tag;

/// A Wasm module
#[derive(Debug)]
pub struct Module {
    /// Raw bytes of a module
    bytes: Vec<u8>,

    /// Module types
    types: Vec<Type>,
}

/// A unique function signature
#[derive(Debug)]
struct Type {}

/// The magic header that every Wasm module begins with
const MAGIC_HEADER: &[u8] = &[0x6d, 0x73, 0x61, 0x00];

impl Module {
    pub fn new(bytes: Vec<u8>) -> Self {
        // In memory representation of module
        let m = Self {
            bytes,
            types: Vec::new(),
        };

        // Track position as we decode bytes
        let mut pos: usize = 0;

        // Check for magic header
        // TODO
        tag(MAGIC_HEADER)(m.bytes)
        pos += 4;

        // Check Wasm version
        // TODO
        pos += 4;

        while (pos < m.bytes.len()) {
            // First byte is segment ID which marks the type of segment
            let id = 1; // TODO

            // Next four bytes are the size of the section
            let section_size = 1; // TODO

            // TODO: Consider marking start_pos before parsing section
        }

        m
    }
}
