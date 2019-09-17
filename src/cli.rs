//! CLI handler definition.

use std::env;
use clap::{App, Arg};

pub(crate) struct Cli {
    pub(crate) cmd: String,
    pub(crate) nb_threads: u32,
    pub(crate) runs: u64,
}

impl Cli {
    pub fn new() -> Self {
        // TODO: Use env instead here
        // let env_vars = env::vars();
        let matches = App::new(env::var("CARGO_PKG_NAME").unwrap())
            .version(&*format!("{}", env::var("CARGO_PKG_VERSION").unwrap()))
            .author("dymayday <dymayday@gmail.com>")
            .about("The app is looking for bug in the matrix.")
            .arg(
                Arg::with_name("cmd")
                    .required(true)
                    .help("The command of the process we want to evaluate."),
            )
            .arg(
                Arg::with_name("runs")
                    .short("r")
                    .default_value("10")
                    .help("Perform exactly NUM runs for the command."),
            )
            .arg(
                Arg::with_name("nb_threads")
                    .short("j")
                    .default_value("1")
                    .help("The number of threads we want to run test harness."),
            )
            .get_matches();

        Self {
            cmd: matches
                .value_of("cmd")
                .expect("No command specified.")
                .to_owned(),
            nb_threads: matches
                .value_of("nb_threads")
                .unwrap_or("1")
                .parse::<u32>()
                .expect("Fail to cast number of threads argument."),
            runs: matches
                .value_of("runs")
                .unwrap_or("1")
                .parse::<u64>()
                .expect("Fail to cast number of threads argument."),
        }
    }
}
