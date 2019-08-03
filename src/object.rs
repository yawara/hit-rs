use crate::blob::Blob;
use crate::commit::Commit;
use crate::tree::Tree;

pub enum Object {
    Commit(Commit),
    Tree(Tree),
    Blob(Blob),
}

impl Object {
    pub fn into_blob(self) -> Option<Blob> {
        match self {
            Object::Blob(blob) => Some(blob),
            _ => None,
        }
    }

    pub fn into_tree(self) -> Option<Tree> {
        match self {
            Object::Tree(tree) => Some(tree),
            _ => None,
        }
    }

    pub fn into_commit(self) -> Option<Commit> {
        match self {
            Object::Commit(commit) => Some(commit),
            _ => None,
        }
    }
}
