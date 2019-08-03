use crate::blob::Blob;
use crate::commit::Commit;
use crate::error::Result;
use crate::object::Object;
use crate::oid::Oid;
use crate::tree::Tree;

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

impl StandardOdb {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            objects: path.as_ref().to_path_buf(),
        }
    }
}

impl Odb for StandardOdb {
    fn get(&self, oid: &Oid) -> Result<Object> {
        let hex = oid.hex();
        let pre = &hex[0..2];
        let last = &hex[2..];
        let mut object_path = self.objects.clone();
        object_path.push(pre);
        object_path.push(last);
        let f = std::fs::File::open(object_path)?;
        let reader = BufReader::new(f);
        let mut decoder = ZlibDecoder::new(reader);
        let mut buf = Vec::new();
        decoder.read_to_end(&mut buf);
        let mut buf = &buf[..];
        let mut object_type = Vec::new();
        buf.read_until(b' ', &mut object_type)?;
        object_type.pop();
        let mut object_size = Vec::new();
        buf.read_until(0x00, &mut object_size)?;
        object_size.pop();
        let object_size = String::from_utf8(object_size)?;
        let object_size = object_size.parse::<usize>()?;
        let mut content = Vec::new();
        buf.read_to_end(&mut content);
        match &object_type[..] {
            b"blob" => Ok(Object::Blob(Blob::new(&content))),
            b"tree" => unimplemented!(),
            b"commit" => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}
