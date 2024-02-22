use super::lua_value::LuaValue;

fn iadd(a: i64, b: i64) -> i64 {
    a + b
}

fn fadd(a: f64, b: f64) -> f64 {
    a + b
}

fn isub(a: i64, b: i64) -> i64 {
    a - b
}

fn fsub(a: f64, b: f64) -> f64 {
    a - b
}

fn imul(a: i64, b: i64) -> i64 {
    a * b
}

fn fmul(a: f64, b: f64) -> f64 {
    a * b
}

fn idiv(a: i64, b: i64) -> i64 {
    a / b
}

fn fdiv(a: f64, b: f64) -> f64 {
    a / b
}

fn imod(a: i64, b: i64) -> i64 {
    super::math::i_mod(a, b)
}

fn fmod(a: f64, b: f64) -> f64 {
    super::math::f_mod(a, b)
}

fn pow(a: f64, b: f64) -> f64 {
    a.powf(b)
}

fn div(a: f64, b: f64) -> f64 {
    a / b
}

fn iidiv(a: i64, b: i64) -> i64 {
    super::math::i_floor_div(a, b)
}

fn fidiv(a: f64, b: f64) -> f64 {
    super::math::f_floor_div(a, b)
}

fn band(a: i64, b: i64) -> i64 {
    a & b
}

fn bor(a: i64, b: i64) -> i64 {
    a | b
}

fn bxor(a: i64, b: i64) -> i64 {
    a ^ b
}

fn shl(a: i64, b: i64) -> i64 {
    super::math::shift_left(a, b)
}

fn shr(a: i64, b: i64) -> i64 {
    super::math::shift_right(a, b)
}

fn iunm(a: i64, _: i64) -> i64 {
    -a
}

fn funm(a: f64, _: f64) -> f64 {
    -a
}

fn bnot(a: i64, _: i64) -> i64 {
    !a
}

fn inone(_: i64, _: i64) -> i64 {
    0
}

fn fnone(_: f64, _: f64) -> f64 {
    0.0
}

pub const OPS: &'static [(fn(i64, i64) -> i64, fn(f64, f64) -> f64)] = &[
    (iadd, fadd),
    (isub, fsub),
    (imul, fmul),
    (imod, fmod),
    (inone, pow),
    (inone, div),
    (iidiv, fidiv),
    (band, fnone),
    (bor, fnone),
    (bxor, fnone),
    (shl, fnone),
    (shr, fnone),
    (iunm, funm),
    (bnot, fnone),
];

