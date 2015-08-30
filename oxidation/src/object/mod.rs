//! # Oxidation -- Lua scripting language written in Rust
//!
//! # Object - Some lua types we need in rust.
//!
//! # Examples
//!

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub type lua_Number = f64;
pub type Instruction = u32;

/// Common Headers Section

pub struct CommonHeader {
    next: Option<Box<GCObject>>,
    tt: u8,
    marked: u8,
}

pub struct ClosureHeader {
    isC: u8,
    nupvalues: u8,
    gclist: Vec<Box<GCObject>>,
    env: Table,
}


//#define ClosureHeader \
//	CommonHeader; lu_byte isC; lu_byte nupvalues; GCObject *gclist; \
//	struct Table *env


pub struct CClosure {
    /*isC: u8,
    nupvalues: u8,
    gclist: Vec<GCObject>,
    env: Table,*/
    header: ClosureHeader,
    //f: lua_CFunction,
    upvalue: Vec<TValue>,
}

pub struct LClosure {
    /*isC: u8,
    nupvalues: u8,
    gclist: Vec<GCObject>,
    env: Table,*/
    header: ClosureHeader,
    p: Proto,
    upvals: Vec<UpVal>,
}

pub enum Closure {
    c(CClosure),
    l(LClosure),
}

pub struct TString {
    //next: Box<GCObject>,
    //marked: u8,
    header: CommonHeader,
    reserved: u8,
    hash: u64,
    len: usize,
}


pub struct Udata {
    // L_Umaxalign dummy;  /* ensures maximum alignment for `local' udata */
    header: CommonHeader,
    //next: Box<GCObject>,
    //marked: u8,
    metatable: Table,
    env: Table,
    len: usize,
}

//#define CommonHeader	GCObject *next; lu_byte tt; lu_byte marked

pub struct GCHeader {
    // Internals are known as CommonHeader in lua (#define)
    header: CommonHeader,
    //next: Box<GCObject>,
    //marked: u8,
}

pub enum GCObject {
    gch(Box<GCHeader>),
    ts(TString),
    u(Udata),
    cl(Closure),
    h(Table),
    p(Proto),
    uv(UpVal),
    //th(lua_State), // For threads
}

pub enum Value {
    gc(Box<GCObject>),
    // need to add void *p -> Cory says this points to user daya
    n(lua_Number),
    b(bool),
}

// TValueFields Value value; int tt
pub struct TValue {
    value: Value,
}

pub enum TKey {
    nk {
        value: Value,
        next: Box<Node>,
    },
    tvk(TValue),
}

pub struct Node {
    i_val: TValue,
    i_key: TKey,
}

pub struct Table {
    //Add CommonHeader
    header: CommonHeader,
    flags: u8,
    lsizenode: u8,
    metatable: Option<Box<Table>>,
    array: Vec<TValue>,
    node: Node,
    lastefree: Node,
    gclist: Vec<Box<GCObject>>,
    sizearray: i32,
}

pub struct LocVar {
    varname: TString,
    startpc: i32,
    endpc: i32,
}

pub enum UpValUnion_u {
    value(TValue),
    l { prev: Box<UpVal>, next: Box<UpVal> },
}

pub struct UpVal {
    //next: Box<GCObject>,
    //marked: u8,
    header: CommonHeader,
    v: TValue,
    u: UpValUnion_u,
}

pub struct Proto {
    //add CommonHeader
    k: Vec<TValue>,
    code: Instruction,
    p: Vec<Proto>,
    lineinfo: Vec<i32>,
    locvars: Vec<LocVar>,
    upvalues: Vec<TString>,
    source: TString,
    sizeupvalues: i32,
    sizek: i32,
    sizecode: i32,
    sizelineinfo: i32,
    sizep: i32,
    sizelocvars: i32,
    linedefined: i32,
    lastlinedefined: i32,
    gclist: Vec<GCObject>,
    nups: u8,
    numparams: u8,
    is_vararg: u8,
    maxstacksize: u8,
}
