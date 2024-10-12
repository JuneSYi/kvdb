use kvdb::kvstore::kvstore::KvStore;

fn main() {
    println!("inside src/bin/kvdb.rs");
    let a = KvStore::new("one".to_string(),"clown".to_string());
    let val = a.get_key();
    println!("{:?}", val);
    println!("{:?}", a);
}


