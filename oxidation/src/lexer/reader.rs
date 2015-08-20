use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::fs::File;
use std::char;

use parser::tokens::Token;

pub trait Reader {
  fn next_token(&mut self) -> Result<Token>;
  fn error(&self, &str) -> !;
}

pub struct FileReader {
  pub file_name   : String,
  pub line_number : u32,
  
  source_line     : String,
  line_pos        : usize,
  source_file     : BufReader<File>,
}

impl FileReader {
  pub fn new(filename : &str) -> Result<FileReader> {
    let mut f = try!(File::open(Path::new(filename)));
    let mut b = BufReader::new(f);
    let mut s = String::new();
    try!(b.read_line(&mut s));

    Ok(FileReader {
      file_name: filename.to_string(),
      line_number: 1,

      source_line: s,
      line_pos: 0,
      source_file: b,
    })
  }

  fn bump(&mut self) -> Result<char> {
    if self.line_pos >= self.source_line.len() - 1 {
      self.source_line.clear();
      let r = self.source_file.read_line(&mut self.source_line);
      match r {
        Ok(0) => return Err(Error::new(ErrorKind::Other, "EOF")),
        Err(e) => return Err(e),
        _ => ()
      }
      self.line_pos = 0;
      self.line_number += 1;
      Ok(self.source_line[self.line_pos..].chars().next().unwrap())
    } else {
      let c = self.source_line[self.line_pos..].chars().next().unwrap();
      self.line_pos += c.len_utf8();
      Ok(c)
    }
  }

  fn bump_line(&mut self) -> Result<()> {
    self.source_line.clear();
    let r = self.source_file.read_line(&mut self.source_line);
    match r {
      Ok(0) => return Err(Error::new(ErrorKind::Other, "EOF")),
      Err(e) => return Err(e),
      _ => ()
    }
    self.line_pos = 0;
    self.line_number += 1;
    Ok(())
  }

  fn current(&self) -> char {
    self.source_line[self.line_pos..].chars().next().unwrap()
  }

  fn skip_sep(&mut self) -> Result<i8> {
    let mut n = 0;
    let s = self.current();
    if s != '[' && s != ']' {
      self.error("skip_sep");
    }
    try!(self.bump());
    loop {
      if self.current() != '=' {
        break;
      }
      try!(self.bump());
      n += 1;
    }

    if self.current() == s {
      Ok(n)
    } else {
      Ok((-n) - 1)
    }
  }

  fn read_long_string(&mut self, sep : i8) -> Result<String> {
    let mut s = String::new();

    try!(self.bump());   // skip 2nd [
    if self.current() == '\r' || self.current() == '\n' {
      // Lua ignores an immediate newline for long strings
      // so that it will not be included in the string
      try!(self.bump_line());
    }
    
    loop {
      match self.current() {
        ']' => {
          let p = try!(self.skip_sep());
          if p == sep {
            try!(self.bump());  // skip 2nd ]
            return Ok(s);
          }
        },
        c => {
          s.push(c);
          try!(self.bump());
        }
      }
    }
  }

  fn read_string(&mut self) -> Result<String> {
    let del = &self.current();
    let mut s = String::new();
    try!(self.bump());   // skip leading del
    
    while self.current() != *del {
      match self.current() {
        '\n' | '\r' => self.error("unfinished string"),
        '\\' => {
          let mut c : char;
          try!(self.bump());
          match self.current() {
            'a' => c = '\x07',
            'b' => c = '\x08',
            'f' => c = '\x0C',
            'n' => c = '\n',
            'r' => c = '\r',
            't' => c = '\t',
            'v' => c = '\x0B',
            x => {
              if !x.is_digit(10) {
                c = x;
              } else {
                let mut y : u32 = 0;
                let mut i : u8 = 0;
                loop {
                  y = 10*y + self.current().to_digit(10).unwrap();
                  try!(self.bump());
                  i += 1;
                  if i >= 3 || !self.current().is_digit(10) {
                    break;
                  }
                }
                c = char::from_u32(y).unwrap();
              }
            }
          }
          s.push(c);
        },
        c => s.push(c)
      }
      try!(self.bump());
    }
    try!(self.bump());   // skip final del

    Ok(s)
  }

  fn read_number(&mut self) -> Result<String> {
    let mut s = String::new();

    loop {
      s.push(self.current());
      try!(self.bump());

      if !self.current().is_digit(10) && self.current() != '.' {
        break;
      }
    }

    if self.current() == 'E' || self.current() == 'e' {
      s.push(self.current());
      try!(self.bump());
      if self.current() == '+' || self.current() == '-' {
        s.push(self.current());
        try!(self.bump());
      }
    }

    while self.current().is_alphanumeric() || self.current() == '_' {
      s.push(self.current());
      try!(self.bump());
    }

    Ok(s)
  }

