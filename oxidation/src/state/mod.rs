//! # Oxidation -- Lua scripting language written in Rust
//!
//! # State - data structures representing the state of the interpreter
//!
//! # Examples
//!

use std::collections::HashMap;

pub struct LuaState {
  // Intern pool for strings.
  // TODO: Change to a HashSet once HashSet::get() is stable
  pub strings: HashMap<String, String>,
}

impl LuaState {
  pub fn new() -> LuaState {
    LuaState {
      strings: HashMap::new()
    }
  }
}

