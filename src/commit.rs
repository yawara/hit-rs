use crate::object::Oid;
use crate::tree::Tree;

pub struct Commit {
    id: Oid,
    prev_commit: Option<Oid>,
    trees: Vec<Tree>
}
