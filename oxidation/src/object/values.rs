use std::hash::{Hash, Hasher};

use config::TLuaNumber;

use object::string::InString;
use object::function::Proto;
use object::table::Table;

// To get around Rust's trait orphan rules
#[derive(PartialEq,Debug,Clone)]
pub struct LuaNumber {
  value: TLuaNumber
}


#[derive(PartialEq,Eq,Hash,Debug,Clone)]
pub enum Value {
  Nil,
  Boolean(bool),
  Number(LuaNumber),
  String(InString),
  Function(Proto),
  Table(Table),
  Thread,               // TODO: Figure out how to represent this
  Userdata              // TODO: Figure out how to represent this
}

impl Value {
  fn is_false(&self) -> bool {
    match *self {
      Value::Nil => true,
      Value::Boolean(false) => true,
      _ => false
    }
  }
}


impl Eq for LuaNumber {}

// Shitty hash implementation, but not much we can do.
// C Lua's algorithm would require unsafe code in Rust.
impl Hash for LuaNumber {
  fn hash<H>(&self, state: &mut H) where H: Hasher {
    let s = format!("{}", self.value);
    s.hash(state);
  }
}

