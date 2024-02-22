use crate::binary::object::Type;

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum LuaValue {
    Nil,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    Str(String),
}

impl LuaValue {
    pub fn type_id(&self) -> i8 {
        match self {
            LuaValue::Nil => Type::Nil as i8,
            LuaValue::Boolean(_) => Type::Boolean as i8,
            LuaValue::Number(_) => Type::Number as i8,
            LuaValue::Integer(_) => Type::Number as i8,
            LuaValue::Str(_) => Type::String as i8,
        }
    }

    pub fn to_boolean(&self) -> bool {
        match self {
            LuaValue::Nil => false,
            LuaValue::Boolean(b) => *b,
            _ => true,
        }
    }
}
