use std::fs;
use std::path::{Path, PathBuf};

use crate::odb::Odb;
use crate::error::Result;

pub struct Repository {}

impl Repository {
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self> {
        // ".git/objects"
        // ".git/objects/pack"
        // ".git/objects/info"
        // ".git/HEAD"
        // ".git/fooks"
        // ".git/refs"
        // ".git/refs/heads/"
        // ".git/refs/tags"
        
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        unimplemented!()
    }
}