  fn read_name(&mut self) -> Result<Token> {
    let mut s = String::new();

    while self.current().is_alphanumeric() || self.current() == '_' {
      s.push(self.current());
      try!(self.bump());
    }

    match &s[0..] {
      "and" => Ok(Token::And),
      "break" => Ok(Token::Break),
      "do" => Ok(Token::Do),
      "else" => Ok(Token::Else),
      "elseif" => Ok(Token::Elseif),
      "end" => Ok(Token::End),
      "false" => Ok(Token::False),
      "for" => Ok(Token::For),
      "function" => Ok(Token::Function),
      "if" => Ok(Token::If),
      "in" => Ok(Token::In),
      "local" => Ok(Token::Local),
      "nil" => Ok(Token::Nil),
      "not" => Ok(Token::Not),
      "or" => Ok(Token::Or),
      "repeat" => Ok(Token::Repeat),
      "return" => Ok(Token::Return),
      "then" => Ok(Token::Then),
      "true" => Ok(Token::True),
      "until" => Ok(Token::Until),
      "while" => Ok(Token::While),
      _ => Ok(Token::Name(s))
    }
  }

  fn or_eq(&mut self, base : Token, eq : Token) -> Result<Token> {
    try!(self.bump());
    if self.current() != '=' {
      return Ok(base);
    } else {
      try!(self.bump());
      return Ok(eq);
    }
  }
}

impl Reader for FileReader {
  fn next_token(&mut self) -> Result<Token> {
    loop {
      match self.current() {
        '-' => {
          try!(self.bump());
          if self.current() != '-' {
            return Ok(Token::Minus);
          } else {
            // Lex a comment
            try!(self.bump());
            if self.current() == '[' {
              let sep = try!(self.skip_sep());
              if sep >= 0 {
                // Block comment
                try!(self.read_long_string(sep));
                continue;
              }
            } else {
              // Line comment
              println!("Found comment");
              try!(self.bump_line());
              continue;
            }
          }
        },

        '[' => {
          let sep = try!(self.skip_sep());
          if sep >= 0 {
            let s = try!(self.read_long_string(sep));
            return Ok(Token::String(s));
          } else if sep == -1 {
            return Ok(Token::LSquare);
          } else {
            self.error("invalid string delimiter");
          }
        },

        '=' => return self.or_eq(Token::Assign, Token::Eq),
        '<' => return self.or_eq(Token::Lt, Token::Leq),
        '>' => return self.or_eq(Token::Gt, Token::Geq),
        '~' => return self.or_eq(Token::Not, Token::Neq),

        '"' | '\'' => {
          let s = try!(self.read_string());
          return Ok(Token::String(s));
        },

        '.' => {
          try!(self.bump());
          if self.current() == '.' {
            try!(self.bump());
            if self.current() == '.' {
              return Ok(Token::Dotdotdot);
            } else {
              return Ok(Token::Dotdot);
            }
          } else if !self.current().is_digit(10) {
            return Ok(Token::Dot);
          } else {
            let s = try!(self.read_number());
            return Ok(Token::Number(s));
          }
        },

        '+' => { try!(self.bump()); return Ok(Token::Plus) },
        '*' => { try!(self.bump()); return Ok(Token::Times) },
        '/' => { try!(self.bump()); return Ok(Token::Divide) },
        '%' => { try!(self.bump()); return Ok(Token::Mod) },
        '^' => { try!(self.bump()); return Ok(Token::Exp) },
        '#' => { try!(self.bump()); return Ok(Token::Len) },
        '(' => { try!(self.bump()); return Ok(Token::LParen) },
        ')' => { try!(self.bump()); return Ok(Token::RParen) },
        '{' => { try!(self.bump()); return Ok(Token::LBrace) },
        '}' => { try!(self.bump()); return Ok(Token::RBrace) },
        ']' => { try!(self.bump()); return Ok(Token::RSquare) },
        ';' => { try!(self.bump()); return Ok(Token::Semi) },
        ':' => { try!(self.bump()); return Ok(Token::Colon) },
        ',' => { try!(self.bump()); return Ok(Token::Comma) },

        _ => {
          if self.current().is_whitespace() {
            try!(self.bump());
            continue;
          } else if self.current().is_digit(10) {
            let s = try!(self.read_number());
            return Ok(Token::Number(s));
          } else if self.current().is_alphabetic() || self.current() == '_' {
            return self.read_name();
          } else {
            self.error("unexpected character");
          }
        }
      }
    }
  }

  fn error(&self, s : &str) -> ! {
    panic!("lexical error: {}:{}: {}", self.file_name, self.line_number, s)
  }
}


