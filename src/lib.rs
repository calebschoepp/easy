pub use module::Module;
use nom::IResult;

// TODO: Using command cargo modules generate tree --lib --with-types develop a better module tree structure
// For example should I actually have a parent module called decode/parse that does all that work? Where does validation live?
mod instructions;
mod module;

/// Trait that allows a type to decode itself from a sequence of bytes using nom
trait Decode
where
    Self: Sized,
{
    /// Decode the given bytes into the type Self
    fn decode<'a>(input: &'a [u8]) -> IResult<&[u8], Self, nom::error::Error<&'a [u8]>>;
}

/// Trait that allows you to create a new module
pub trait New<T>
where
    Self: Sized,
{
    // TODO: Return result so that errors are explicit
    /// Create a new module
    fn new(input: T) -> Option<Module>;
}

impl New<&[u8]> for Module {
    /// Create a new module from a sequence of bytes
    fn new(bytes: &[u8]) -> Option<Module> {
        match Module::decode(bytes) {
            Ok((_, module)) => Some(module),
            Err(_) => None,
        }
    }
}

impl New<&str> for Module {
    /// Create a new module from a sequence of characters
    fn new(_chars: &str) -> Option<Self> {
        todo!()
    }
}

impl Module {
    /// TODO: Document
    pub fn validate(&self) -> Result<(), &'static str> {
        // TODO: Improve error return type
        // TODO: Implement
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_module_from_bytes() {
        let bytes: &[u8] = include_bytes!("../examples/module.wasm");
        assert!(Module::new(bytes).is_some());
    }

    #[test]
    #[should_panic]
    fn test_module_from_str() {
        let str = "(module)";
        Module::new(str);
    }
}
