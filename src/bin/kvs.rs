use std::process::exit;

use clap::{Command, Arg};

use kvs::KvStore;

fn main() {
    let mut kvs = KvStore::new();
    let args = Command::new("kvdb")
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
            kvs.set(k.to_string(), v.to_string());
            eprintln!("unimplemented");
            std::process::exit(1);
        },
        Some(("get", sub_m)) => {
            let k = sub_m.get_one::<String>("key").expect("key argument not found");
            kvs.get(k.to_string());
            eprintln!("unimplemented");
            std::process::exit(1);
        },
        Some(("rm", sub_m)) => {
            let k = sub_m.get_one::<String>("key").expect("key argument not found");
            kvs.remove(k.to_string());
            eprintln!("unimplemented");
            std::process::exit(1);
        },
        _ => exit(1)
    }
}


// fn main() {
//     let args = Command::new("kvdb")
//         .version(env!("CARGO_PKG_VERSION"))
//         // .arg(
//         //     Arg::new("vers")
//         //         .short('V')
//         //         .action(clap::ArgAction::Version)
//         // )
//         .arg(
//             Arg::new("set")
//                 .num_args(2)
//                 .action(clap::ArgAction::Append)
//         )
//         .arg(
//             Arg::new("get")
//                 .num_args(1)
//                 .action(clap::ArgAction::Set)
//         )
//         .arg(
//             Arg::new("rm")
//                 .num_args(1)
//                 .action(clap::ArgAction::Set)
//         )
//         .get_matches();
    
// }


// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     #[arg()]
//     get: String,

//     #[arg()]
//     set: String,

//     #[arg()]
//     rm: String,

// }

// fn main() {
//     let args = Args::parse();
    
// }