//! # Oxidation -- Lua scripting language written in Rust
//!
//! * Virtual Machine - Runs bytecode
//!
//! # Examples
//!


pub mod opcode;
pub mod vm;


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

    let mut b = BufReader::new(f);

    let mut f = Lexer::new(filename, &mut b)
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
    let mut source = LexString::new(s.to_string());
    let mut f = Lexer::new("<string>", &mut source)
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
  fn test_vm() {
      println!("Testing the Oxidation VM (beta)");
      lex("hello.lua")
  }
}
