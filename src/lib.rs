use indices::{FuncIdx, GlobalIdx, MemIdx, TableIdx, TypeIdx};
use nom::{
    bytes::complete::{tag, take},
    combinator::{consumed, map},
    multi::many0,
    IResult,
};
use types::{FuncType, GlobalType, MemType, RefType, TableType, ValType};
use util::Decode;
use values::Name;

mod indices;
mod types;
mod util;
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

/// A component of a module record
#[derive(Debug, PartialEq)]
enum Section {
    /// Intended for use in debugging or third-party extensions
    CustomSection((Name, Vec<u8>)),
    /// Types found in the module
    TypeSection(Vec<FuncType>),
    /// Imports that are required for instantiation
    ImportSection(Vec<Import>),
    /// Correlation between functions and their respective types
    FunctionSection(Vec<TypeIdx>),
    /// Tables for indirection
    TableSection(Vec<Table>),
    /// Linear memories
    MemorySection(Vec<Memory>),
    /// Globally accessible variables
    GlobalSection(Vec<Global>),
    /// Exports accessible to the host environment
    ExportSection(Vec<Export>),
    /// The index of a start function that is automatically invoked
    StartSection(Option<FuncIdx>),
    /// Elements that can be used to initialize tables
    ElementSection(Vec<Element>),
    /// The actual instructions to be executed
    CodeSection(Vec<Code>),
    /// Data segments that can be used to initialize memory
    DataSection(Vec<Data>),
    /// The number of data segments in the data section
    DataCountSection(Option<u32>),
}

impl Decode for Section {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, id) = u8::decode(input)?;
        let (input, size) = u32::decode(input)?;

        match id {
            0 => {
                // Custom section
                let (input, (consumed, name)) = consumed(Name::decode)(input)?;
                let (input, data) = take(size as usize - consumed.len())(input)?;
                Ok((input, Section::CustomSection((name, data.to_owned()))))
            }
            1 => {
                // Type section
                map(Vec::<FuncType>::decode, |types| Section::TypeSection(types))(input)
            }
            2 => {
                // Import section
                map(Vec::<Import>::decode, |imports| {
                    Section::ImportSection(imports)
                })(input)
            }
            3 => {
                // Func section
                map(Vec::<TypeIdx>::decode, |functions| {
                    Section::FunctionSection(functions)
                })(input)
            }
            4 => {
                // Table section
                map(Vec::<Table>::decode, |tables| Section::TableSection(tables))(input)
            }
            5 => {
                // Memory section
                map(Vec::<Memory>::decode, |memories| {
                    Section::MemorySection(memories)
                })(input)
            }
            6 => {
                // Global section
                map(Vec::<Global>::decode, |globals| {
                    Section::GlobalSection(globals)
                })(input)
            }
            7 => {
                // Export section
                map(Vec::<Export>::decode, |exports| {
                    Section::ExportSection(exports)
                })(input)
            }
            8 => {
                // Start section
                map(Option::<FuncIdx>::decode, |start| {
                    Section::StartSection(start)
                })(input)
            }
            9 => {
                // Element section
                map(Vec::<Element>::decode, |elements| {
                    Section::ElementSection(elements)
                })(input)
            }
            10 => {
                // Code section
                map(Vec::<Code>::decode, |code| Section::CodeSection(code))(input)
            }
            11 => {
                // Data section
                map(Vec::<Data>::decode, |data| Section::DataSection(data))(input)
            }
            12 => {
                // Data section
                map(Option::<u32>::decode, |data_count| {
                    Section::DataCountSection(data_count)
                })(input)
            }
            _ => unreachable!(), // TODO: This should probably actually throw a parse error rather than panic
        }
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Import {
    module: Name,
    name: Name,
    descriptor: ImportDescriptor,
}

impl Decode for Import {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
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
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Table {
    tt: TableType,
}

impl Decode for Table {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Memory {
    mt: MemType,
}

impl Decode for Memory {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Global {
    gt: GlobalType,
    init: Expression,
}

impl Decode for Global {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Expression(Vec<Instruction>);

impl Decode for Expression {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
enum Instruction {}

impl Decode for Instruction {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Export {
    nm: Name,
    d: ExportDescriptor,
}

impl Decode for Export {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub enum ExportDescriptor {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

impl Decode for ExportDescriptor {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub enum ElementKind {
    FuncRef,
}

impl Decode for ElementKind {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub enum Element {
    ActiveIndex(Expression, Vec<FuncIdx>),
    PassiveIndex(ElementKind, Vec<FuncIdx>),
    ActiveExplicitIndex(TableIdx, Expression, ElementKind, Vec<FuncIdx>),
    DeclarativeIndex(ElementKind, Vec<FuncIdx>),
    ActiveExpression(Expression, Vec<Expression>),
    PassiveExpression(RefType, Vec<Expression>),
    ActiveExplicitExpression(TableIdx, Expression, RefType, Vec<Expression>),
    DeclarativeExpression(RefType, Vec<Expression>),
}

impl Decode for Element {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Code {
    pub size: u32,
    pub code: Func,
}

impl Decode for Code {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Func {
    pub locals: Vec<Local>,
    pub body: Expression,
}

impl Decode for Func {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub struct Local {
    pub count: u32,
    pub value_type: ValType,
}

impl Decode for Local {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub enum Data {
    Active(Expression, Vec<u8>),
    Passive(Vec<u8>),
    ActiveExplicit(MemIdx, Expression, Vec<u8>),
}

impl Decode for Data {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
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

    // #[test]
    // fn test_section() {
    //     // assert_eq!(Section::decode(&[0x00]), Ok((EMPTY, Section::CustomSection(()))));
    //     assert_eq!(
    //         Section::decode(&[
    //             0x01, 0x88, 0x80, 0x80, 0x80, 0x00, 0x02, 0x60, 0x01, 0x7F, 0x00, 0x60, 0x00, 0x00
    //         ]),
    //         Ok((
    //             EMPTY,
    //             Section::TypeSection(vec!(
    //                 FuncType {
    //                     rt1: vec!(ValType::NumType(NumType::I32)),
    //                     rt2: vec!()
    //                 },
    //                 FuncType {
    //                     rt1: vec!(),
    //                     rt2: vec!()
    //                 }
    //             ))
    //         ))
    //     );
    // }
}
