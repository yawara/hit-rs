use crate::oid::Oid;
use std::collections::btree_map;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug)]
pub struct Mode(pub Vec<u8>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name(pub Vec<u8>);

#[derive(Debug, Clone, Copy)]
pub enum EntryKind {
    Tree,
    Blob,
}

#[derive(Debug)]
pub struct TreeEntry {
    oid: Oid,
    mode: Mode,
}

#[derive(Debug)]
pub struct Tree {
    entries: BTreeMap<Name, TreeEntry>,
}

impl TreeEntry {
    pub fn new(oid: Oid, mode: Mode) -> Self {
        Self { oid, mode }
    }

    pub fn kind(&self) -> EntryKind {
        if self.mode.0[0] == b'1' {
            EntryKind::Blob
        } else {
            EntryKind::Tree
        }
    }
}

impl Tree {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    pub fn append_entry(&mut self, name: Name, entry: TreeEntry) {
        self.entries.insert(name, entry);
    }
}

impl<'a> IntoIterator for &'a Tree {
    type Item = (&'a Name, &'a TreeEntry);
    type IntoIter = btree_map::Iter<'a, Name, TreeEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:>06}",
            std::str::from_utf8(&self.0)
                .unwrap()
                .parse::<usize>()
                .unwrap()
        )
    }
}

impl fmt::Display for EntryKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntryKind::Tree => write!(f, "tree"),
            EntryKind::Blob => write!(f, "blob"),
        }
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, entry) in self {
            writeln!(
                f,
                "{} {} {}    {}",
                entry.mode,
                entry.kind(),
                entry.oid,
                name
            )?;
        }
        Ok(())
    }
}
