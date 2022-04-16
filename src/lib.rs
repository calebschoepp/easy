pub struct Module {
    bytes: Vec<u8>,
}

impl Module {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}
