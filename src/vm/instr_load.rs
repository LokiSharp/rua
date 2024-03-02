use crate::api::LuaVM;

use super::instruction::Instruction;

// OP_LOADI            A sBx               R[A] := sBx
pub fn load_i(i: u32, vm: &mut dyn LuaVM) {
    let (a, sbx) = (i.get_arg_a() + 1, i.get_arg_sbx());
    vm.push_integer(sbx as i64);
    vm.replace(a);
}

// OP_LOADF            A sBx               R[A] := (lua_Number)sBx
pub fn load_f(i: u32, vm: &mut dyn LuaVM) {
    let (a, sbx) = (i.get_arg_a() + 1, i.get_arg_sbx());
    vm.push_number(sbx as f64);
    vm.replace(a);
}

// OP_LOADK            A Bx                R[A] := K[Bx]
pub fn load_k(i: u32, vm: &mut dyn LuaVM) {
    let (a, bx) = (i.get_arg_a() + 1, i.get_arg_bx());
    vm.get_const(bx);
    vm.replace(a);
}

// OP_LOADKX           A                   R[A] := K[extra arg]
pub fn load_kx(i: u32, vm: &mut dyn LuaVM) {
    let a = i.get_arg_a() + 1;
    let ax = vm.fetch().get_arg_ax();
    vm.get_const(ax);
    vm.replace(a);
}

// OP_LOADFALSE        A                   R[A] := false
pub fn load_false(i: u32, vm: &mut dyn LuaVM) {
    let a = i.get_arg_a() + 1;
    vm.push_boolean(false);
    vm.replace(a);
}

// OP_LFALSESKIP       A                   R[A] := false; pc++ (*)
pub fn load_l_false_skip(i: u32, vm: &mut dyn LuaVM) {
    let a = i.get_arg_a() + 1;
    vm.push_boolean(false);
    vm.replace(a);
    vm.add_pc(1);
}

// OP_LOADTRUE         A                   R[A] := true
pub fn load_true(i: u32, vm: &mut dyn LuaVM) {
    let a = i.get_arg_a() + 1;
    vm.push_boolean(true);
    vm.replace(a);
}

// OP_LOADNIL          A B                 R[A], R[A+1], ..., R[A+B] := nil
pub fn load_nil(i: u32, vm: &mut dyn LuaVM) {
    let (a, mut b) = (i.get_arg_a() + 1, i.get_arg_b());
    vm.push_nil();
    while b >= 0 {
        vm.copy(-1, a + b);
        b -= 1
    }
    vm.pop(1);
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        api::LuaAPI,
        binary::chunk::{Constant, Prototype},
        state::LuaState,
    };

    use super::*;

    #[test]
    fn test_load_i() {
        let mut vm = LuaState::new();
        vm.push_integer(0);
        load_i(0b10000000000000000_00000000_0000001, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 1);
    }

    #[test]
    fn test_load_f() {
        let mut vm = LuaState::new();
        vm.push_integer(0);
        load_f(0b10000000000000000_00000000_0000010, &mut vm);
        assert!(vm.is_number(1));
        assert!(vm.to_number(1) == 1.0);
    }

    #[test]
    fn tets_load_k() {
        let mut proto = Prototype::default();
        proto.constants.push(Constant::Integer(2));
        let mut vm = LuaState::new_with_proto(Rc::new(proto));

        vm.push_integer(0);
        load_k(0b00000000000000000_00000000_0000011, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 2);
    }

    #[test]
    fn test_load_kx() {
        let mut proto = Prototype::default();
        proto.constants.push(Constant::Integer(2));
        proto.code.push(0b00000000000000000_00000000_0000000);
        let mut vm = LuaState::new_with_proto(Rc::new(proto));
        vm.push_integer(0);

        load_kx(0b00000000_00000000_0_00000000_0000100, &mut vm);
        assert!(vm.is_integer(1));
        assert!(vm.to_integer(1) == 2);
    }

    #[test]
    fn test_load_false() {
        let mut vm = LuaState::new();
        vm.push_integer(0);
        load_false(0b00000000_00000000_0_00000000_0000101, &mut vm);
        assert!(vm.is_boolean(1));
        assert!(vm.to_boolean(1) == false);
    }

    #[test]
    fn test_load_l_false_skip() {
        let mut vm = LuaState::new();
        vm.push_integer(0);
        load_l_false_skip(0b00000000_00000000_0_00000000_0000110, &mut vm);
        assert!(vm.is_boolean(1));
        assert!(vm.to_boolean(1) == false);
        assert!(vm.pc() == 1);
    }

    #[test]
    fn test_load_true() {
        let mut vm = LuaState::new();
        vm.push_integer(0);
        load_true(0b00000000_00000000_0_00000000_0000111, &mut vm);
        assert!(vm.is_boolean(1));
        assert!(vm.to_boolean(1) == true);
    }

    #[test]
    fn test_load_nil() {
        let mut vm = LuaState::new();
        for _ in 1..=10 {
            vm.push_integer(0);
        }
        load_nil(0b00000000_00000101_0_00000000_0001000, &mut vm);
        for i in 1..=5 {
            assert!(vm.is_nil(i));
        }
    }
}
