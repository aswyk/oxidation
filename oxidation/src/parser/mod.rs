//! # Oxidation -- Lua scripting language written in Rust
//!
//! * Parser - Transforms token stream into byte code
//!
//! # Examples
//!

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod tokens;
pub mod parser;

#[cfg(test)]
mod tests {
  use std::error::Error;
  use lexer::reader::Reader;
  use lexer::reader::FileReader;
  use parser::parser::Parser;

  fn parse(filename : &str) {
    let mut f = FileReader::new(filename)
                .ok()
                .expect(&format!("Unable to open {} for test", filename)[0..]);

    //let mut parser = Parser::new(f);

    //match parser {
    match Parser::new(f) {
        Some(mut p) => { p.parse(None); },
        None => { println!("ERROR :  Return DEBUG info."); }
    }
    //parser.parse(None);

    /*
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
    }*/
  }

  #[test]
  fn parser() {
    parse("hello.lua")
  }
}
