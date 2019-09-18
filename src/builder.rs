//! FlakyFinder builder pattern definition.

use crate::{cli::Cli, error::FlakyFinderResult, FlakyFinder};
use std::process::ExitStatus;

pub(crate) struct FlakyFinderBuilder {
    /// The actual command that we need to test
    cmd: String,
    /// The status of the process we are currently evaluating
    exit_status: Option<ExitStatus>,
    /// Let's run those tests in parallel
    nb_threads: u32,
    /// How many times we should run the command
    runs: u64,
    /// Shall we stop on the first flaky test found or continue
    should_continue: bool,
}

impl std::default::Default for FlakyFinderBuilder {
    fn default() -> Self {
        Self {
            cmd: String::default(),
            exit_status: None,
            nb_threads: 1,
            runs: 10,
            should_continue: false,
        }
    }
}

impl FlakyFinderBuilder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn from_cli() -> FlakyFinderResult<Self> {
        let cli = Cli::new()?;
        Ok(Self {
            cmd: cli.cmd.clone(),
            nb_threads: cli.nb_threads,
            runs: cli.runs,
            should_continue: cli.should_continue,
            ..Default::default()
        })
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
            outputs: Vec::new(),
            nb_threads: self.nb_threads,
            runs: self.runs,
            should_continue: self.should_continue,
        }
    }
}
