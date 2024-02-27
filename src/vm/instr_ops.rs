use crate::api::{op::ArithOp, LuaVM};

use super::instruction::Instruction;

//                     R[A] := R[B] op R[C]
fn arith(i: u32, vm: &mut dyn LuaVM, op: u8) {
    let (a, b, c) = (i.get_arg_a() + 1, i.get_arg_b(), i.get_arg_c());

    vm.get_rk(b);
    vm.get_rk(c);
    vm.arith(op);
    vm.replace(a);
}

//                     R[A] := R[B] + sC
fn arith_i(i: u32, vm: &mut dyn LuaVM, op: u8) {
    let (a, b, c) = (i.get_arg_a() + 1, i.get_arg_b(), i.get_arg_c());

    vm.get_rk(b);
    vm.push_integer(c as i64);
    vm.arith(op);
    vm.replace(a);
}

//                     R[A] := R[B] + K[C]:number
fn arith_k(i: u32, vm: &mut dyn LuaVM, op: u8) {
    let (a, b, c) = (i.get_arg_a() + 1, i.get_arg_b(), i.get_arg_c());

    vm.get_rk(b);
    vm.get_const(c);
    vm.arith(op);
    vm.replace(a);
}

// OP_ADDI             A B sC              R[A] := R[B] + sC
fn add_i(i: u32, vm: &mut dyn LuaVM) {
    arith_i(i, vm, ArithOp::ADD as u8);
}

// OP_ADDK             A B C               R[A] := R[B] + K[C]:number
pub fn add_k(i: u32, vm: &mut dyn LuaVM) {
    arith_k(i, vm, ArithOp::ADD as u8);
}

// OP_SUBK             A B C               R[A] := R[B] - K[C]:number
pub fn sub_k(i: u32, vm: &mut dyn LuaVM) {
    arith_k(i, vm, ArithOp::SUB as u8);
}

// OP_MULK             A B C               R[A] := R[B] * K[C]:number
pub fn mul_k(i: u32, vm: &mut dyn LuaVM) {
    arith_k(i, vm, ArithOp::MUL as u8);
}

// OP_MODK             A B C               R[A] := R[B] % K[C]:number
pub fn _mod_k(i: u32, vm: &mut dyn LuaVM) {
    arith_k(i, vm, ArithOp::MOD as u8);
}

// OP_POWK             A B C               R[A] := R[B] ^ K[C]:number
pub fn pow_k(i: u32, vm: &mut dyn LuaVM) {
    arith_k(i, vm, ArithOp::POW as u8);
}

// OP_DIVK             A B C               R[A] := R[B] / K[C]:number
pub fn div_k(i: u32, vm: &mut dyn LuaVM) {
    arith_k(i, vm, ArithOp::DIV as u8);
}

// OP_IDIVK            A B C               R[A] := R[B] // K[C]:number
pub fn idiv_k(i: u32, vm: &mut dyn LuaVM) {
    arith_k(i, vm, ArithOp::IDIV as u8);
}

// OP_ADD              A B C               R[A] := R[B] + K[C]:number
pub fn add(i: u32, vm: &mut dyn LuaVM) {
    arith(i, vm, ArithOp::ADD as u8);
}

// OP_SUB              A B C               R[A] := R[B] - R[C]
pub fn sub(i: u32, vm: &mut dyn LuaVM) {
    arith(i, vm, ArithOp::SUB as u8);
}

// OP_MUL              A B C               R[A] := R[B] * R[C]
pub fn mul(i: u32, vm: &mut dyn LuaVM) {
    arith(i, vm, ArithOp::MUL as u8);
}

// OP_MOD              A B C               R[A] := R[B] % R[C]
pub fn _mod(i: u32, vm: &mut dyn LuaVM) {
    arith(i, vm, ArithOp::MOD as u8);
}

// OP_POW              A B C               R[A] := R[B] ^ R[C]
pub fn pow(i: u32, vm: &mut dyn LuaVM) {
    arith(i, vm, ArithOp::POW as u8);
}

// OP_DIV              A B C               R[A] := R[B] / R[C]
pub fn div(i: u32, vm: &mut dyn LuaVM) {
    arith(i, vm, ArithOp::DIV as u8);
}

// OP_IDIV             A B C               R[A] := R[B] // R[C]
pub fn idiv(i: u32, vm: &mut dyn LuaVM) {
    arith(i, vm, ArithOp::IDIV as u8);
}

