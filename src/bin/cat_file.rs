extern crate hit;

use hit::odb::Odb;
use hit::odb::StandardOdb;
use hit::oid::Oid;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let odb = StandardOdb::from_path(".git/objects");
    let oid = Oid::from_hex(&args[1]);
    let object = odb.get(&oid).unwrap();
    match object.as_blob() {
        Some(blob) => {
            println!("{}", blob.as_str());
            return;
        }
        None => (),
    }
    match object.as_tree() {
        Some(tree) => {
            print!("{}", tree);
            return;
        }
        None => (),
    }
    match object.as_commit() {
        Some(commit) => {
            print!("{}", commit);
            return;
        }
        None => (),
    }
}
