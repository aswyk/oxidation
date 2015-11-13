
/*
chunk ::= {stat [`;´]} [laststat [`;´]]

	block ::= chunk

	stat ::=  varlist `=´ explist |
		 functioncall |
		 do block end |
		 while exp do block end |
		 repeat block until exp |
		 if exp then block {elseif exp then block} [else block] end |
		 for Name `=´ exp `,´ exp [`,´ exp] do block end |
		 for namelist in explist do block end |
		 function funcname funcbody |
		 local function Name funcbody |
		 local namelist [`=´ explist]

	laststat ::= return [explist] | break
	funcname ::= Name {`.´ Name} [`:´ Name]
	varlist ::= var {`,´ var}
	var ::=  Name | prefixexp `[´ exp `]´ | prefixexp `.´ Name
	namelist ::= Name {`,´ Name}
	explist ::= {exp `,´} exp
	exp ::=  nil | false | true | Number | String | `...´ | function |
		 prefixexp | tableconstructor | exp binop exp | unop exp
	prefixexp ::= var | functioncall | `(´ exp `)´
	functioncall ::=  prefixexp args | prefixexp `:´ Name args
	args ::=  `(´ [explist] `)´ | tableconstructor | String
	function ::= function funcbody
	funcbody ::= `(´ [parlist] `)´ block end
	parlist ::= namelist [`,´ `...´] | `...´
	tableconstructor ::= `{´ [fieldlist] `}´
	fieldlist ::= field {fieldsep field} [fieldsep]
	field ::= `[´ exp `]´ `=´ exp | Name `=´ exp | exp
	fieldsep ::= `,´ | `;´
	binop ::= `+´ | `-´ | `*´ | `/´ | `^´ | `%´ | `..´ |
		 `<´ | `<=´ | `>´ | `>=´ | `==´ | `~=´ |
		 and | or
	unop ::= `-´ | not | `#´
*/

use lexer::reader::*;

struct Block {
    lexer : FileReader,
}

impl Block {
    pub fn new(lex : FileReader) -> Block {
        Block { lexer : lex }
        //try!(b.read_line(&mut s));
    }

    pub fn stat(&mut self){} //::=  varlist `=´ explist |
		 //functioncall |
		 //do block end |
		 //while exp do block end |
		 //repeat block until exp |
		 //if exp then block {elseif exp then block} [else block] end |
		 //for Name `=´ exp `,´ exp [`,´ exp] do block end |
		 //for namelist in explist do block end |
		 //function funcname funcbody |
		 //local function Name funcbody |
		 //local namelist [`=´ explist]
    pub fn laststat(&mut self){} //::= return [explist] | break
	pub fn funcname(&mut self){} //::= Name {`.´ Name} [`:´ Name]
	pub fn varlist(&mut self){} //::= var {`,´ var}
	pub fn var(&mut self){} //::=  Name | prefixexp `[´ exp `]´ | prefixexp `.´ Name
	pub fn namelist(&mut self){} //::= Name {`,´ Name}
	pub fn explist(&mut self){} //::= {exp `,´} exp
	pub fn exp(&mut self){} //::=  nil | false | true | Number | String | `...´ | function |
		 //prefixexp | tableconstructor | exp binop exp | unop exp
	pub fn prefixexp(&mut self){} //::= var | functioncall | `(´ exp `)´
	pub fn functioncall(&mut self){} //::=  prefixexp args | prefixexp `:´ Name args
	pub fn args(&mut self){} //::=  `(´ [explist] `)´ | tableconstructor | String
	pub fn function(&mut self){} //::= function funcbody
	pub fn funcbody(&mut self){} //::= `(´ [parlist] `)´ block end
	pub fn parlist(&mut self){} //::= namelist [`,´ `...´] | `...´
	pub fn tableconstructor(){} //::= `{´ [fieldlist] `}´
	pub fn fieldlist(&mut self){} //::= field {fieldsep field} [fieldsep]
	pub fn field(&mut self){} //::= `[´ exp `]´ `=´ exp | Name `=´ exp | exp
	pub fn fieldsep(&mut self){} //::= `,´ | `;´
	pub fn binop(&mut self){} //::= `+´ | `-´ | `*´ | `/´ | `^´ | `%´ | `..´ |
		 //`<´ | `<=´ | `>´ | `>=´ | `==´ | `~=´ |
		 //and | or
	pub fn unop(&mut self){} //::= `-´ | not | `#´
}

pub fn processToken(block : &Block){
    
}
