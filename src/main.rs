use clap::{App, Arg, ArgMatches, SubCommand};
use std::process;

mod mix_peer;
mod node;

fn main() {
    let arg_matches = App::new("Nym Mixnode")
        .version("0.1.0")
        .author("Nymtech")
        .about("Implementation of the Loopix-based Mixnode")
        .subcommand(
            SubCommand::with_name("run")
                .about("Starts the mixnode")
                .arg(
                    Arg::with_name("host")
                        .long("host")
                        .help("The custom host on which the mixnode will be running")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("port")
                        .long("port")
                        .help("The port on which the mixnode will be listening")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("layer")
                        .long("layer")
                        .help("The mixnet layer of this particular node")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("local")
                        .long("local")
                        .help("Flag to indicate whether the client is expected to run on a local deployment.")
                        .takes_value(false),
                ),
        )
        .get_matches();

    if let Err(e) = execute(arg_matches) {
        println!("{}", e);
        process::exit(1);
    }
}

fn execute(matches: ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("run", Some(m)) => Ok(node::runner::start(m)),
        _ => Err(usage()),
    }
}

fn usage() -> String {
    banner() + "usage: --help to see available options.\n\n"
}

fn banner() -> String {
    return r#"

      _ __  _   _ _ __ ___
     | '_ \| | | | '_ \ _ \
     | | | | |_| | | | | | |
     |_| |_|\__, |_| |_| |_|
            |___/

             (mixnode)

    "#
    .to_string();
}