pub fn arith(a: &LuaValue, b: &LuaValue, op: u8) -> Option<LuaValue> {
    let (iop, fop) = OPS[op as usize];
    if fop == fnone {
        if let Some(x) = a.to_integer() {
            if let Some(y) = b.to_integer() {
                return Some(LuaValue::Integer(iop(x, y)));
            }
        }
    } else {
        if iop != inone {
            if let LuaValue::Integer(x) = a {
                if let LuaValue::Integer(y) = b {
                    return Some(LuaValue::Integer(iop(*x, *y)));
                }
            }
        }
        if let Some(x) = a.to_number() {
            if let Some(y) = b.to_number() {
                return Some(LuaValue::Number(fop(x, y)));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{api::op::ArithOp, state::lua_value::LuaValue};

    #[test]
    fn test_arith() {
        let cases: &[(LuaValue, LuaValue, u8, LuaValue)] = &[
            (
                LuaValue::Integer(1),
                LuaValue::Integer(2),
                ArithOp::ADD as u8,
                LuaValue::Integer(3),
            ),
            (
                LuaValue::Number(1.0),
                LuaValue::Number(2.0),
                ArithOp::ADD as u8,
                LuaValue::Number(3.0),
            ),
            (
                LuaValue::Integer(1),
                LuaValue::Number(2.0),
                ArithOp::ADD as u8,
                LuaValue::Number(3.0),
            ),
            (
                LuaValue::Number(2.0),
                LuaValue::Integer(1),
                ArithOp::SUB as u8,
                LuaValue::Number(1.0),
            ),
            (
                LuaValue::Integer(1),
                LuaValue::Number(2.0),
                ArithOp::SUB as u8,
                LuaValue::Number(-1.0),
            ),
            (
                LuaValue::Integer(1),
                LuaValue::Integer(2),
                ArithOp::SUB as u8,
                LuaValue::Integer(-1),
            ),
            (
                LuaValue::Integer(2),
                LuaValue::Integer(1),
                ArithOp::MUL as u8,
                LuaValue::Integer(2),
            ),
            (
                LuaValue::Number(2.0),
                LuaValue::Number(1.0),
                ArithOp::MUL as u8,
                LuaValue::Number(2.0),
            ),
            (
                LuaValue::Integer(2),
                LuaValue::Number(1.0),
                ArithOp::MUL as u8,
                LuaValue::Number(2.0),
            ),
            (
                LuaValue::Integer(2),
                LuaValue::Integer(1),
                ArithOp::MOD as u8,
                LuaValue::Integer(0),
            ),
            (
                LuaValue::Number(2.0),
                LuaValue::Number(1.0),
                ArithOp::MOD as u8,
                LuaValue::Number(0.0),
            ),
            (
                LuaValue::Integer(2),
                LuaValue::Number(1.0),
                ArithOp::MOD as u8,
                LuaValue::Number(0.0),
            ),
            (
                LuaValue::Number(2.0),
                LuaValue::Integer(1),
                ArithOp::MOD as u8,
                LuaValue::Number(0.0),
            ),
            (
                LuaValue::Integer(2),
                LuaValue::Integer(1),
                ArithOp::POW as u8,
                LuaValue::Number(2.0),
            ),
            (
                LuaValue::Number(2.0),
                LuaValue::Number(1.0),
                ArithOp::POW as u8,
                LuaValue::Number(2.0),
            ),
            (
                LuaValue::Integer(2),
                LuaValue::Number(1.0),
                ArithOp::POW as u8,
                LuaValue::Number(2.0),
            ),
            (
                LuaValue::Number(2.0),
                LuaValue::Integer(1),
                ArithOp::POW as u8,
                LuaValue::Number(2.0),
            ),
            (
                LuaValue::Number(2.0),
                LuaValue::Integer(1),
                ArithOp::DIV as u8,
                LuaValue::Number(2.0),
            ),
            (
                LuaValue::Integer(2),
                LuaValue::Number(1.0),
                ArithOp::DIV as u8,
                LuaValue::Number(2.0),
            ),
            (
                LuaValue::Number(2.0),
                LuaValue::Number(1.0),
                ArithOp::DIV as u8,
                LuaValue::Number(2.0),
            ),
            (
                LuaValue::Integer(2),
                LuaValue::Integer(1),
                ArithOp::IDIV as u8,
                LuaValue::Integer(2),
            ),
            (
                LuaValue::Number(2.0),
                LuaValue::Number(1.0),
                ArithOp::IDIV as u8,
                LuaValue::Number(2.0),
            ),
            (
                LuaValue::Integer(2),
                LuaValue::Number(1.0),
                ArithOp::IDIV as u8,
                LuaValue::Number(2.0),
            ),
            (
                LuaValue::Number(2.0),
                LuaValue::Integer(1),
                ArithOp::IDIV as u8,
                LuaValue::Number(2.0),
            ),
            (
                LuaValue::Integer(0b10),
                LuaValue::Integer(0b01),
                ArithOp::BAND as u8,
                LuaValue::Integer(0b00),
            ),
            (
                LuaValue::Integer(0b10),
                LuaValue::Integer(0b01),
                ArithOp::BOR as u8,
                LuaValue::Integer(0b11),
            ),
            (
                LuaValue::Integer(0b10),
                LuaValue::Integer(0b01),
                ArithOp::BXOR as u8,
                LuaValue::Integer(0b11),
            ),
            (
                LuaValue::Integer(0b10),
                LuaValue::Integer(0b01),
                ArithOp::SHL as u8,
                LuaValue::Integer(0b100),
            ),
            (
                LuaValue::Integer(0b10),
                LuaValue::Integer(0b01),
                ArithOp::SHR as u8,
                LuaValue::Integer(0b01),
            ),
            (
                LuaValue::Integer(2),
                LuaValue::Integer(1),
                ArithOp::UNM as u8,
                LuaValue::Integer(-2),
            ),
            (
                LuaValue::Number(2.0),
                LuaValue::Integer(1),
                ArithOp::UNM as u8,
                LuaValue::Number(-2.0),
            ),
            (
                LuaValue::Integer(0b10),
                LuaValue::Integer(1),
                ArithOp::BNOT as u8,
                LuaValue::Integer(-3),
            ),
        ];
        for case in cases {
            assert_eq!(
                arith(&case.0, &case.1, case.2),
                Some(case.3.clone()),
                "{:?}",
                case
            );
        }
    }
}
