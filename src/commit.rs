use crate::oid::Oid;

#[derive(Debug)]
pub struct Commit {
    parents: Vec<Oid>,
    tree: Oid,
}
