use std::hash::{Hash, Hasher};
use std::rc::Rc;

use state::LuaState;

/// An interned string.  String interning guarantees that only
/// one copy of a string exists in the runtime at any given time.
pub struct InString {
  string: Rc<String>,        // TODO: use &str and .as_ptr() instead?
  intern: *const Rc<String>
}

impl InString {
  pub fn new(ls: &mut LuaState, s: &String) -> InString {
    let insert : bool;

    {
      let val = ls.strings.get(s);
      match val {
        Some(_) => insert = false,
        None => insert = true,
      }
    }

    if insert {
      ls.strings.insert(s.clone(), Rc::new(s.clone()));
    }
    
    let x = ls.strings[s].clone();
    
    InString {
      string: x,
      intern: &ls.strings[s] as *const Rc<String>
    }
  }
}

// Comparison for interned strings is just pointer comparison.
impl PartialEq for InString {
  fn eq(&self, other: &InString) -> bool {
    self.intern == other.intern
  }
}

impl Eq for InString {}

impl Hash for InString {
  fn hash<H>(&self, state: &mut H) where H: Hasher {
    self.string.hash(state)
  }
}

impl ToString for InString {
  fn to_string(&self) -> String {
    let ref s = *self.string;
    s.clone()
  }
}

