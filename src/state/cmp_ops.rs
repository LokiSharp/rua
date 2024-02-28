use super::lua_value::LuaValue;
use crate::api::op::CmpOp;

pub fn compare(a: &LuaValue, b: &LuaValue, op: u8) -> Option<bool> {
    match CmpOp::from_u8(op) {
        Some(CmpOp::EQ) => Some(eq(a, b)),
        Some(CmpOp::LT) => lt(a, b),
        Some(CmpOp::LE) => le(a, b),
        Some(CmpOp::GT) => gt(a, b),
        Some(CmpOp::GE) => ge(a, b),
        _ => None,
    }
}

macro_rules! cmp {
    ($a:ident $op:tt $b:ident) => {
        match $a {
            LuaValue::Str(x) => match $b {
                LuaValue::Str(y) => Some(x $op y),
                _ => None,
            },
            LuaValue::Integer(x) => match $b {
                LuaValue::Integer(y) => Some(x $op y),
                LuaValue::Number(y) => Some((*x as f64) $op *y),
                _ => None,
            },
            LuaValue::Number(x) => match $b {
                LuaValue::Number(y) => Some(x $op y),
                LuaValue::Integer(y) => Some(*x $op (*y as f64)),
                _ => None,
            },
            _ => None,
        }
    }
}

fn eq(a: &LuaValue, b: &LuaValue) -> bool {
    if let Some(x) = cmp!(a == b) {
        x
    } else {
        match (a, b) {
            (LuaValue::Nil, LuaValue::Nil) => true,
            (LuaValue::Boolean(x), LuaValue::Boolean(y)) => x == y,
            _ => false,
        }
    }
}

fn lt(a: &LuaValue, b: &LuaValue) -> Option<bool> {
    cmp!(a < b)
}

fn le(a: &LuaValue, b: &LuaValue) -> Option<bool> {
    cmp!(a <= b)
}

fn gt(a: &LuaValue, b: &LuaValue) -> Option<bool> {
    cmp!(a > b)
}

fn ge(a: &LuaValue, b: &LuaValue) -> Option<bool> {
    cmp!(a >= b)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compare_eq() {
        let a = LuaValue::Number(10.0);
        let b = LuaValue::Number(10.0);
        let op = CmpOp::EQ as u8;
        assert_eq!(compare(&a, &b, op), Some(true));
    }

    #[test]
    fn test_compare_lt() {
        let a = LuaValue::Integer(5);
        let b = LuaValue::Number(10.0);
        let op = CmpOp::LT as u8;
        assert_eq!(compare(&a, &b, op), Some(true));
    }

    #[test]
    fn test_compare_le() {
        let a = LuaValue::Number(10.0);
        let b = LuaValue::Integer(5);
        let op = CmpOp::LE as u8;
        assert_eq!(compare(&a, &b, op), Some(false));
    }

    #[test]
    fn test_compare_invalid_op() {
        let a = LuaValue::Boolean(true);
        let b = LuaValue::Boolean(false);
        let op = 100; // Invalid op
        assert_eq!(compare(&a, &b, op), None);
    }
}
