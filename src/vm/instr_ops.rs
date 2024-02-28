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

//                    R(A) := op R(B)
fn unary_arith(i: u32, vm: &mut dyn LuaVM, op: u8) {
    let (a, b) = (i.get_arg_a() + 1, i.get_arg_b() + 1);
    vm.push_value(b);
    vm.arith(op);
    vm.replace(a);
}

//                    if ((R[A] op R[B]) ~= k) then pc++
fn compare(i: u32, vm: &mut dyn LuaVM, op: u8) {
    let (a, b, k) = (i.get_arg_a() + 1, i.get_arg_b() + 1, i.get_arg_k());
    vm.get_rk(a);
    vm.get_rk(b);
    if vm.compare(-2, -1, op) != (k != 0) {
        vm.add_pc(1);
    }
    vm.pop(2);
}

//                    if ((R[A] op sB) ~= k) then pc++
fn compare_i(i: u32, vm: &mut dyn LuaVM, op: u8) {
    let (a, sb, k) = (i.get_arg_a() + 1, i.get_arg_sb(), i.get_arg_k());
    vm.get_rk(a);
    vm.push_integer(sb as i64);
    if vm.compare(-2, -1, op) != (k != 0) {
        vm.add_pc(1);
    }
    vm.pop(2);
}

//                    if ((R[A] op K[B]) ~= k) then pc++
fn compare_k(i: u32, vm: &mut dyn LuaVM, op: u8) {
    let (a, b, k) = (i.get_arg_a() + 1, i.get_arg_b(), i.get_arg_k());
    vm.get_rk(a);
    vm.get_const(b);
    if vm.compare(-2, -1, op) != (k != 0) {
        vm.add_pc(1);
    }
    vm.pop(2);
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

// OP_BANDK            A B C               R[A] := R[B] & K[C]:integer
pub fn band_k(i: u32, vm: &mut dyn LuaVM) {
    arith_k(i, vm, ArithOp::BAND as u8);
}

// OP_BORK             A B C               R[A] := R[B] | K[C]:integer
pub fn bor_k(i: u32, vm: &mut dyn LuaVM) {
    arith_k(i, vm, ArithOp::BOR as u8);
}

// OP_BXORK            A B C               R[A] := R[B] ~ K[C]:integer
pub fn bxor_k(i: u32, vm: &mut dyn LuaVM) {
    arith_k(i, vm, ArithOp::BXOR as u8);
}

// OP_SHLI             A B C               R[A] := sC << R[B]
pub fn shl_i(i: u32, vm: &mut dyn LuaVM) {
    arith_i(i, vm, ArithOp::SHL as u8);
}

// OP_SHRI             A B C               R[A] := R[B] >> sC
pub fn shr_i(i: u32, vm: &mut dyn LuaVM) {
    arith_i(i, vm, ArithOp::SHR as u8);
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

// OP_UNM              A B                 R[A] := -R[B]
pub fn unm(i: u32, vm: &mut dyn LuaVM) {
    unary_arith(i, vm, ArithOp::UNM as u8);
}

// OP_BNOT             A B                 R[A] := ~R[B]
pub fn bnot(i: u32, vm: &mut dyn LuaVM) {
    unary_arith(i, vm, ArithOp::BNOT as u8);
}

// OP_EQ               A B k               if ((R[A] == R[B]) ~= k) then pc++
pub fn eq(i: u32, vm: &mut dyn LuaVM) {
    compare(i, vm, CmpOp::EQ as u8);
}

// OP_LT               A B k               if ((R[A] <  R[B]) ~= k) then pc++
pub fn lt(i: u32, vm: &mut dyn LuaVM) {
    compare(i, vm, CmpOp::LT as u8);
}

// OP_LE               A B k               if ((R[A] <= R[B]) ~= k) then pc++
pub fn le(i: u32, vm: &mut dyn LuaVM) {
    compare(i, vm, CmpOp::LE as u8);
}

// OP_EQK              A B k               if ((R[A] == K[B]) ~= k) then pc++
pub fn eq_k(i: u32, vm: &mut dyn LuaVM) {
    compare_k(i, vm, CmpOp::EQ as u8);
}

// OP_EQI              A sB k              if ((R[A] == sB) ~= k) then pc++
pub fn eq_i(i: u32, vm: &mut dyn LuaVM) {
    compare_i(i, vm, CmpOp::EQ as u8);
}

// OP_LTI              A sB k              if ((R[A] < sB) ~= k) then pc++
pub fn lt_i(i: u32, vm: &mut dyn LuaVM) {
    compare_i(i, vm, CmpOp::LT as u8);
}

// OP_LEI              A sB k              if ((R[A] <= sB) ~= k) then pc++
pub fn le_i(i: u32, vm: &mut dyn LuaVM) {
    compare_i(i, vm, CmpOp::LE as u8);
}

// OP_GTI              A sB k              if ((R[A] > sB) ~= k) then pc++
pub fn gt_i(i: u32, vm: &mut dyn LuaVM) {
    compare_i(i, vm, CmpOp::GT as u8);
}

// OP_GEI              A sB k              if ((R[A] >= sB) ~= k) then pc++
pub fn ge_i(i: u32, vm: &mut dyn LuaVM) {
    compare_i(i, vm, CmpOp::GE as u8);
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
    fn test_band_k() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(0b00000001);
        vm.proto.constants.push(Constant::Integer(0b11111111));
        band_k(0b00000000_00000001_0_00000000_0011101, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 0b00000001)
    }

    #[test]
    fn test_bor_k() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(0b00000000);
        vm.proto.constants.push(Constant::Integer(0b11111111));
        bor_k(0b00000000_00000001_0_00000000_0011110, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 0b11111111)
    }

    #[test]
    fn test_bxor_k() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(0b00000001);
        vm.proto.constants.push(Constant::Integer(0b11111111));
        bxor_k(0b00000000_00000001_0_00000000_0011110, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 0b11111110)
    }

    #[test]
    fn test_shl_i() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(0b00001111);
        shl_i(0b00000001_00000001_0_00000000_0100000, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 0b00011110)
    }

    #[test]
    fn test_shr_i() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(0b00001111);
        shr_i(0b00000001_00000001_0_00000000_0100001, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 0b0000111)
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

    #[test]
    fn test_unm_integer() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        unm(0b00000000_00000001_0_00000000_0110001, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == -1)
    }

    #[test]
    fn test_unm_number() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_number(1.0);
        unm(0b00000000_00000001_0_00000000_0110001, &mut vm);
        assert!(vm.is_number(1));
        assert!(vm.to_number(1) == -1.0)
    }

    #[test]
    fn test_bnot() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(0b11111111);
        bnot(0b00000000_00000001_0_00000000_0110010, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == -256)
    }

    #[test]
    fn test_eq() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        vm.push_integer(1);
        assert!(vm.pc() == 0);
        eq(0b00000000_00000001_0_00000000_0111001, &mut vm);
        assert!(vm.pc() == 1);

        vm.push_integer(0);
        assert!(vm.pc() == 1);
        eq(0b00000000_00000010_0_00000001_0111001, &mut vm);
        assert!(vm.pc() == 1);
    }

    #[test]
    fn test_lt() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        vm.push_integer(2);
        assert!(vm.pc() == 0);
        lt(0b00000000_00000001_0_00000000_0111010, &mut vm);
        assert!(vm.pc() == 1);

        vm.push_integer(2);
        assert!(vm.pc() == 1);
        lt(0b00000000_00000010_0_00000001_0111010, &mut vm);
        assert!(vm.pc() == 1);

        vm.push_integer(0);
        assert!(vm.pc() == 1);
        lt(0b00000000_00000011_0_00000001_0111010, &mut vm);
        assert!(vm.pc() == 1);
    }

    #[test]
    fn test_le() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        vm.push_integer(2);
        assert!(vm.pc() == 0);
        le(0b00000000_00000001_0_00000000_0111011, &mut vm);
        assert!(vm.pc() == 1);

        vm.push_integer(2);
        assert!(vm.pc() == 1);
        le(0b00000000_00000010_0_00000001_0111011, &mut vm);
        assert!(vm.pc() == 2);

        vm.push_integer(0);
        assert!(vm.pc() == 2);
        le(0b00000000_00000011_0_00000001_0111011, &mut vm);
        assert!(vm.pc() == 2);
    }

    #[test]
    fn test_eq_k() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        vm.proto.constants.push(Constant::Integer(1));
        assert!(vm.pc() == 0);
        eq_k(0b00000000_00000000_0_00000000_0111100, &mut vm);
        assert!(vm.pc() == 1);

        vm.proto.constants.push(Constant::Integer(0));
        assert!(vm.pc() == 1);
        eq_k(0b00000000_00000001_0_00000001_0111100, &mut vm);
        assert!(vm.pc() == 1);
    }

    #[test]
    fn test_eq_i() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        assert!(vm.pc() == 0);
        eq_i(0b00000000_10000000_0_00000000_0111101, &mut vm);
        assert!(vm.pc() == 1);

        assert!(vm.pc() == 1);
        eq_i(0b00000000_01111111_0_00000000_0111101, &mut vm);
        assert!(vm.pc() == 1);
    }

    #[test]
    fn test_lt_i() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        assert!(vm.pc() == 0);
        lt_i(0b00000000_10000001_0_00000000_0111110, &mut vm);
        assert!(vm.pc() == 1);

        vm.push_integer(2);
        assert!(vm.pc() == 1);
        lt_i(0b00000000_10000001_0_00000001_0111110, &mut vm);
        assert!(vm.pc() == 1);

        assert!(vm.pc() == 1);
        lt_i(0b00000000__0_00000001_111110, &mut vm);
        assert!(vm.pc() == 1);
    }

    #[test]
    fn test_le_i() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        assert!(vm.pc() == 0);
        le_i(0b00000000_10000001_0_00000000_0111111, &mut vm);
        assert!(vm.pc() == 1);

        vm.push_integer(2);
        assert!(vm.pc() == 1);
        le_i(0b00000000_10000001_0_00000001_0111111, &mut vm);
        assert!(vm.pc() == 2);

        vm.push_integer(0);
        assert!(vm.pc() == 2);
        le_i(0b00000000_01111111_0_00000001_0111111, &mut vm);
        assert!(vm.pc() == 2);
    }

    #[test]
    fn test_gt_i() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        vm.push_integer(1);
        assert!(vm.pc() == 0);
        gt_i(0b00000000_01111111_0_00000000_1000000, &mut vm);
        assert!(vm.pc() == 1);

        vm.push_integer(1);
        assert!(vm.pc() == 1);
        gt_i(0b00000000_10000000_0_00000001_1000000, &mut vm);
        assert!(vm.pc() == 1);

        assert!(vm.pc() == 1);
        gt_i(0b00000000_10000001_0_00000001_1000000, &mut vm);
        assert!(vm.pc() == 1);
    }
}
