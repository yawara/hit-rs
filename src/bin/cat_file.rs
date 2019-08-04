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
    let tree = object.into_tree().unwrap();
    println!("{:?}", tree);
    // let blob = object.into_blob().unwrap();
    // print!("{}", blob.as_str());
}
