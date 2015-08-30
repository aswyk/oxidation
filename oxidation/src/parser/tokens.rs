use std::fmt::{Display, Formatter, Error};

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Token {
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

impl Display for Token {
  fn fmt(&self, f : &mut Formatter) -> Result<(), Error> {
    let repr = match *self {
      Token::And => "'and'",
      Token::Break => "'break'",
      Token::Do => "'do'",
      Token::Else => "'else'",
      Token::Elseif => "'elseif'",
      Token::End => "'end'",
      Token::False => "'false'",
      Token::For => "'for'",
      Token::Function => "'function'",
      Token::If => "'if'",
      Token::In => "'in'",
      Token::Local => "'local'",
      Token::Nil => "'nil'",
      Token::Not => "'not'",
      Token::Or => "'or'",
      Token::Repeat => "'repeat'",
      Token::Return => "'return'",
      Token::Then => "'then'",
      Token::True => "'true'",
      Token::Until => "'until'",
      Token::While => "'while'",

      Token::Plus => "'+'",
      Token::Minus => "'-'",
      Token::Times => "'*'",
      Token::Divide => "'/'",
      Token::Mod => "'%'",
      Token::Exp => "'^'",
      Token::Len => "'#'",
      Token::Eq => "'=='",
      Token::Neq => "'~='",
      Token::Leq => "'<='",
      Token::Geq => "'>='",
      Token::Lt => "'<'",
      Token::Gt => "'>'",
      Token::Assign => "'='",
      Token::LParen => "'('",
      Token::RParen => "')'",
      Token::LBrace => "'{'",
      Token::RBrace => "'}'",
      Token::LSquare => "'['",
      Token::RSquare => "']'",
      Token::Semi => "';'",
      Token::Colon => "':'",
      Token::Comma => "','",
      Token::Dot => "'.'",
      Token::Dotdot => "'..'",
      Token::Dotdotdot => "'...'",

      Token::String(_) => "string literal",
      Token::Number(_) => "number literal",
      Token::Name(_)   => "name",

      Token::EOF => "<eof>"
    };

    try!(f.write_str(repr));
    
    Ok(())
  }
}
