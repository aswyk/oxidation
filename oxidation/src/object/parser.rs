use super::*;


pub enum ExpressionKind {
    VVOID,          /* no value */
    VNIL,
    VTRUE,
    VFALSE,
    VK,             /* info = index of constant in `k' */
    VKNUM,          /* nval = numerical value */
    VLOCAL,         /* info = local register */
    VUPVAL,         /* info = index of upvalue in `upvalues' */
    VGLOBAL,        /* info = index of table; aux = index of global name in `k' */
    VINDEXED,       /* info = table register; aux = index register (or `k') */
    VJMP,           /* info = instruction pc */
    VRELOCABLE,     /* info = instruction pc */
    VNONRELOC,      /* info = result register */
    VCALL,          /* info = instruction pc */
    VVARARG,        /* info = instruction pc */
}

pub enum ExpressionDescriptionUVariable
{
    s { info: i32, aux: i32 },
    nval (lua_Number),
}

pub struct ExpressionDescription {
    k: ExpressionKind,
    u: ExpressionDescriptionUVariable,
    t: i32,
    f: i32,
}

pub struct UpValDescription {
    k: u8,
    info: u8,
}

pub struct FuncState {
    f: Proto,
    h: Table,
    prev: Box<FuncState>,
    ls: Box<LexState>,
    //lua state,
    bl: Box<BlockCnt>,
    pc: i32,
    lasttarget: i32,
    jpc: i32,
    freereg: i32,
    nk: i32,
    np: i32,
    nlocvars: i16,
    nactvar: u8,
    upvalues: Vec<UpValDescription>,// = Vec::with_capacity(1000),
    actvar: Vec<u16>,// = Vec::with_capacity(1000),
}
