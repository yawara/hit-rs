extern crate hit;

use hit::index::*;
use std::env;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // let mut index = Index::new();
    // let index_entry = IndexEntry::new("src/lib.rs");
    // index.add_entry(index_entry);
    let args: Vec<String> = env::args().collect();
    let repo_root = &args[1];
    let mut git_index = PathBuf::from(repo_root);
    git_index.push(".git/index");
    let mut f = std::fs::File::open(git_index).unwrap();
    let mut reader = BufReader::new(f);
    let mut index = Index::from_reader(reader).unwrap();
    println!("{:?}", &index);
    let entry = IndexEntry::new(&args[2]);
    index.add_entry(entry);
    let f = std::fs::File::create(".git/index").unwrap();
    index.write(f).unwrap();
}
