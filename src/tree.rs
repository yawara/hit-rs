use crate::oid::Oid;

struct Node {
    id: Oid,
    name: String,
}

pub struct Tree {
    id: Oid,
    entries: Vec<Node>
}
