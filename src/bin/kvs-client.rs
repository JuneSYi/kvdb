use std::{env, process::exit};
use std::io::{Result};
use clap::{Command, Arg};

use kvs::KvStore;

fn main() -> Result<()> {
    let this_dir = env::current_dir()?;
    let mut kvs = KvStore::open(&this_dir)?;
    let args = Command::new("kvs-client")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            Command::new("set")
                .arg(Arg::new("key").num_args(1))
                .arg(Arg::new("value").num_args(1))
        )
        .subcommand(
            Command::new("get")
                .arg(Arg::new("key").num_args(1))
        )
        .subcommand(
            Command::new("rm")
                .arg(Arg::new("key").num_args(1))
        )
        .get_matches();
    match args.subcommand() {
        Some(("set", sub_m)) => {
            let k = sub_m.get_one::<String>("key").expect("key argument not found");
            let v = sub_m.get_one::<String>("value").expect("value argument not found");
            kvs.set(k.to_string(), v.to_string())
        },
        Some(("get", sub_m)) => {
            let k = sub_m.get_one::<String>("key").expect("key not found");
            let result = kvs.get(k.to_string())?;
            if let Some(val) = result {
                println!("{val}");
                exit(0)
            } else {
                println!("Key not found");
                exit(0)    
            }
        },
        Some(("rm", sub_m)) => {
            let k = sub_m.get_one::<String>("key").expect("key not found");
            // kvs.remove(k.to_string())
            match kvs.remove(k.to_string()) {
                Ok(()) => {
                    exit(0)
                },
                Err(_e) => {
                    println!("Key not found");
                    exit(1)
                }
            }
        },
        _ => exit(1)
    }
}
