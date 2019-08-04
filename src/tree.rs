use crate::oid::Oid;
use std::collections::HashMap;
use std::fmt;

pub struct Mode(pub Vec<u8>);

#[derive(PartialEq, Eq, Hash)]
pub struct Name(pub Vec<u8>);

#[derive(Debug)]
pub struct TreeEntry {
    oid: Oid,
    mode: Mode,
}

#[derive(Debug)]
pub struct Tree {
    entries: HashMap<Name, TreeEntry>,
}

impl TreeEntry {
    pub fn new(oid: Oid, mode: Mode) -> Self {
        Self { oid, mode }
    }
}

impl Tree {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn append_entry(&mut self, name: Name, entry: TreeEntry) {
        self.entries.insert(name, entry);
    }
}

impl fmt::Debug for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}
