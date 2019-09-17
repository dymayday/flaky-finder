#![feature(try_trait)]
use builder::FlakyFinderBuilder;
use crossbeam_channel;
use error::FlakyFinderResult;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use std::{
    io::Write,
    process::{Command, ExitStatus},
};
use threadpool;

mod builder;
mod cli;
mod error;

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
    pub(crate) fn run(&self) -> FlakyFinderResult<()> {
        // println!("param = {:#?}", self);

        // Provide a custom bar style
        let pb = ProgressBar::new(self.runs);
        pb.set_style(ProgressStyle::default_bar().template(
            "{spinner:.cyan} [{elapsed_precise}] [{bar:40.white/gray}] ({pos}/{len}, ETA {eta})",
        ));

        for _ in (0..self.runs).progress_with(pb) {
            let output = Command::new("sh")
                .arg("-c")
                .arg(self.cmd.clone())
                .output()
                .expect("Fail to run command process.");

            let status = output.status;
            if !status.success() {
                std::io::stdout().write_all(&output.stdout)?;
                std::io::stderr().write_all(&output.stderr)?;
                break;
            }
        }
        Ok(())
    }

    /// Runs a command multiple time trying to find if it can fail at some point.
    pub(crate) fn par_run(cmd: &str, nb_threads: u32, runs: u64) -> FlakyFinderResult<()> {
        // Provide a custom bar style
        let pb = ProgressBar::new(runs);
        pb.set_style(ProgressStyle::default_bar().template(
            "{spinner:.cyan} [{elapsed_precise}] [{bar:40.white/gray}] ({pos}/{len}, ETA {eta})",
        ));

        let (sx, rx) = crossbeam_channel::bounded(runs as usize);

        let cmd = std::sync::Arc::new(cmd.to_string());

        // Execute the process at least one time in order to single process the compilation
        print!(">> Compiling...");
        ::std::io::stdout().flush()?;
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd.to_string())
            .output()
            .expect("Fail to run command process.");
        sx.send(output)
            .expect("Fail to send Command's output to channel.");
        println!("done.");

        let pool = threadpool::ThreadPool::new(nb_threads as usize);

        for _ in 0..runs - 1 {
            let cmd = cmd.clone();
            let sx = sx.clone();

            pool.execute(move || {
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(cmd.to_string())
                    .output()
                    .expect("Fail to run command process.");

                sx.send(output)
                    .expect("Fail to send Command's output to channel.");
            });
        }

        drop(sx);

        for recv_output in rx.iter().progress_with(pb) {
            let status = recv_output.status;
            if !status.success() {
                std::io::stdout().write_all(&recv_output.stdout)?;
                std::io::stderr().write_all(&recv_output.stderr)?;
                break;
            }
        }
        drop(rx);

        Ok(())
    }
}

fn main() {
    let ff = FlakyFinderBuilder::from_cli().build();

    if ff.runs < 1 {
        panic!("Number of 'runs' has to be > 0.")
    }

    if ff.nb_threads > 1 {
        FlakyFinder::par_run(&ff.cmd, ff.nb_threads, ff.runs)
            .expect("Fail to run processes in parallel.");
    } else {
        ff.run().expect("Fail to processes.");
    }
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
