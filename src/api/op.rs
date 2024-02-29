use super::consts::*;

#[allow(dead_code)]
pub enum ArithOp {
    ADD,
    SUB,
    MUL,
    MOD,
    POW,
    DIV,
    IDIV,
    BAND,
    BOR,
    BXOR,
    SHL,
    SHR,
    UNM,
    BNOT,
}

#[allow(dead_code)]
impl ArithOp {
    pub fn from_u8(value: u8) -> Option<ArithOp> {
        match value {
            LUA_OPADD => Some(ArithOp::ADD),
            LUA_OPSUB => Some(ArithOp::SUB),
            LUA_OPMUL => Some(ArithOp::MUL),
            LUA_OPMOD => Some(ArithOp::MOD),
            LUA_OPPOW => Some(ArithOp::POW),
            LUA_OPDIV => Some(ArithOp::DIV),
            LUA_OPIDIV => Some(ArithOp::IDIV),
            LUA_OPBAND => Some(ArithOp::BAND),
            LUA_OPBOR => Some(ArithOp::BOR),
            LUA_OPBXOR => Some(ArithOp::BXOR),
            LUA_OPSHL => Some(ArithOp::SHL),
            LUA_OPSHR => Some(ArithOp::SHR),
            LUA_OPUNM => Some(ArithOp::UNM),
            LUA_OPBNOT => Some(ArithOp::BNOT),
            _ => None,
        }
    }
}

pub enum CmpOp {
    EQ,
    LT,
    LE,
    GT,
    GE,
}

impl CmpOp {
    pub fn from_u8(value: u8) -> Option<CmpOp> {
        match value {
            LUA_OPEQ => Some(CmpOp::EQ),
            LUA_OPLT => Some(CmpOp::LT),
            LUA_OPLE => Some(CmpOp::LE),
            LUA_OPGT => Some(CmpOp::GT),
            LUA_OPGE => Some(CmpOp::GE),
            _ => None,
        }
    }
}
