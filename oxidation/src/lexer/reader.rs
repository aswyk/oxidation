use std::char;

use parser::tokens::{Token, TokenType as TT};
use lexer::result::{LexerResult, LexerError};
use lexer::stringsource::StringSource;


pub trait Reader {
  fn next_token(&mut self) -> LexerResult<Token>;
  fn peek_token(&mut self) -> LexerResult<Token>;
}


pub struct Lexer {
  pub source_name   : String,

  line_number       : u32,
  source            : Box<StringSource>,
  source_line       : String,
  line_pos          : usize,
  peek_tok          : Option<Token>
}

impl Lexer {
  pub fn new(source_name: &str, mut source: Box<StringSource>)
    -> LexerResult<Lexer>
  {
    let s = try!(source.get_string());

    Ok(Lexer {
      source_name: source_name.to_string(),

      line_number: 1,
      source: source,
      source_line: s,
      line_pos: 0,
      peek_tok: None
    })
  }

  // Make a new token with the current line number
  fn token(&self, tt: TT) -> Token {
    Token::new(self.line_number, tt)
  }

  // Advance the lexer state ahead one character
  fn bump(&mut self) -> LexerResult<()> {
    if self.line_pos >= self.source_line.len() - 1 {
      self.bump_line()
    } else {
      let c = self.current();
      self.line_pos += c.len_utf8();
      Ok(())
    }
  }

  // Advance the lexer state ahead one line
  fn bump_line(&mut self) -> LexerResult<()> {
    self.source_line.clear();
    let s = try!(self.source.get_string());

    self.source_line = s;
    self.line_pos = 0;
    self.line_number += 1;

    Ok(())
  }

  // Get the character at the current lexer position
  fn current(&self) -> char {
    self.source_line[self.line_pos..].chars().next().unwrap()
  }

