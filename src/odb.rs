use crate::error::Result;
use std::path::Path;

pub struct Odb {}

impl Odb {
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self> {
        unimplemented!()
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<()> {
        unimplemented!()
    }
}
