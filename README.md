[![Build status](https://travis-ci.com/dymayday/flaky-finder.svg?branch=master)](https://travis-ci.com/dymayday/flaky-finder)


# flaky-finder
CLI tool meant to find flaky test by running multiple times a test harness until it fails


## Install

Both techniques requires Rust and Cargo that can be isntall following [this](https://doc.rust-lang.org/cargo/getting-started/installation.html<Paste>).

Using **Cargo**:

```bash
cargo install flaky-finder
```

Or by compiling it, but you will need to use the binary in `target/release/flaky-finder`:

```bash
git clone https://github.com/dymayday/flaky-finder.git && cd flaky-finder
cargo build --release
```


## Example

To run 100 times a test with cargo to discover a potential flakyness, we can run this:
```bash
flaky-finder -j2 -r100 "cargo test -- --nocapture release_test"
```

## ToDo

- Add ability to run process in multiple threads.
- Add better documentation.
