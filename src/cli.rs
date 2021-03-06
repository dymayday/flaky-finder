//! CLI handler definition.

use crate::error::FlakyFinderResult;
use clap::{App, Arg};

pub(crate) struct Cli {
    pub(crate) cmd: String,
    pub(crate) nb_threads: u32,
    pub(crate) runs: u64,
    pub(crate) should_continue: bool,
    pub(crate) show_errors_as_summary: bool,
}

impl Cli {
    pub fn new() -> FlakyFinderResult<Self> {
        // TODO: Use env instead here
        let matches = App::new("Flaky-Finder")
            .version("0.2.22")
            .author("dymayday <dymayday@gmail.com>")
            .about("This app is looking for flakyness in tests in the matrix.")
            .arg(
                Arg::with_name(r#""cmd""#)
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
            .arg(
                Arg::with_name("continue")
                    .short("c")
                    .long("continue")
                    .help("Whether or not we want to stop at the fist error found."),
            )
            .arg(
                Arg::with_name("print")
                    .short("p")
                    .long("print-error-on-the-fly")
                    .help("Show errors as they arrive of show them as a summary at the end."),
            )
            .get_matches();

        Ok(Self {
            cmd: matches
                .value_of(r#""cmd""#)
                .expect("No command specified.")
                .to_owned(),
            nb_threads: matches
                .value_of("nb_threads")
                .unwrap_or("1")
                .parse::<u32>()
                .expect("Fail to cast 'number of threads' argument."),
            runs: matches
                .value_of("runs")
                .unwrap_or("1")
                .parse::<u64>()
                .expect("Fail to cast 'number of runs' argument."),
            should_continue: matches.is_present("continue"),
            show_errors_as_summary: !matches.is_present("print"),
        })
    }
}
