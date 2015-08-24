//! # Oxidation -- Lua scripting language written in Rust
//!
//! # Lexer - Transforms source text into a token stream
//!
//! # Examples
//! 


pub mod reader;


#[cfg(test)]
mod tests {
  use std::error::Error;
  use lexer::reader::Reader;
  use lexer::reader::FileReader;

  fn lex(filename : &str) {
    let mut f = FileReader::new(filename)
                .ok()
                .expect(&format!("Unable to open {} for test", filename)[0..]);

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

  #[test]
  fn hello() {
    lex("hello.lua")
  }

  #[test]
  fn life() {
    lex("life.lua")
  }
}
