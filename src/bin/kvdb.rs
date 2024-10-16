use kvdb::kvstore::kvstore::KvStore;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    arg_cmd: String,
    arg_key: String,
    arg_value: String,
}

fn main() {
    println!("inside src/bin/kvdb.rs");
    let args = Args::parse();
    let first = args.arg_key;
    let second = args.arg_value;
    println!("first: {:?}, second: {:?}", first, second);

    let mut store = KvStore::new();
    // let a = KvStore::new("one".to_string(),"clown".to_string());
    // let val = a.get_key();
    // println!("{:?}", val);
    // println!("{:?}", a);
}


