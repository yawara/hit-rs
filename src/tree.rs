
use std::collections::HashMap;
use crate::oid::Oid;

pub struct Tree {
    nodes: HashMap<String, Oid>
}