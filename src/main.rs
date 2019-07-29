extern crate hit;

use hit::index::*;
use std::io::BufReader;
use std::io::Write;

fn main() {
    // let mut index = Index::new();
    // let index_entry = IndexEntry::new("src/lib.rs");
    // index.add_entry(index_entry);
    let mut f = std::fs::File::open(".git/index").unwrap();
    let mut reader = BufReader::new(f);
    let mut index = Index::from_reader(reader);
    println!("{:?}", &index);
    let entry = IndexEntry::new("src/main.rs");
    index.add_entry(entry);
    let mut f = std::fs::File::create(".git/index").unwrap();
    f.write(&index.bytes());
}
