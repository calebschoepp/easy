use nom::{bytes::complete::tag, multi::many0, IResult};

use crate::Decode;

use self::{
    code::Code,
    data::Data,
    element::Element,
    export::Export,
    global::Global,
    import::Import,
    indices::{FuncIdx, TypeIdx},
    memory::Memory,
    section::Section,
    table::Table,
    types::FuncType,
};

mod code;
mod data;
mod element;
mod export;
mod global;
mod import;
pub mod indices;
mod memory;
mod section;
mod table;
pub mod types;
mod values;

/// A Wasm module
#[derive(Debug, PartialEq)]
pub struct Module {
    types: Vec<FuncType>,
    imports: Vec<Import>,
    functions: Vec<TypeIdx>, // TODO: Does this need to be a more robust type for execution?
    tables: Vec<Table>,
    memories: Vec<Memory>,
    globals: Vec<Global>,
    exports: Vec<Export>,
    start: Option<FuncIdx>,
    elements: Vec<Element>,
    code: Vec<Code>,
    data: Vec<Data>,
    data_count: Option<u32>,
}

impl Decode for Module {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        // Check for the magic header at the beginning of all Wasm modules
        let (input, _) = magic_header(input)?;

        // Check that it is a Wasm version we support
        let (input, _) = wasm_version(input)?;

        // Decode an arbitrary number of sections
        let (input, sections) = many0(Section::decode)(input)?;

        // Create an empty module that we can populate
        let mut module = Self {
            types: Vec::new(),
            imports: Vec::new(),
            functions: Vec::new(),
            tables: Vec::new(),
            memories: Vec::new(),
            globals: Vec::new(),
            exports: Vec::new(),
            start: None,
            elements: Vec::new(),
            code: Vec::new(),
            data: Vec::new(),
            data_count: None,
        };

        // Build up a module based on the sections we've decoded
        sections.into_iter().for_each(|section| match section {
            Section::CustomSection(_) => (),
            Section::TypeSection(types) => module.types = types,
            Section::ImportSection(imports) => module.imports = imports,
            Section::FunctionSection(functions) => module.functions = functions,
            Section::TableSection(tables) => module.tables = tables,
            Section::MemorySection(memories) => module.memories = memories,
            Section::GlobalSection(globals) => module.globals = globals,
            Section::ExportSection(exports) => module.exports = exports,
            Section::StartSection(start) => module.start = start,
            Section::ElementSection(elements) => module.elements = elements,
            Section::CodeSection(code) => module.code = code,
            Section::DataSection(data) => module.data = data,
            Section::DataCountSection(data_count) => module.data_count = data_count,
        });

        // Return the decoded module
        Ok((input, module))
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
