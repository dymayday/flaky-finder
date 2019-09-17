use builder::FlakyFinderBuilder;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use std::{
    io::Write,
    process::{Command, ExitStatus},
};

mod builder;
mod cli;

#[derive(Debug)]
pub(crate) struct FlakyFinder {
    /// The actual command that we need to test.
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

impl FlakyFinder {
    /// Runs a command multiple time trying to find if it can fail at some point.
    pub(crate) fn spawn(&mut self) -> Result<(), std::io::Error> {
        // println!("param = {:#?}", self);

        // Provide a custom bar style
        let pb = ProgressBar::new(self.runs);
        pb.set_style(ProgressStyle::default_bar().template(
            "{spinner:.cyan} [{elapsed_precise}] [{bar:40.white/gray}] ({pos}/{len}, ETA {eta})",
            // "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        ));

        // TODO: Run the tests in parallel
        for _ in (0..self.runs).progress_with(pb) {
            // std::thread::sleep(std::time::Duration::from_millis(100));

            let output = Command::new("sh")
                .arg("-c")
                .arg(self.cmd.clone())
                .output()
                .expect("Fail to run command process.");

            // let status = output.status().expect("Falt to get process status.");
            let status = output.status;
            if !status.success() {
                // std::io::stdout().write_all(&output.stdout).unwrap();
                std::io::stderr().write_all(&output.stderr).unwrap();
                break;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut ff = FlakyFinderBuilder::from_cli().build();
    ff.spawn().expect("Fail to spawn.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let _cmd = "cargo test -- --nocapture release_test";
        let cmd = "ls";
        let _ff = FlakyFinderBuilder::new().cmd(cmd).nb_threads(1).build();

        // assert_eq!(true, false);
    }
}
