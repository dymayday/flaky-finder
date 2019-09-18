//! Parking several utility stuff here.

use crate::error::FlakyFinderResult;
use std::io::{stderr, stdout, Write};

/// Print a vector of byte as String and flush the stdout buffer.
pub fn fstdout(output: &[u8]) -> FlakyFinderResult<()> {
    stdout().write_all(output)?;
    stdout().flush()?;
    Ok(())
}

/// Print a vector of byte as String and flush the stderr buffer.
pub fn fstderr(output: &[u8]) -> FlakyFinderResult<()> {
    stderr().write_all(output)?;
    stderr().flush()?;
    Ok(())
}
