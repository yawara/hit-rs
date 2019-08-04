use crate::oid::Oid;
use chrono::offset::FixedOffset;
use chrono::DateTime;
use chrono::TimeZone;
use std::fmt;

#[derive(Debug)]
pub struct Identity {
    name: Vec<u8>,
    email: Vec<u8>,
    datetime: DateTime<FixedOffset>,
}

impl Identity {
    pub fn new(name: Vec<u8>, email: Vec<u8>, datetime: DateTime<FixedOffset>) -> Self {
        Self {
            name,
            email,
            datetime,
        }
    }
}

#[derive(Debug)]
pub struct Commit {
    tree: Oid,
    parents: Vec<Oid>,
    author: Identity,
    committer: Identity,
    message: Vec<u8>,
}

impl Commit {
    pub fn new(
        tree: Oid,
        parents: Vec<Oid>,
        author: Identity,
        committer: Identity,
        message: Vec<u8>,
    ) -> Self {
        Self {
            tree: tree,
            parents: parents,
            author: author,
            committer: committer,
            message: message,
        }
    }
}

impl fmt::Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} <{}> {}",
            std::str::from_utf8(&self.name).unwrap(),
            std::str::from_utf8(&self.email).unwrap(),
            self.datetime
        )
    }
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "tree {}", self.tree)?;
        for parent in &self.parents {
            writeln!(f, "parent {}", parent)?;
        }
        writeln!(f, "author {}", self.author)?;
        writeln!(f, "comitter {}", self.committer)?;
        write!(f, "{}", std::str::from_utf8(&self.message).unwrap())
    }
}
