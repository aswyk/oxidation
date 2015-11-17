use config::LuaNumber;

use object::string::InString;
//use object::function::Proto;
//use object::table::Table;

pub enum Value {
  Nil,
  Boolean(bool),
  Number(LuaNumber),
  String(InString),
//Function(Proto),
//Table(Table),
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

