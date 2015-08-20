//! # Oxidation -- Lua scripting language written in Rust
//!
//! # Lexer - Transforms source text into a token stream
//!
//! # Examples
//!     TODO: ...

use parser::tokens::Token;
use lexer::reader::Reader;
use lexer::reader::FileReader;

pub mod reader;


#[cfg(test)]
mod tests {
  use super::*;
  use std::error::Error;
  use lexer::reader::Reader;
  use lexer::reader::FileReader;

  #[test]
  fn lex() {
    let mut f = FileReader::new("hello.lua")
                .ok()
                .expect("Unable to open file for test");

    loop {
      let tok = f.next_token();
      match tok {
        Ok(t) => println!("{:?}", t),
        Err(e) => {
          match e.description() {
            "EOF" => break,
            err => panic!("{}", err)
          } 
        }
      }
    }
  }
}
