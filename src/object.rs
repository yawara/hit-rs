use crate::commit::Commit;
use crate::tree::Tree;
use crate::blob::Blob;

pub enum Object {
    Commit(Commit),
    Tree(Tree),
    Blob(Blob)
}

