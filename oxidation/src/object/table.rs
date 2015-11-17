use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use object::values::Value;

#[derive(PartialEq,Eq,Debug,Clone)]
pub struct Table {
  array_part: Vec<Value>,
  hash_part:  HashMap<Value, Value>
}

impl Hash for Table {
  fn hash<H>(&self, state: &mut H) where H: Hasher {
    self.array_part.hash(state);
    for (key, val) in &self.hash_part {
      key.hash(state);
      val.hash(state);
    }
  }
}

// TODO
