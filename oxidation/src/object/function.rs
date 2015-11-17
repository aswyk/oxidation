use object::values::Value;
//use vm::Instruction;

pub struct Proto {
  k: Vec<Value>,
//code: Vec<Instruction>,
  p: Vec<Proto>,
  lineinfo: Vec<u32>,

  // TODO
}

