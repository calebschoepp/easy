pub use module::{util::Decode, Module};

// TODO: Using command cargo modules generate tree --lib --with-types develop a better module tree structure
// For example should I actually have a parent module called decode/parse that does all that work? Where does validation live?
mod instructions;
mod module;

impl Module {
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
