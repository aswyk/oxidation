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
  use std::io::Error;
  use lexer::reader::Reader;
  use lexer::reader::FileReader;

  #[test]
  fn lex() {
    let mut f = FileReader::new("hello.lua").ok().unwrap();

    loop {
      let tok = f.next_token();
      match tok {
        Ok(t) => println!("{:?}", t),
        Err(e) => {
          println!("{:?}", e);
          break;
        }
      }
    }
  }
}
