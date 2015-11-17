use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use config::TLuaNumber;

use object::values::Value;

#[derive(PartialEq,Eq,Debug,Clone)]
pub struct Table {
  array_part: Vec<Rc<Value>>,
  hash_part:  HashMap<Value, Rc<Value>>
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
      array_part: a.into_iter().map(|x| Rc::new(x)).collect(),
      hash_part: HashMap::new()
    }
  }

  pub fn insert(&mut self, key: Value, val: Value) {
    if let Some(k) = arrayindex(&key) {
      let ln = self.array_part.len();
      if k < ln * 2 {
        if k < ln {
          self.array_part[k] = Rc::new(val);
        } else {
          self.grow_array(ln * 2);
          self.array_part[k] = Rc::new(val);
        }
      } else {
        self.hash_part.insert(key, Rc::new(val));
      }
    } else {
      self.hash_part.insert(key, Rc::new(val));
    }
  }

  pub fn get(&self, key: &Value) -> Rc<Value> {
    let mut rc = Rc::new(Value::Nil);

    if let Some(k) = arrayindex(&key) {
      if k < self.array_part.len() {
        rc = self.array_part[k].clone();
      }
      
      // If it's not in the array part even if it could be, check
      // the hash part.
      if let Value::Nil = *rc {
        if self.hash_part.contains_key(key) {
          return self.hash_part[key].clone();
        } else {
          return Rc::new(Value::Nil);
        }
      } else {
        return rc;
      }
    } else {
      if self.hash_part.contains_key(key) {
        return self.hash_part[key].clone();
      } else {
        return Rc::new(Value::Nil);
      }
    }

    return Rc::new(Value::Nil);
  }

  fn grow_array(&mut self, capacity: usize) {
    let i = self.array_part.len();
    while i <= capacity {
      self.array_part.push(Rc::new(Value::Nil));
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

