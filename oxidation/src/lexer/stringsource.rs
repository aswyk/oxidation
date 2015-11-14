use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use lexer::result::{LexerResult, LexerError};

pub trait StringSource {
  fn get_string(&mut self) -> LexerResult<String>;
}

impl StringSource for BufReader<File> {
  fn get_string(&mut self) -> LexerResult<String> {
    let mut s = String::new();
    let r = try!(self.read_line(&mut s));
    if r == 0 {
      Err(LexerError::EOF)
    } else {
      Ok(s)
    }
  }
}

pub struct LexString {
  source_lines : Vec<String>,
  current_line : usize
}

impl LexString {
  pub fn new(s: String) -> LexString {
    LexString {
      source_lines: s.split("\n").map(|x| x.to_string()).collect(),
      current_line: 0
    }
  }
}

impl StringSource for LexString {
  fn get_string(&mut self) -> LexerResult<String> {
    let i = self.current_line;
    self.current_line += 1;
    if i < self.source_lines.len() {
      Ok(self.source_lines[i].clone())
    } else {
      Err(LexerError::EOF)
    }
  }
}
