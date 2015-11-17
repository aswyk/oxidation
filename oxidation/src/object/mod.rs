//! # Oxidation -- Lua scripting language written in Rust
//!
//! # Object - Data structures to represent Lua values
//!
//! # Examples
//!

pub mod values;
pub mod string;
pub mod function;
pub mod table;

#[cfg(test)]
mod tests {
  use state::LuaState;
  use object::string::InString;

  #[test]
  fn instring() {
    let mut ls = LuaState::new();

    let a = InString::new(&mut ls, &"hello".to_string());
    let b = InString::new(&mut ls, &"hello".to_string());
    let c = InString::new(&mut ls, &"world".to_string());

    println!("{:?}", a);
    println!("{:?}", b);

    assert!(a == b);
    assert!(a != c);

    assert!(ls.strings.len() == 2);
  }
}

