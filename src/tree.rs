use crate::oid::Oid;
use std::collections::HashMap;

pub struct Tree {
    nodes: HashMap<String, Oid>,
}
