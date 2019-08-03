pub struct Blob {
    content: Vec<u8>,
}

impl Blob {
    pub fn new(content: &[u8]) -> Self {
        Self {
            content: content.to_vec(),
        }
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.content).unwrap()
    }
}
