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
            0 => Some(ArithOp::ADD),
            1 => Some(ArithOp::SUB),
            2 => Some(ArithOp::MUL),
            3 => Some(ArithOp::MOD),
            4 => Some(ArithOp::POW),
            5 => Some(ArithOp::DIV),
            6 => Some(ArithOp::IDIV),
            7 => Some(ArithOp::BAND),
            8 => Some(ArithOp::BOR),
            9 => Some(ArithOp::BXOR),
            10 => Some(ArithOp::SHL),
            11 => Some(ArithOp::SHR),
            12 => Some(ArithOp::UNM),
            13 => Some(ArithOp::BNOT),
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
            0 => Some(CmpOp::EQ),
            1 => Some(CmpOp::LT),
            2 => Some(CmpOp::LE),
            3 => Some(CmpOp::GT),
            4 => Some(CmpOp::GE),
            _ => None,
        }
    }
}
