use crate::oid::Oid;

pub struct Commit {
    parents: Vec<Oid>,
    tree: Oid,
}
