pub struct Blob {
    content: Vec<u8>,
}

impl Blob {
    pub fn new(content: &[u8]) -> Self {
        Self {
            content: content.to_vec()
        }
    }
}
