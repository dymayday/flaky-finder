//! Flaky-Finder custom error definition.

use std::{error::Error as StdError, fmt, io};

pub type FlakyFinderResult<T> = Result<T, FlakyFinderError>;

/// An error that can occur when interacting with the algorithm.
#[derive(Debug)]
pub struct FlakyFinderError(Box<ErrorKind>);

impl FlakyFinderError {
    /// A constructor for `FlakyFinderError`.
    pub fn new(kind: ErrorKind) -> Self {
        FlakyFinderError(Box::new(kind))
    }

    #[allow(dead_code)]
    /// Helper function to build a new error with an [Other](ErrorKind::Other) ErrorKind.
    pub fn new_other(s: &str) -> Self {
        FlakyFinderError::new(ErrorKind::Other(s.to_owned()))
    }

    #[allow(dead_code)]
    /// Return the specific type of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    #[allow(dead_code)]
    /// Unwrap this error into its underlying type.
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

/// The specific type of an error.
#[derive(Debug)]
pub enum ErrorKind {
    /// An I/O error that occurred while processing a data stream.
    Io(io::Error),
    /// Error occuring from None Option
    NoneError(std::option::NoneError),
    /// Error occuring while converting bytes to String.
    Utf8Error(std::str::Utf8Error),
    /// Yet undefined error.
    Other(String),
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

impl StdError for FlakyFinderError {
    /// The lower-level source of this error, if any.
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self.0 {
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::NoneError(ref _err) => None,
            ErrorKind::Utf8Error(ref err) => Some(err),
            ErrorKind::Other(ref _s) => None,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for FlakyFinderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Io(ref err) => err.fmt(f),
            ErrorKind::NoneError(ref _err) => write!(f, "None value encountered."),
            ErrorKind::Utf8Error(ref err) => err.fmt(f),
            ErrorKind::Other(ref s) => write!(f, "Unknown error encountered: '{}'.", s),
            _ => unreachable!(),
        }
    }
}

impl From<io::Error> for FlakyFinderError {
    fn from(err: io::Error) -> Self {
        FlakyFinderError::new(ErrorKind::Io(err))
    }
}

impl From<std::option::NoneError> for FlakyFinderError {
    fn from(err: std::option::NoneError) -> Self {
        FlakyFinderError::new(ErrorKind::NoneError(err))
    }
}

impl From<std::str::Utf8Error> for FlakyFinderError {
    fn from(err: std::str::Utf8Error) -> Self {
        FlakyFinderError::new(ErrorKind::Utf8Error(err))
    }
}
