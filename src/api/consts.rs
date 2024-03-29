/* 基本类型 */
pub const LUA_TNONE: i8 = -1;
pub const LUA_TNIL: i8 = 0;
pub const LUA_TBOOLEAN: i8 = 1;
pub const LUA_TLIGHTUSERDATA: i8 = 2;
pub const LUA_TNUMBER: i8 = 3;
pub const LUA_TSTRING: i8 = 4;
pub const LUA_TTABLE: i8 = 5;
pub const LUA_TFUNCTION: i8 = 6;
pub const LUA_TUSERDATA: i8 = 7;
pub const LUA_TTHREAD: i8 = 8;
pub const LUA_NUMTYPES: i8 = 9;

/* 扩展类型 */
pub const LUA_TUPVAL: i8 = LUA_NUMTYPES;
pub const LUA_TPROTO: i8 = LUA_NUMTYPES + 1;
pub const LUA_TDEADKEY: i8 = LUA_NUMTYPES + 2;
pub const LUA_TOLALTYPES: i8 = LUA_NUMTYPES + 3;

/* 算数运算符 */
pub const LUA_OPADD: u8 = 0; // +
pub const LUA_OPSUB: u8 = 1; // -
pub const LUA_OPMUL: u8 = 2; // *
pub const LUA_OPMOD: u8 = 3; // %
pub const LUA_OPPOW: u8 = 4; // ^
pub const LUA_OPDIV: u8 = 5; // /
pub const LUA_OPIDIV: u8 = 6; // //
pub const LUA_OPBAND: u8 = 7; // &
pub const LUA_OPBOR: u8 = 8; // |
pub const LUA_OPBXOR: u8 = 9; // ~
pub const LUA_OPSHL: u8 = 10; // <<
pub const LUA_OPSHR: u8 = 11; // >>
pub const LUA_OPUNM: u8 = 12; // -
pub const LUA_OPBNOT: u8 = 13; // ~

/* 比较运算符 */
pub const LUA_OPEQ: u8 = 0; // ==
pub const LUA_OPLT: u8 = 1; // <
pub const LUA_OPLE: u8 = 2; // <=
pub const LUA_OPGT: u8 = 3; // >
pub const LUA_OPGE: u8 = 4; // >=

/* 其他常量 */
pub const LUA_MINSTACK: usize = 20;
pub const LUAI_MAXSTACK: usize = 1000000;
pub const LUA_REGISTRYINDEX: isize = -(LUAI_MAXSTACK as isize) - 1000;
pub const LUA_RIDX_GLOBALS: isize = 2;
