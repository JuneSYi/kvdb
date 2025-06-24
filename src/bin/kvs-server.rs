use std::{env, io::Result, process::exit};
use clap::{Command, Arg};

fn main() -> Result<()> {
    let args = Command::new("kvs-server")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("engine")
                .required(false)
                .long("engine")
                .value_parser(["kvs", "sled"])
                .help("The storage engine to use"),
        )
        .arg(
            Arg::new("addr")
                .required(false)
                .long("addr")
                .default_value("127.0.0.1:4000")
                .help("The address to listen on"),
        )
        .get_matches();
    let current_dir = env::current_dir()?;
    let engine_file_path = current_dir.join(".engine");
    let cli_engine = args.get_one::<String>("engine").map(|s| s.as_str());
    let previous_engine = std::fs::read_to_string(&engine_file_path).ok();

    let engine_to_run = match (cli_engine, previous_engine.as_deref()) {
        (Some(c), Some(p)) if c != p => { // if current != previous engine, error
            eprintln!("Wrong engine: current specified engine was '{}' but prior engine specified was '{}'", p, c);
            exit(1);
        }
        (Some(c), _) => c, // if specified, and no prior
        (None, Some(p)) => p, // if none specified, defaults to previous
        (None, None) => "kvs", // Default
    };

    if !engine_file_path.exists() {
        std::fs::write(&engine_file_path, engine_to_run)?;
    }
    let addr = args.get_one::<String>("addr").unwrap();
    eprintln!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    eprintln!("Storage engine: {}", engine_to_run);
    eprintln!("Listening on {}", addr);
    Ok(())
}
