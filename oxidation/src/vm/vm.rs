//! # Oxidation -- Lua scripting language written in Rust
//!
//! # Virtual Machine
//!
//! # Examples
//!

// TODO : This will be removed at a later stage, just have this here now
//          so that i dont have to worry about warnings.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]

use vm::opcode::Opcode;

struct VM
{
    opcodes     : Vec<Opcode>,
    opCodeIndex : usize,
}


impl VM {
    pub fn new() -> VM
    {
        VM { opcodes : Vec::new(), opCodeIndex : 0 }
    }

    pub fn setOpcodes(&mut self, opcodes : Vec<Opcode>)
    {

    }

    pub fn Move(&mut self){}
    pub fn LoadK(&mut self){}
    pub fn LoadBool(&mut self){}
    pub fn LoadNil(&mut self){}
    pub fn GetUpVal(&mut self){}
    pub fn GetGlobal(&mut self){}
    pub fn GetTable(&mut self){}
    pub fn SetGlobal(&mut self){}
    pub fn SetUpVal(&mut self){}
    pub fn SetTable(&mut self){}
    pub fn NewTable(&mut self){}
    pub fn _Self(&mut self){}
    pub fn Add(&mut self){}
    pub fn Sub(&mut self){}
    pub fn Mul(&mut self){}
    pub fn Div(&mut self){}
    pub fn Mod(&mut self){}
    pub fn Pow(&mut self){}
    pub fn Unm(&mut self){}
    pub fn Not(&mut self){}
    pub fn Len(&mut self){}
    pub fn Concat(&mut self){}
    pub fn Jmp(&mut self){}
    pub fn Eq(&mut self){}
    pub fn Lt(&mut self){}
    pub fn Le(&mut self){}
    pub fn Test(&mut self){}
    pub fn Testset(&mut self){}
    pub fn Call(&mut self){}
    pub fn TailCall(&mut self){}
    pub fn Return(&mut self){}
    pub fn ForLoop(&mut self){}
    pub fn ForPrep(&mut self){}
    pub fn TForLoop(&mut self){}
    pub fn SetList(&mut self){}
    pub fn Close(&mut self){}
    pub fn Closure(&mut self){}
    pub fn VarArg(&mut self){}

    pub fn run(&mut self)
    {
        let mut running : bool = true;
        self.opCodeIndex = 0;

        if self.opcodes.len() < 1 {
            return;
        }

        while(running)
        {
            match self.opcodes[self.opCodeIndex] {
                Opcode::Move        => self.Move(),
                Opcode::LoadK       => self.LoadK(),
                Opcode::LoadBool    => self.LoadBool(),
                Opcode::LoadNil     => self.LoadNil(),
                Opcode::GetUpVal    => self.GetUpVal(),
                Opcode::GetGlobal   => self.GetGlobal(),
                Opcode::GetTable    => self.GetTable(),
                Opcode::SetGlobal   => self.SetGlobal(),
                Opcode::SetUpVal    => self.SetUpVal(),
                Opcode::SetTable    => self.SetTable(),
                Opcode::NewTable    => self.NewTable(),
                Opcode::_Self       => self._Self(),
                Opcode::Add         => self.Add(),
                Opcode::Sub         => self.Sub(),
                Opcode::Mul         => self.Mul(),
                Opcode::Div         => self.Div(),
                Opcode::Mod         => self.Mod(),
                Opcode::Pow         => self.Pow(),
                Opcode::Unm         => self.Unm(),
                Opcode::Not         => self.Not(),
                Opcode::Len         => self.Len(),
                Opcode::Concat      => self.Concat(),
                Opcode::Jmp         => self.Jmp(),
                Opcode::Eq          => self.Eq(),
                Opcode::Lt          => self.Lt(),
                Opcode::Le          => self.Le(),
                Opcode::Test        => self.Test(),
                Opcode::Testset     => self.Testset(),
                Opcode::Call        => self.Call(),
                Opcode::TailCall    => self.TailCall(),
                Opcode::Return      => self.Return(),
                Opcode::ForLoop     => self.ForLoop(),
                Opcode::ForPrep     => self.ForPrep(),
                Opcode::TForLoop    => self.TForLoop(),
                Opcode::SetList     => self.SetList(),
                Opcode::Close       => self.Close(),
                Opcode::Closure     => self.Closure(),
                Opcode::VarArg      => self.VarArg(),
            }
        }
    }

}
