use crate::api::LuaVM;

use super::instruction::{Instruction, MAXARG_C};

const LFIELDS_PER_FLUSH: isize = 50;

// OP_NEWTABLE         A B C k             R[A] := {}
pub fn new_table(i: u32, vm: &mut dyn LuaVM) {
    let (a, mut b, mut c, k) = (
        i.get_arg_a() + 1,
        i.get_arg_b(),
        i.get_arg_c(),
        i.get_arg_k(),
    );
    if b > 0 {
        b = 1 << (b - 1);
    }
    if k != 0 {
        c += (vm.pc() as u32).get_arg_ax() * (MAXARG_C + 1)
    }
    vm.add_pc(1);
    vm.create_table(b as usize, c as usize);
    vm.replace(a);
}

// OP_GETTABLE         A B C               R[A] := R[B][R[C]]
pub fn get_table(i: u32, vm: &mut dyn LuaVM) {
    let (a, b, c) = (i.get_arg_a() + 1, i.get_arg_b() + 1, i.get_arg_c());
    vm.get_rk(c);
    vm.get_table(b);
    vm.replace(a);
}

// OP_GETI             A B C               R[A] := R[B][C]
pub fn get_i(i: u32, vm: &mut dyn LuaVM) {
    let (a, b, c) = (i.get_arg_a() + 1, i.get_arg_b() + 1, i.get_arg_c());
    vm.get_i(b, c as i64);
    vm.replace(a);
}
// OP_GETFIELD         A B C               R[A] := R[B][K[C]:string]
pub fn get_field(i: u32, vm: &mut dyn LuaVM) {
    let (a, b, c) = (i.get_arg_a() + 1, i.get_arg_b() + 1, i.get_arg_c());
    vm.get_const(c);
    let k = vm.to_string(-1);
    vm.pop(1);
    vm.get_field(b, k.as_str());
    vm.replace(a);
}

// OP_SETTABLE         A B C               R[A][R[B]] := RK(C)
pub fn set_table(i: u32, vm: &mut dyn LuaVM) {
    let (a, b, c) = (i.get_arg_a() + 1, i.get_arg_b(), i.get_arg_c());
    vm.get_rk(b);
    vm.get_rk(c);
    vm.set_table(a);
}

// OP_SETI             A B C               R[A][B] := RK(C)
pub fn set_i(i: u32, vm: &mut dyn LuaVM) {
    let (a, b, c) = (i.get_arg_a() + 1, i.get_arg_b(), i.get_arg_c());
    vm.get_const(c);
    vm.set_i(a, b as i64);
}

// OP_SETFIELD         A B C               R[A][K[B]:string] := RK(C)
pub fn set_field(i: u32, vm: &mut dyn LuaVM) {
    let (a, b, c) = (i.get_arg_a() + 1, i.get_arg_b(), i.get_arg_c());
    vm.get_const(b);
    let k = vm.to_string(-1);
    vm.pop(1);
    vm.get_const(c);
    vm.set_field(a, k.as_str());
}

// OP_SETLIST          A B C k             R[A][C+i] := R[A+i], 1 <= i <= B
pub fn set_list(i: u32, vm: &mut dyn LuaVM) {
    let (a, b, c, k) = (
        i.get_arg_a() + 1,
        i.get_arg_b(),
        i.get_arg_c(),
        i.get_arg_k(),
    );

    vm.check_stack(1);
    let mut idx = c;
    for j in 1..(b + 1) {
        idx += 1;
        vm.push_value(a + j);
        vm.set_i(a, idx as i64);
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::LuaAPI, binary::chunk::Prototype, state::LuaState};

    use super::*;

    #[test]
    fn test_table() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_nil();
        new_table(0b00000000_00000000_0_00000000_0010011, &mut vm);
        assert!(vm.is_table(1));
        vm.push_nil();
        new_table(0b00000001_00000001_1_00000001_0010011, &mut vm);
        assert!(vm.is_table(2));
        vm.push_integer(1);
        vm.push_string("1".to_string());
        set_table(0b00000011_00000010_0_00000001_0010000, &mut vm);
        get_table(0b00000010_00000001_0_00000000_0001100, &mut vm);
        assert!(vm.is_table(2));
        assert!(vm.is_string(1));
        assert!(vm.to_string(1) == "1".to_string());
    }
}
