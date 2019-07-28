use crate::oid::Oid;
use std::io::prelude::*;
use std::path::Path;

pub struct Blob {
    pub id: Oid,
    data: Vec<u8>,
}

impl Blob {
    pub fn new(content: &[u8]) -> Self {
        let content_size = content.len();
        let mut data = Vec::new();
        data.extend_from_slice(b"blob ");
        data.extend_from_slice(&content_size.to_string().into_bytes());
        data.push(0);
        data.extend_from_slice(content);
        Self {
            id: Oid::from_data(&data),
            data: data,
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let mut file = std::fs::File::open(path).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        Blob::new(&buf)
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn content(&self) -> &[u8] {
        let null = self.data.iter().position(|x| x == &0).unwrap();
        &self.data[null + 1..]
    }
}
