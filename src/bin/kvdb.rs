use kvdb::kvstore::kvstore::KvStore;
use clap::{Parser, Command, Arg, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    arg_cmd: String,
    arg_key: String,
    arg_value: Option<String>,
}

fn main() {
    let c = Command::new("kvdb")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new("set")
            .about("Set a string value of a string key")
                .arg(
                    Arg::new("key")
                        .help("The string key to set")
                        .required(true))
                .arg(
                    Arg::new("value")
                        .help("The string value to set")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("get")
                .about("Get a string value of a string key")
                .arg(
                    Arg::new("key")
                    .help("The key to get")
                    .required(true)
                )
        )
        .subcommand(
            Command::new("rm")
            .about("Remove a string value of a string key")
                .arg(
                    Arg::new("key")
                        .help("The key to remove")
                        .required(true)
                )
        )
        .get_matches();
}