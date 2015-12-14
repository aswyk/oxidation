use std::hash::{Hash, Hasher};

use config::TLuaNumber;

use object::string::InString;
use object::function::Proto;
use object::table::Table;

use gc::GCObject;

use state::LuaState;

// To get around Rust's trait orphan rules
#[derive(PartialEq,Debug,Clone)]
pub struct LuaNumber {
  pub value: TLuaNumber
}

#[derive(PartialEq,Eq,Hash,Debug,Clone)]
pub enum Value {
  Nil,
  Number(LuaNumber),
  Boolean(bool),
  Object(GCObject),
  //LightUserdata         // TODO: Figure out how to represent this
}

#[derive(PartialEq,Eq,Hash,Debug,Clone)]
pub enum GCVariant {
  String(InString),
  //Closure(Closure),       // TODO
  Table(Table),
  Proto(Proto),
  //Upvalue(Upvalue),       // TODO
  Thread(LuaState),
  //Userdata              // TODO: Figure out how to represent this
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

