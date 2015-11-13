use std::error::Error;
use std::fmt::{Display,Formatter};

#[derive(Debug)]
pub enum LexerError {
  IOError(::std::io::Error),
  GenericError(&'static str)
}

pub type LexerResult<T> = Result<T, LexerError>;

impl Error for LexerError {
  fn description(&self) -> &str {
    match *self {
      LexerError::IOError(ref e) => e.description(),
      LexerError::GenericError(ref s) => &s
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      LexerError::IOError(ref e) => Some(e),
      LexerError::GenericError(_) => None
    }
  }
}

impl From<::std::io::Error> for LexerError {
  fn from(err: ::std::io::Error) -> LexerError {
    LexerError::IOError(err)
  }
}

impl Display for LexerError {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), ::std::fmt::Error> {
    match *self {
      LexerError::IOError(ref e) => fmt.write_fmt(format_args!("{}", e)),
      LexerError::GenericError(ref s) => fmt.write_fmt(format_args!("{}", s))
    }
  }
}

