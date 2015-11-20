//! # Oxidation -- Lua scripting language written in Rust
//!
//! * Virtual Machine - Runs bytecode
//!
//! # Examples
//!


pub mod opcode;
pub mod vm;


#[cfg(test)]
mod tests {
  #[test]
  fn test_vm() {
      println!("Testing the Oxidation VM (beta)");
  }
}
