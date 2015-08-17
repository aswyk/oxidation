//! # Oxidation -- Lua scripting language written in Rust
//!
//!
//! Oxidation's goal is to impliment the Lua scripting language and VM
//! in Rust.
//!
//! Oxidation Components
//! * Lexer - Transforms source text into a token stream
//! * Parser - Transforms token stream into byte code
//! * Virtual Machine - Runs bytecode
//! * Object - Implimentation of internal data layout.
//! * Garbage Collector - Collects Garbage, DUH.
//! * TODO: std - Standard library functions available in the scripting language.
//!
//! # How to use the core library
//!     TODO: ...


pub mod lexer;
pub mod parser;
pub mod vm;
pub mod gc;
pub mod object;


//pub use vm::*;
//pub use opcode::*;

#[test]
fn it_works() {
}
