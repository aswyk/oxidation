use std::fmt::{Display, Formatter, Error};

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum TokenType {
  And,
  Break,
  Do,
  Else,
  Elseif,
  End,
  False,
  For,
  Function,
  If,
  In,
  Local,
  Nil,
  Not,
  Or,
  Repeat,
  Return,
  Then,
  True,
  Until,
  While,
  
  Plus,
  Minus,
  Times,
  Divide,
  Mod,
  Exp,
  Len,
  Eq,
  Neq,
  Leq,
  Geq,
  Lt,
  Gt,
  Assign,
  LParen,
  RParen,
  LBrace,
  RBrace,
  LSquare,
  RSquare,
  Semi,
  Colon,
  Comma,
  Dot,
  Dotdot,
  Dotdotdot,

  String(String),
  Number(String),
  Name(String),

  EOF
}

#[derive(Debug, Clone)]
pub struct Token {
  pub line_number: u32,
  pub token_type:  TokenType
}

impl Token {
  pub fn new(line_number: u32, token_type: TokenType) -> Token {
    Token {
      line_number: line_number,
      token_type: token_type
    }
  }
}

impl Display for TokenType {
  fn fmt(&self, f : &mut Formatter) -> Result<(), Error> {
    let repr = match *self {
      TokenType::And => "'and'",
      TokenType::Break => "'break'",
      TokenType::Do => "'do'",
      TokenType::Else => "'else'",
      TokenType::Elseif => "'elseif'",
      TokenType::End => "'end'",
      TokenType::False => "'false'",
      TokenType::For => "'for'",
      TokenType::Function => "'function'",
      TokenType::If => "'if'",
      TokenType::In => "'in'",
      TokenType::Local => "'local'",
      TokenType::Nil => "'nil'",
      TokenType::Not => "'not'",
      TokenType::Or => "'or'",
      TokenType::Repeat => "'repeat'",
      TokenType::Return => "'return'",
      TokenType::Then => "'then'",
      TokenType::True => "'true'",
      TokenType::Until => "'until'",
      TokenType::While => "'while'",

      TokenType::Plus => "'+'",
      TokenType::Minus => "'-'",
      TokenType::Times => "'*'",
      TokenType::Divide => "'/'",
      TokenType::Mod => "'%'",
      TokenType::Exp => "'^'",
      TokenType::Len => "'#'",
      TokenType::Eq => "'=='",
      TokenType::Neq => "'~='",
      TokenType::Leq => "'<='",
      TokenType::Geq => "'>='",
      TokenType::Lt => "'<'",
      TokenType::Gt => "'>'",
      TokenType::Assign => "'='",
      TokenType::LParen => "'('",
      TokenType::RParen => "')'",
      TokenType::LBrace => "'{'",
      TokenType::RBrace => "'}'",
      TokenType::LSquare => "'['",
      TokenType::RSquare => "']'",
      TokenType::Semi => "';'",
      TokenType::Colon => "':'",
      TokenType::Comma => "','",
      TokenType::Dot => "'.'",
      TokenType::Dotdot => "'..'",
      TokenType::Dotdotdot => "'...'",

      TokenType::String(_) => "string literal",
      TokenType::Number(_) => "number literal",
      TokenType::Name(_)   => "name",

      TokenType::EOF => "<eof>"
    };

    try!(f.write_str(repr));
    
    Ok(())
  }
}
