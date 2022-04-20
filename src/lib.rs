pub use module::Module;
use nom::IResult;

// TODO: Using command cargo modules generate tree --lib --with-types develop a better module tree structure
// For example should I actually have a parent module called decode/parse that does all that work? Where does validation live?
mod instructions;
mod module;

/// TODO: Document
trait Decode
where
    Self: Sized,
{
    /// Decode the given bytes into the type Self
    fn decode<'a>(input: &'a [u8]) -> IResult<&[u8], Self, nom::error::Error<&'a [u8]>>;
}

impl Module {
    /// TODO: Document
    pub fn new(bytes: &[u8]) -> Option<Self> {
        // TODO: Return result so that errors are explicit
        match Module::decode(bytes) {
            Ok((_, module)) => Some(module),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_module_new() {
        let bytes = include_bytes!("../examples/module.wasm");
        assert!(Module::new(bytes).is_some());
    }
}
