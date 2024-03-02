use std::rc::Rc;

pub const LUA_SIGNATURE: [u8; 4] = [0x1b, 0x4c, 0x75, 0x61]; // "\x1bLua"
pub const LUAC_VERSION: u8 = 0x54;
pub const LUAC_FORMAT: u8 = 0;
pub const LUAC_DATA: [u8; 6] = [0x19, 0x93, 0x0d, 0x0a, 0x1a, 0x0a]; // "\x19\x93\r\n\x1a\n"
pub const INSTRUCTION_SIZE: u8 = 4;
pub const LUA_INTEGER_SIZE: u8 = 8;
pub const LUA_NUMBER_SIZE: u8 = 8;
pub const LUAC_INT: i64 = 0x5678;
pub const LUAC_NUM: f64 = 370.5;

#[allow(dead_code)]
#[derive(Debug)]
pub struct BinaryChunk {
    pub header: Header,
    pub size_of_upvalues: u8,
    pub function: Prototype,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Header {
    pub lua_signature: [u8; 4],
    pub luac_version: u8,
    pub luac_format: u8,
    pub luac_data: [u8; 6],
    pub size_of_instruction: u8,
    pub size_of_lua_int: u8,
    pub size_of_lua_num: u8,
    pub luac_int: i64,
    pub luac_num: f64,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Prototype {
    pub source: Option<String>, // debug
    pub line_defined: usize,
    pub last_line_defined: usize,
    pub num_params: u8,
    pub is_vararg: u8,
    pub max_stack_size: u8,
    pub code: Vec<u32>,
    pub constants: Vec<Constant>,
    pub upvalues: Vec<Upvalue>,
    pub protos: Vec<Rc<Prototype>>,
    pub line_info: Vec<i8>,              // debug
    pub abs_line_info: Vec<AbsLineInfo>, // debug
    pub loc_vars: Vec<LocVar>,           // debug
    pub upvalue_names: Vec<String>,      // debug
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Upvalue {
    pub instack: u8,
    pub idx: u8,
    pub kind: u8,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AbsLineInfo {
    pub pc: usize,
    pub line: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LocVar {
    pub var_name: String,
    pub start_pc: usize,
    pub end_pc: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Constant {
    Nil,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    Str(String),
}
