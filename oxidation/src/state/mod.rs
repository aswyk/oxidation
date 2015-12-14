//! # Oxidation -- Lua scripting language written in Rust
//!
//! # State - data structures representing the state of the interpreter
//!
//! # Examples
//!

use std::rc::Rc;
use std::collections::HashMap;

use object::values::Value;
use gc::{GCObject, Root};

use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, Eq};

#[derive(Debug, Clone)]
pub struct LuaState {
  // Intern pool for strings.
  // TODO: Change to a HashSet once HashSet::get() is stable
  pub strings: HashMap<String, Rc<String>>,
  
  pub stack: Vec<Value>,
  pub heap: Vec<GCObject>,
  pub roots: Vec<Root>
}

impl LuaState {
  pub fn new() -> LuaState {
    LuaState {
      strings: HashMap::new(),

      stack: Vec::new(),
      heap: Vec::new(),
      roots: Vec::new()
    }
  }
}

impl Hash for LuaState {
  // TODO: Is this enough?
  fn hash<H>(&self, state: &mut H) where H: Hasher {
    self.stack.hash(state);
  }
}

impl PartialEq for LuaState {
  fn eq(&self, other: &Self) -> bool {
    let a = self as *const LuaState;
    let b = other as *const LuaState;
    a == b
  }
}

impl Eq for LuaState {}
