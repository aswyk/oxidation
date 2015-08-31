#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
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
use std::error::Error;
use object::*;
use lexer::reader::*;
use super::tokens::Token;

///
///
/// Rules for Statements
///
///

fn block_follow(token: &Token) -> bool
{
    match *token {
        Token::Else => { return true; }
        Token::Elseif => {return true; }
        Token::End => { return true; }
        Token::Until => { return true; }
        _ => { return false;}
    }
}



pub struct Parser {
    lexer : FileReader,
    currentToken : Option<Token>,
}

pub struct TVM;

impl Parser {
    pub fn new(mut lex : FileReader) -> Option<Parser> {

        let mut tok = lex.peek_token();

        match tok {
            Ok(mut t) => {
                return Some( Parser {
                                lexer : lex,
                                currentToken : Some(t.clone())
                                }
                            );
            },
            Err(e) => {
                return None;
            }
        }

        //try!(b.read_line(&mut s));
    }

    pub fn loadPeekToken(&mut self) -> bool {

        let tok = self.lexer.peek_token();
        match tok {
            Ok(t) =>
            {
                self.currentToken = Some(t.clone());
                return true;
            },
            Err(e) => {
                match e.description() {
                    "EOF" =>
                    {
                        self.currentToken = None;//Token::EOF;
                        return true;
                    },
                    err => panic!("{}", err)
                }
            }
        }
        return false;
    }

    pub fn loadNextToken(&mut self) -> bool {

        let tok = self.lexer.next_token();
        match tok {
            Ok(t) =>
            {
                self.currentToken = Some(t.clone());
                return true;
            },
            Err(e) => {
                match e.description() {
                    "EOF" =>
                    {
                        self.currentToken = Some(Token::EOF);
                        return true;
                    },
                    err => panic!("{}", err)
                }
            }
        }
        return false;
    }
    // main parser entry point
    //Proto *luaY_parser (lua_State *L, ZIO *z, Mbuffer *buff, const char *name) {
    pub fn parse(&mut self, vm: Option<TVM> ) -> i32 {

        println!("Starting Parsing");

        //struct LexState lexstate;
        //struct FuncState funcstate;
        //lexstate.buff = buff;
        //luaX_setinput(L, &lexstate, z, luaS_new(L, name));
        //open_func(&lexstate, &funcstate);
        //funcstate.f->is_vararg = VARARG_ISVARARG;  /* main func. is always vararg */
        //luaX_next(&lexstate);  /* read first token */
        self.chunk();//chunk(&lexstate);
        //check(&lexstate, TK_EOS);
        //close_func(&lexstate);
        //lua_assert(funcstate.prev == NULL);
        //lua_assert(funcstate.f->nups == 0);
        //lua_assert(lexstate.fs == NULL);
        //return funcstate.f;


        println!("Completed Parsing");

        return 3;
    }

    pub fn chunk(&mut self) {
        println!("Parser::chuck");
        /* chunk -> { stat [`;'] } */
        let mut islast = false;
        let mut cToken : & Token = &self.currentToken.clone().unwrap().clone();

        //println!("token = {}", cToken);
        //enterlevel(self); // code gen
        //while !islast && block_follow(self.getNextToken()) {
        //while !islast && block_follow(cToken) {
        loop {
            println!("peek = {}", self.lexer.peek_token().unwrap().clone());
            let tok = self.lexer.next_token();
            match tok {
                Ok(mut t) => { println!("token = {} - peek = {}", t, self.lexer.peek_token().unwrap().clone()); },
                Err(e) => {}
            }
        }
            //islast = self.statement();
            //testnext(ls, ';');
            //lua_assert(ls->fs->f->maxstacksize >= ls->fs->freereg && ls->fs->freereg >= ls->fs->nactvar);
            //ls->fs->freereg = ls->fs->nactvar;  /* free registers */
        //}
        //leavelevel(ls);   // code gen
    }

    pub fn block(&mut self) {
        println!("Parser::block");
        /* block -> chunk */
        //FuncState *fs = ls->fs;
        //BlockCnt bl;
        //enterblock(fs, &bl, 0);
        self.chunk();
        //lua_assert(bl.breaklist == NO_JUMP);
        //leaveblock(fs);
    }

    pub fn statement(&mut self) -> bool{
        println!("Parser::Statement");
        let line = self.lexer.line_number;

        match self.lexer.peek_token() {
            Ok(t) => {
                match t {
                    _ => { return false; }
                }
            },
            Err(e) => {
              match e.description() {
                "EOF" => { return false; },
                err => { panic!("{}", err); }
              }
          },
      }
  }
        //::=  varlist `=´ explist |
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
    pub fn ifstat(&mut self, line: u32) {

    }

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

/*
pub fn processToken(block : &Block){

}
*/
