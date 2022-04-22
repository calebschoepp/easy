use nom::{
    bytes::complete::take,
    combinator::{consumed, map},
    IResult,
};

use crate::Decode;

use super::{
    code::Code,
    data::Data,
    element::Element,
    export::Export,
    global::Global,
    import::Import,
    indices::{FuncIdx, TypeIdx},
    memory::Memory,
    table::Table,
    types::FuncType,
    values::Name,
};

/// A component of a module record
#[derive(Debug, PartialEq)]
pub enum Section {
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

#[cfg(test)]
mod tests {
    use crate::module::{
        import::ImportDescriptor,
        types::{Limits, MemType, NumType, RefType, TableType, ValType},
    };

    use super::*;

    const EMPTY: &[u8] = &[];

    #[test]
    fn test_custom_section() {
        let input = &[0x00, 0x05, 0x01, 0xAA, 0x01, 0x02, 0x03];
        let section = Section::decode(input);
        assert_eq!(
            section,
            Ok((
                EMPTY,
                Section::CustomSection((Name(vec!(0xAA)), vec!(0x01, 0x02, 0x03)))
            ))
        )
    }

    #[test]
    fn test_type_section() {
        let input = &[0x01, 0x6, 0x01, 0x60, 0x01, 0x7F, 0x01, 0x7F];
        let section = Section::decode(input);
        assert_eq!(
            section,
            Ok((
                EMPTY,
                Section::TypeSection(vec!(FuncType {
                    rt1: vec!(ValType::NumType(NumType::I32)),
                    rt2: vec!(ValType::NumType(NumType::I32)),
                }))
            ))
        )
    }

    #[test]
    fn test_import_section() {
        let input = &[0x02, 0x07, 0x01, 0x01, 0xAA, 0x01, 0xAB, 0x00, 0x00];
        let section = Section::decode(input);
        assert_eq!(
            section,
            Ok((
                EMPTY,
                Section::ImportSection(vec!(Import {
                    module: Name(vec!(0xAA)),
                    name: Name(vec!(0xAB)),
                    descriptor: ImportDescriptor::Func(0)
                }))
            ))
        )
    }

    #[test]
    fn test_function_section() {
        let input = &[0x03, 0x03, 0x02, 0x00, 0x01];
        let section = Section::decode(input);
        assert_eq!(section, Ok((EMPTY, Section::FunctionSection(vec!(0, 1)))))
    }

    #[test]
    fn test_table_section() {
        let input = &[0x04, 0x5, 0x01, 0x70, 0x01, 0x00, 0x01];
        let section = Section::decode(input);
        assert_eq!(
            section,
            Ok((
                EMPTY,
                Section::TableSection(vec!(Table {
                    tt: TableType {
                        lim: Limits {
                            min: 0,
                            max: Some(1),
                        },
                        et: RefType::FuncRef
                    }
                }))
            ))
        )
    }

    #[test]
    fn test_memory_section() {
        let input = &[0x05, 0x03, 0x01, 0x00, 0x00];
        let section = Section::decode(input);
        assert_eq!(
            section,
            Ok((
                EMPTY,
                Section::MemorySection(vec!(Memory {
                    mt: MemType {
                        lim: Limits { min: 0, max: None }
                    }
                }))
            ))
        )
    }

    // TODO: Write tests
}
