# flaky-finder
CLI tool met to find flaky test by running multiple times a test harness until it fails

## Example

To run 100 times a test with cargo to discover a potential flakyness, we can run this:
```bash
flaky-finder -j2 -r100 "cargo test -- --nocapture release_test"
```
