use crate::blob::Blob;
use crate::commit::Commit;
use crate::error::Result;
use crate::object::Object;
use crate::oid::Oid;
use crate::tree::{Mode, Name, Tree, TreeEntry};

use flate2::bufread::ZlibDecoder;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

pub trait Odb {
    fn get(&self, oid: &Oid) -> Result<Object>;
}

pub struct StandardOdb {
    objects: PathBuf,
}

#[derive(Debug, Clone, Copy)]
enum ObjectType {
    Blob,
    Commit,
    Tree,
}

struct ObjectHeader {
    pub object_type: ObjectType,
    pub object_size: usize,
}

impl ObjectType {
    fn new<T: AsRef<[u8]>>(object_type: T) -> Self {
        match object_type.as_ref() {
            b"blob" => ObjectType::Blob,
            b"commit" => ObjectType::Commit,
            b"tree" => ObjectType::Tree,
            _ => panic!(),
        }
    }
}

impl ObjectHeader {
    fn read<R: BufRead>(mut reader: R) -> Self {
        let mut type_vec = Vec::new();
        reader.read_until(b' ', &mut type_vec);
        type_vec.pop();
        let object_type = ObjectType::new(&type_vec);

        let mut size_vec = Vec::new();
        reader.read_until(0x00, &mut size_vec);
        size_vec.pop();
        let object_size: usize = std::str::from_utf8(&size_vec).unwrap().parse().unwrap();
        Self {
            object_type,
            object_size,
        }
    }
}

impl StandardOdb {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            objects: path.as_ref().to_path_buf(),
        }
    }

    fn object_path(&self, oid: &Oid) -> PathBuf {
        let hex = oid.hex();
        let pre = &hex[0..2];
        let last = &hex[2..];
        let mut object_path = self.objects.clone();
        object_path.push(pre);
        object_path.push(last);
        object_path
    }

    fn decompress<R: BufRead>(reader: R) -> Vec<u8> {
        let mut decoder = ZlibDecoder::new(reader);
        let mut buf = Vec::new();
        decoder.read_to_end(&mut buf).unwrap();
        buf
    }

    fn read_object<R: BufRead>(mut reader: R) -> Object {
        let header = ObjectHeader::read(&mut reader);
        match header.object_type {
            ObjectType::Blob => Object::Blob(Self::read_blob(&mut reader, header.object_size)),
            ObjectType::Commit => Object::Commit(Self::read_commit(&mut reader)),
            ObjectType::Tree => Object::Tree(Self::read_tree(&mut reader)),
        }
    }

    fn read_blob<R: Read>(mut reader: R, object_size: usize) -> Blob {
        let mut buf = Vec::new();
        let object_size = reader.read_to_end(&mut buf).unwrap();
        Blob::new(&buf)
    }

    fn read_tree<R: BufRead>(mut reader: R) -> Tree {
        let mut tree = Tree::new();
        loop {
            let mut mode = Vec::new();
            let mut name = Vec::new();
            let mode_size = reader.read_until(b' ', &mut mode).unwrap();
            if mode_size == 0 {
                break;
            }
            reader.read_until(0x00, &mut name);
            let oid = Oid::from_reader(&mut reader).unwrap();
            let entry = TreeEntry::new(oid, Mode(mode));
            tree.append_entry(Name(name), entry);
        }
        tree
    }

    fn read_commit<R: Read>(mut reader: R) -> Commit {
        unimplemented!()
    }
}

impl Odb for StandardOdb {
    fn get(&self, oid: &Oid) -> Result<Object> {
        let object_path = self.object_path(oid);
        let f = std::fs::File::open(object_path)?;
        let reader = BufReader::new(f);
        let decompressed: &[u8] = &Self::decompress(reader);
        Ok(Self::read_object(decompressed))
    }
}
