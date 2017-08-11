extern crate clap;
extern crate env_logger;
extern crate ruma;

use clap::{App, AppSettings, Arg, SubCommand};

use ruma::config::Config;
use ruma::crypto::generate_macaroon_secret_key;
use ruma::server::Server;

fn main() {
    env_logger::init().expect("Failed to initialize logger.");

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs the Ruma server")
                .arg(Arg::with_name("config")
                     .short("c")
                     .long("config")
                     .value_name("PATH")
                     .help("Path to a configuration file")
                     .takes_value(true))
        )
        .subcommand(
            SubCommand::with_name("secret")
                .about("Generates a random value to be used as a macaroon secret key")
        )
        .get_matches();

    match matches.subcommand() {
        ("run", Some(submatches)) => {
            let config = match Config::from_file(submatches.value_of("config")) {
                Ok(config) => config,
                Err(error) => {
                    println!("Failed to load configuration file: {}", error);

                    return;
                }
            };

            match Server::new(&config) {
                Ok(server) => {
                    if let Err(error) = server.run() {
                        println!("{}", error);
                    }
                },
                Err(error) => {
                    println!("Failed to create server: {}", error);

                    return;
                }
            }
        }
        ("secret", Some(_)) => match generate_macaroon_secret_key() {
            Ok(key) => println!("{}", key),
            Err(error) => println!("Failed to generate macaroon secret key: {}", error),
        },
        _ => println!("{}", matches.usage()),
    };
}