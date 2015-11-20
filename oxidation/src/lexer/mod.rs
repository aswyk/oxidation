//! # Oxidation -- Lua scripting language written in Rust
//!
//! # Lexer - Transforms source text into a token stream
//!
//! # Examples
//!


pub mod reader;
pub mod result;
pub mod stringsource;


#[cfg(test)]
mod tests {
  use std::error::Error;
  use std::fs::File;
  use std::path::Path;
  use std::io::BufReader;

  use lexer::reader::Reader;
  use lexer::reader::Lexer;
  use lexer::stringsource::LexString;
  use parser::tokens::TokenType;

  fn lex(filename: &str) {
    let f = File::open(Path::new(filename))
            .ok()
            .expect(&format!("Unable to open {} for test", filename)[0..]);

    let b = Box::new(BufReader::new(f));

    let mut f = Lexer::new(filename, b)
                .ok()
                .expect(&format!("Unable to create lexer for {}", filename)[0..]);


    loop {
      let tok = f.next_token();
      match tok {
        Ok(t) =>
          match t.token_type {
            TokenType::EOF => break,
            tt => println!("{}: {:?}", t.line_number, tt)
          },
        Err(e) => panic!("{}", e.description())
      }
    }
  }

  fn lex_string(s: &str) {
    let source = Box::new(LexString::new(s.to_string()));
    let mut f = Lexer::new("<string>", source)
                .ok()
                .expect(&format!("Unable to create lexer for user string"));

    loop {
      let tok = f.next_token();
      match tok {
        Ok(t) =>
          match t.token_type {
            TokenType::EOF => break,
            tt => println!("{}: {:?}", t.line_number, tt)
          },
        Err(e) => panic!("{}", e.description())
      }
    }
  }

  #[test]
  fn hello() {
    lex("hello.lua")
  }

  #[test]
  fn string_lexer() {
    // TODO: there might be a bug here...
    //       return doesn't show up without the newline
    lex_string(r"local x = 1 + 2
                 print(x)
                 return");
  }

  /*#[test]
  fn life() {
    lex("life.lua")
  }*/
}
