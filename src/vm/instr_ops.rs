use crate::api::{
    op::{ArithOp, CmpOp},
    LuaVM,
};

use super::instruction::Instruction;

//                     R[A] := R[B] op R[C]
fn arith(i: u32, vm: &mut dyn LuaVM, op: u8) {
    let (a, b, c) = (i.get_arg_a() + 1, i.get_arg_b(), i.get_arg_c());

    vm.get_rk(b);
    vm.get_rk(c);
    vm.arith(op);
    vm.replace(a);
}

// OP_ADD              A B C               R[A] := R[B] + R[C]
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
#[cfg(test)]
mod tests {
    use crate::{api::LuaAPI, binary::chunk::Prototype, state::LuaState};

    use super::*;

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
        let result = vm.is_number(1);
        let value = vm.to_number(1);
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
}
