// This code needs a lot of love...
//
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use config::TLuaNumber;

use object::values::Value;

#[derive(PartialEq,Eq,Debug,Clone)]
pub struct Table {
  array_part: Vec<Value>,
  hash_part:  HashMap<Value, Value>
}

impl Table {
  pub fn new() -> Table {
    Table {
      array_part: Vec::new(),
      hash_part: HashMap::new()
    }
  }

  pub fn new_with_array(a: Vec<Value>) -> Table {
    Table {
      array_part: a,
      hash_part: HashMap::new()
    }
  }

  pub fn insert(&mut self, key: Value, val: Value) {
    if let Some(k) = arrayindex(&key) {
      let ln = self.array_part.len();
      if k < ln * 2 {
        if k < ln {
          self.array_part[k] = val;
        } else {
          self.grow_array(ln * 2);
          self.array_part[k] = val;
        }
      } else {
        self.hash_part.insert(key, val);
      }
    } else {
      self.hash_part.insert(key, val);
    }
  }

  pub fn get(&self, key: &Value) -> Value {
    let mut rc = Value::Nil;

    if let Some(k) = arrayindex(&key) {
      if k < self.array_part.len() {
        rc = self.array_part[k].clone();
      }
      
      // If it's not in the array part even if it could be, check
      // the hash part.
      if let Value::Nil = rc {
        if self.hash_part.contains_key(key) {
          return self.hash_part[key].clone();
        } else {
          return Value::Nil;
        }
      } else {
        return rc.clone();
      }
    } else {
      if self.hash_part.contains_key(key) {
        return self.hash_part[key].clone();
      } else {
        return Value::Nil;
      }
    }

    return Value::Nil;
  }

  fn grow_array(&mut self, capacity: usize) {
    let i = self.array_part.len();
    while i <= capacity {
      self.array_part.push(Value::Nil);
    }
  }
}


// Returns Some(index) for key if key is an appropriate key for
// the array part of the table, else None.
fn arrayindex(key: &Value) -> Option<usize> {
  match *key {
    Value::Number(ref n) => {
      let k = n.value as usize;
      if k as TLuaNumber == n.value {
        Some(k)
      } else {
        None
      }
    },
    _ => None
  }
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

