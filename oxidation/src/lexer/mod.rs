//! # Oxidation -- Lua scripting language written in Rust
//!
//! # Lexer - Transforms source text into a token stream
//!
//! # Examples
//!


pub mod reader;
pub mod result;


#[cfg(test)]
mod tests {
  use std::error::Error;
  use lexer::reader::Reader;
  use lexer::reader::FileReader;
  use parser::tokens::Token;

  fn lex(filename : &str) {
    let mut f = FileReader::new(filename)
                .ok()
                .expect(&format!("Unable to open {} for test", filename)[0..]);

    loop {
      let tok = f.next_token();
      match tok {
        Ok(Token::EOF) => break,
        Ok(t) => println!("{:?}", t),
        Err(e) => { panic!("{}", e.description()) }
      }
    }
  }

  #[test]
  fn hello() {
    lex("hello.lua")
  }

  /*#[test]
  fn life() {
    lex("life.lua")
  }*/
}