  // Skip n equals signs in a long string delimiter. Return n.
  fn skip_sep(&mut self) -> LexerResult<i8> {
    let mut n = 0;
    let s = self.current();
    if s != '[' && s != ']' {
      return Err(LexerError::GenericError("skip_sep"))
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

  // Read a long string of the form [[str]], [=[str]=], ...
  fn read_long_string(&mut self, sep : i8) -> LexerResult<String> {
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

  // Read a normal form string in quotes
  fn read_string(&mut self) -> LexerResult<String> {
    let del = &self.current();
    let mut s = String::new();
    try!(self.bump());   // skip leading del

    while self.current() != *del {
      match self.current() {
        '\n' | '\r' =>
          return Err(LexerError::GenericError("unfinished string")),
        '\\' => {
          // TODO: I feel like this should be mut...
          let c : char;
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

  // Read a number... might be malformed...
  fn read_number(&mut self) -> LexerResult<String> {
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

  // Read a keyword or identifier
  fn read_name(&mut self) -> LexerResult<Token> {
    let mut s = String::new();

    while self.current().is_alphanumeric() || self.current() == '_' {
      s.push(self.current());
      try!(self.bump());
    }

    match &s[0..] {
      "and" => Ok(self.token(TT::And)),
      "break" => Ok(self.token(TT::Break)),
      "do" => Ok(self.token(TT::Do)),
      "else" => Ok(self.token(TT::Else)),
      "elseif" => Ok(self.token(TT::Elseif)),
      "end" => Ok(self.token(TT::End)),
      "false" => Ok(self.token(TT::False)),
      "for" => Ok(self.token(TT::For)),
      "function" => Ok(self.token(TT::Function)),
      "if" => Ok(self.token(TT::If)),
      "in" => Ok(self.token(TT::In)),
      "local" => Ok(self.token(TT::Local)),
      "nil" => Ok(self.token(TT::Nil)),
      "not" => Ok(self.token(TT::Not)),
      "or" => Ok(self.token(TT::Or)),
      "repeat" => Ok(self.token(TT::Repeat)),
      "return" => Ok(self.token(TT::Return)),
      "then" => Ok(self.token(TT::Then)),
      "true" => Ok(self.token(TT::True)),
      "until" => Ok(self.token(TT::Until)),
      "while" => Ok(self.token(TT::While)),
      _ => Ok(self.token(TT::Name(s)))
    }
  }

  // Read an operator or the "operator or equal" form
  fn or_eq(&mut self, base : Token, eq : Token) -> LexerResult<Token> {
    try!(self.bump());
    if self.current() != '=' {
      return Ok(base);
    } else {
      try!(self.bump());
      return Ok(eq);
    }
  }

  // Real impl for next_token wrapped for EOF
  fn real_next_token(&mut self) -> LexerResult<Token> {
    match self.peek_tok.clone() {
      Some(tok) => {
        self.peek_tok = None;
        return Ok(tok);
      },
      None => ()
    }

    loop {
      match self.current() {
        '-' => {
          try!(self.bump());
          if self.current() != '-' {
            return Ok(self.token(TT::Minus));
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
              try!(self.bump_line());
              continue;
            }
          }
        },

        '[' => {
          let sep = try!(self.skip_sep());
          if sep >= 0 {
            let s = try!(self.read_long_string(sep));
            return Ok(self.token(TT::String(s)));
          } else if sep == -1 {
            // Just a single [
            return Ok(self.token(TT::LSquare));
          } else {
            return Err(LexerError::GenericError("invalid string delimiter"));
          }
        },

        // Comparison operators
        '=' => {
          let assign = self.token(TT::Assign);
          let eq = self.token(TT::Eq);
          return self.or_eq(assign, eq)
        },
        '<' => {
          let lt = self.token(TT::Lt);
          let leq = self.token(TT::Leq);
          return self.or_eq(lt, leq)
        },
        '>' => {
          let gt = self.token(TT::Gt);
          let geq = self.token(TT::Geq);
          return self.or_eq(gt, geq)
        },
        '~' => {
          let not = self.token(TT::Not);
          let neq = self.token(TT::Neq);
          return self.or_eq(not, neq)
        },

        '"' | '\'' => {
          let s = try!(self.read_string());
          return Ok(self.token(TT::String(s)));
        },

        '.' => {
          try!(self.bump());
          if self.current() == '.' {
            try!(self.bump());
            if self.current() == '.' {
              return Ok(self.token(TT::Dotdotdot));
            } else {
              return Ok(self.token(TT::Dotdot));
            }
          } else if !self.current().is_digit(10) {
            return Ok(self.token(TT::Dot));
          } else {
            let s = try!(self.read_number());
            return Ok(self.token(TT::Number(s)));
          }
        },

        // The easy ones.  Unambiguous single-char tokens
        '+' => { try!(self.bump()); return Ok(self.token(TT::Plus)) },
        '*' => { try!(self.bump()); return Ok(self.token(TT::Times)) },
        '/' => { try!(self.bump()); return Ok(self.token(TT::Divide)) },
        '%' => { try!(self.bump()); return Ok(self.token(TT::Mod)) },
        '^' => { try!(self.bump()); return Ok(self.token(TT::Exp)) },
        '#' => { try!(self.bump()); return Ok(self.token(TT::Len)) },
        '(' => { try!(self.bump()); return Ok(self.token(TT::LParen)) },
        ')' => { try!(self.bump()); return Ok(self.token(TT::RParen)) },
        '{' => { try!(self.bump()); return Ok(self.token(TT::LBrace)) },
        '}' => { try!(self.bump()); return Ok(self.token(TT::RBrace)) },
        ']' => { try!(self.bump()); return Ok(self.token(TT::RSquare)) },
        ';' => { try!(self.bump()); return Ok(self.token(TT::Semi)) },
        ':' => { try!(self.bump()); return Ok(self.token(TT::Colon)) },
        ',' => { try!(self.bump()); return Ok(self.token(TT::Comma)) },

        _ => {
          if self.current().is_whitespace() {
            // Skip white space
            try!(self.bump());
            continue;
          } else if self.current().is_digit(10) {
            let s = try!(self.read_number());
            return Ok(self.token(TT::Number(s)));
          } else if self.current().is_alphabetic() || self.current() == '_' {
            return self.read_name();
          } else {
            return Err(LexerError::GenericError("unexpected character"));
          }
        }
      }
    }
  }
}


impl Reader for Lexer {
  /// Get the next token.  Advances the lexer state.
  fn next_token(&mut self) -> LexerResult<Token> {
    let tok = self.real_next_token();
    match tok {
      Ok(t) => Ok(t),
      Err(LexerError::EOF) => Ok(self.token(TT::EOF)),
      err => err
    }
  }

  /// Have a look at the next token.  May advance lexer state, may not.
  fn peek_token(&mut self) -> LexerResult<Token> {
    match self.peek_tok.clone() {
      Some(tok) => Ok(tok),
      None => {
        let tok = try!(self.next_token());
        self.peek_tok = Some(tok.clone());
        Ok(tok)
      }
    }
  }
}
