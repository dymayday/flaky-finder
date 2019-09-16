//! FlakyFinder builder pattern definition.

use crate::{cli::Cli, FlakyFinder};
use std::process::ExitStatus;

pub(crate) struct FlakyFinderBuilder {
    /// The actual command that we need to test
    cmd: String,
    /// The status of the process we are currently evaluating
    exit_status: Option<ExitStatus>,
    /// The output from the process we are evaluating: stdout/stderr
    output: Option<String>,
    /// Let's run those tests in parallel
    nb_threads: u32,
    /// How many times we should run the command.
    runs: u64,
}

impl std::default::Default for FlakyFinderBuilder {
    fn default() -> Self {
        Self {
            cmd: String::default(),
            exit_status: None,
            output: None,
            nb_threads: 1,
            runs: 10,
        }
    }
}

impl FlakyFinderBuilder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn from_cli() -> Self {
        let cli = Cli::new();
        Self {
            cmd: cli.cmd.clone(),
            nb_threads: cli.nb_threads,
            runs: cli.runs,
            ..Default::default()
        }
    }

    /// The actual command of the process we are evaluating.
    #[allow(dead_code)]
    pub(crate) fn cmd(&mut self, cmd: &str) -> &mut Self {
        self.cmd = cmd.to_string();
        self
    }

    /// The number of concurrent process used to evaluate the test harness.
    #[allow(dead_code)]
    pub(crate) fn nb_threads(&mut self, nb_cpu: u32) -> &mut Self {
        self.nb_threads = nb_cpu;
        self
    }

    /// Builds a [`FlakyFinder`].
    pub(crate) fn build(&self) -> FlakyFinder {
        FlakyFinder {
            cmd: self.cmd.clone(),
            exit_status: self.exit_status,
            output: self.output.clone(),
            nb_threads: self.nb_threads,
            runs: self.runs,
        }
    }
}