// OP_BAND             A B C               R[A] := R[B] & R[C]
pub fn band(i: u32, vm: &mut dyn LuaVM) {
    arith(i, vm, ArithOp::BAND as u8);
}

// OP_BOR              A B C               R[A] := R[B] | R[C]
pub fn bor(i: u32, vm: &mut dyn LuaVM) {
    arith(i, vm, ArithOp::BOR as u8);
}

// OP_BXOR             A B C               R[A] := R[B] ~ R[C]
pub fn bxor(i: u32, vm: &mut dyn LuaVM) {
    arith(i, vm, ArithOp::BXOR as u8);
}

// OP_SHL              A B C               R[A] := R[B] << R[C]
pub fn shl(i: u32, vm: &mut dyn LuaVM) {
    arith(i, vm, ArithOp::SHL as u8);
}

// OP_SHR              A B C               R[A] := R[B] >> R[C]
pub fn shr(i: u32, vm: &mut dyn LuaVM) {
    arith(i, vm, ArithOp::SHR as u8);
}
#[cfg(test)]
mod tests {
    use crate::{
        api::LuaAPI,
        binary::chunk::{Constant, Prototype},
        state::LuaState,
    };

    use super::*;

    #[test]
    fn test_add_i_integer() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        add_i(0b00000010_00000001_0_00000000_0010101, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 3)
    }

    #[test]
    fn test_add_i_number() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_number(1.1);
        add_i(0b00000010_00000001_0_00000000_0010101, &mut vm);
        assert!(vm.is_number(1));
        assert!(numbers_are_equal(vm.to_number(1), 3.1, 0.01))
    }

    #[test]
    fn test_add_k_integer() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        vm.proto.constants.push(Constant::Integer(2));
        add_k(0b00000000_00000001_0_00000000_0010110, &mut vm);
        assert!(vm.is_number(1));
        assert!(vm.to_integer(1) == 3)
    }

    #[test]
    fn test_add_k_number() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_number(1.1);
        vm.proto.constants.push(Constant::Number(2f64));
        add_k(0b00000000_00000001_0_00000000_0010110, &mut vm);
        assert!(vm.is_number(1));
        assert!(numbers_are_equal(vm.to_number(1), 3.1, 0.01))
    }

    #[test]
    fn test_sub_k_integer() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        vm.proto.constants.push(Constant::Integer(2));
        sub_k(0b00000000_00000001_0_00000000_0010111, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == -1)
    }

    #[test]
    fn test_sub_k_number() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_number(1.1);
        vm.proto.constants.push(Constant::Number(2.2));
        sub_k(0b00000000_00000001_0_00000000_0010111, &mut vm);
        assert!(vm.is_number(1));
        assert!(numbers_are_equal(vm.to_number(1), -1.1, 0.01))
    }

    #[test]
    fn test_mul_k_integer() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        vm.proto.constants.push(Constant::Integer(2));
        mul_k(0b00000000_00000001_0_00000000_0011000, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 2)
    }

    #[test]
    fn test_mul_k_number() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_number(1.1);
        vm.proto.constants.push(Constant::Number(2.2));
        mul_k(0b00000000_00000001_0_00000000_0011000, &mut vm);
        assert!(vm.is_number(1));
        assert!(numbers_are_equal(vm.to_number(1), 2.42, 0.01))
    }

    #[test]
    fn test_mod_k_integer() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        vm.proto.constants.push(Constant::Integer(2));
        _mod_k(0b00000000_00000001_0_00000000_0011001, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 1)
    }
    #[test]
    fn test_mod_k_number() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_number(1.1);
        vm.proto.constants.push(Constant::Number(2.2));
        _mod_k(0b00000000_00000001_0_00000000_0011001, &mut vm);
        assert!(vm.is_number(1));
        assert!(numbers_are_equal(vm.to_number(1), 1.1, 0.01))
    }

    #[test]
    fn test_pow_k() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(2);
        vm.proto.constants.push(Constant::Integer(2));
        pow_k(0b00000000_00000001_0_00000000_0101010, &mut vm);
        assert!(vm.is_number(1));
        assert!(vm.to_number(1) == 4f64)
    }

    #[test]
    fn test_div_k() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(2);
        vm.proto.constants.push(Constant::Integer(2));
        div_k(0b00000000_00000001_0_00000000_0101011, &mut vm);
        assert!(vm.is_number(1));
        assert!(vm.to_number(1) == 1f64)
    }

    #[test]
    fn test_idiv_k_integer() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(2);
        vm.proto.constants.push(Constant::Integer(2));
        idiv_k(0b00000000_00000001_0_00000000_0101100, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 1)
    }

    #[test]
    fn test_idiv_k_number() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_number(2.2);
        vm.proto.constants.push(Constant::Number(2.2));
        idiv_k(0b00000000_00000001_0_00000000_0101100, &mut vm);
        assert!(vm.is_number(1));
        assert!(numbers_are_equal(vm.to_number(1), 1f64, 0.01))
    }

    #[test]
    fn test_add_integer() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        vm.push_integer(2);
        add(0b00000010_00000001_0_00000000_0100010, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 3)
    }

    #[test]
    fn test_add_number() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_number(1.1);
        vm.push_number(2.2);
        add(0b00000010_00000001_0_00000000_0100010, &mut vm);
        assert!(vm.is_number(1));
        assert!(numbers_are_equal(vm.to_number(1), 3.3, 0.01))
    }

    #[test]
    fn test_sub_integer() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        vm.push_integer(2);
        sub(0b00000010_00000001_0_00000000_0100011, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == -1)
    }

    #[test]
    fn test_sub_number() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_number(1.1);
        vm.push_number(2.2);
        sub(0b00000010_00000001_0_00000000_0100011, &mut vm);
        assert!(vm.is_number(1));
        assert!(numbers_are_equal(vm.to_number(1), -1.1, 0.01))
    }

    #[test]
    fn test_mul_integer() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        vm.push_integer(2);
        mul(0b00000010_00000001_0_00000000_0100100, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 2)
    }

    #[test]
    fn test_mul_number() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_number(1.1);
        vm.push_number(2.2);
        mul(0b00000010_00000001_0_00000000_0100100, &mut vm);
        assert!(vm.is_number(1));
        assert!(numbers_are_equal(vm.to_number(1), 2.42, 0.01))
    }

    #[test]
    fn test_mod_integer() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        vm.push_integer(2);
        _mod(0b00000010_00000001_0_00000000_0100101, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 1)
    }
    #[test]
    fn test_mod_number() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_number(1.1);
        vm.push_number(2.2);
        _mod(0b00000010_00000001_0_00000000_0100101, &mut vm);
        assert!(vm.is_number(1));
        assert!(numbers_are_equal(vm.to_number(1), 1.1, 0.01))
    }

    #[test]
    fn test_pow() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(2);
        vm.push_integer(2);
        pow(0b00000010_00000001_0_00000000_0100110, &mut vm);
        assert!(vm.is_number(1));
        assert!(vm.to_number(1) == 4f64)
    }

    #[test]
    fn test_div() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(2);
        vm.push_integer(2);
        div(0b00000010_00000001_0_00000000_0100111, &mut vm);
        assert!(vm.is_number(1));
        assert!(vm.to_number(1) == 1f64)
    }

    #[test]
    fn test_idiv_integer() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(2);
        vm.push_integer(2);
        idiv(0b00000010_00000001_0_00000000_0101000, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 1)
    }

    #[test]
    fn test_idiv_number() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_number(2.2);
        vm.push_number(2.2);
        idiv(0b00000010_00000001_0_00000000_0101000, &mut vm);
        assert!(vm.is_number(1));
        assert!(numbers_are_equal(vm.to_number(1), 1f64, 0.01))
    }

    fn numbers_are_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    #[test]
    fn test_band() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(0b00000001);
        vm.push_integer(0b11111111);
        band(0b00000010_00000001_0_00000000_0101001, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 0b00000001)
    }

    #[test]
    fn test_bor() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(0b00000000);
        vm.push_integer(0b11111111);
        bor(0b00000010_00000001_0_00000000_0101010, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 0b11111111)
    }

    #[test]
    fn test_bxor() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(0b00000001);
        vm.push_integer(0b11111111);
        bxor(0b00000010_00000001_0_00000000_0101011, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 0b11111110)
    }

    #[test]
    fn test_shl() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(0b00001111);
        vm.push_integer(1);
        shl(0b00000010_00000001_0_00000000_0101100, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 0b00011110)
    }

    #[test]
    fn test_shr() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(0b00001111);
        vm.push_integer(1);
        shr(0b00000010_00000001_0_00000000_0101101, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 0b0000111)
    }
}
