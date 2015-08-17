//! # Oxidation -- Lua scripting language written in Rust
//!
//! # Opcode
//!
//! # Examples
//!     TODO: ...

#![allow(non_camel_case_types)]

pub enum oxOpcodeE {
    MOVE{a: i32, b: i32},
    ADD{a: i32, b: i32, c: i32},
    CALL{a: i32, b: i32, c: i32},
    JMP{a: i32},
    EQ{a: i32, b: i32, c: i32},
    LT{a: i32, b: i32, c: i32},
}
