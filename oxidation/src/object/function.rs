use object::values::Value;
use object::string::InString;
use vm::opcode::Instruction;

#[derive(PartialEq,Eq,Hash,Debug,Clone)]
pub struct LocVar {
  varname: InString,
  startpc: u32,             // first point where variable is active
  endpc: u32                // first point where variable is dead
}


// Masks for vararg
pub const HASARG  : u8 = 1;
pub const ISVARARG: u8 = 2;
pub const NEEDSARG: u8 = 4;


/// Function prototypes
#[derive(PartialEq,Eq,Hash,Debug,Clone)]
pub struct Proto {
  k: Vec<Value>,            // constants used by the function
  code: Vec<Instruction>,
  p: Vec<Proto>,            // functions defined inside the function
  lineinfo: Vec<u32>,       // map from instructions to source lines
  locvars: Vec<LocVar>,     // information about local variables
  upvalues: Vec<InString>,  // upvalue names
  source: InString,         // source code for this function
  linedefined: u32,
  lastlinedefined: u32,
//TODO: gclist?
  nups: u8,                 // number of upvalues
  numparams: u8,
  is_vararg: u8,            // OR'd values of HASARG, ISVARARG, NEEDSARG
  maxstacksize: u8
}


impl LocVar {
  pub fn new(varname: InString, startpc: u32, endpc: u32) -> LocVar
  {
    LocVar {
      varname: varname,
      startpc: startpc,
      endpc: endpc
    }
  }
}


impl Proto {
  pub fn new(source: InString,
             linedefined: u32,
             lastlinedefined: u32,
             nups: u8,
             numparams: u8,
             is_vararg: u8,
             maxstacksize: u8)
  -> Proto {
    Proto {
      k: Vec::new(),
      code: Vec::new(),
      p: Vec::new(),
      lineinfo: Vec::new(),
      locvars: Vec::new(),
      upvalues: Vec::new(),
      source: source,
      linedefined: linedefined,
      lastlinedefined: lastlinedefined,
      nups: nups,
      numparams: numparams,
      is_vararg: is_vararg,
      maxstacksize: maxstacksize
    }
  }
}

