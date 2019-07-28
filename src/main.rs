extern crate hit;

use hit::index::*;
use std::io::BufReader;

fn main() {
    // let mut index = Index::new();
    // let index_entry = IndexEntry::new("src/lib.rs");
    // index.add_entry(index_entry);
    let mut f = std::fs::File::open(".git/index").unwrap();
    let mut reader = BufReader::new(f);
    let index = Index::from_reader(reader);
    println!("{:?}", &index);
}
