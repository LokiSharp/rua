use std::f32::consts::E;

use crate::api::{
    op::{ArithOp, CmpOp},
    r#type::Type,
    LuaVM,
};

use super::instruction::Instruction;

// OP_FORLOOP          A Bx                update counters; if loop continues then pc-=Bx;
pub fn for_loop(i: u32, vm: &mut dyn LuaVM) {
    let (a, bx) = (i.get_arg_a() + 1, i.get_arg_bx());

    if vm.is_integer(a) {
        let count = vm.to_integer(a + 1);
        if count > 0 {
            let step = vm.to_integer(a + 2);
            let mut idx = vm.to_integer(a);
            vm.push_integer(count - 1);
            vm.replace(a + 1);
            idx += step;
            vm.push_integer(idx);
            vm.replace(a);
            vm.push_integer(idx);
            vm.replace(a + 3);
            vm.add_pc(-bx as isize);
        }
    } else if is_number_for_loop(vm, a) {
        vm.add_pc(-bx as isize);
    }
}
// OP_FORPREP          A Bx                <check values and prepare counters>; if not to run then pc+=Bx+1;
pub fn for_prep(i: u32, vm: &mut dyn LuaVM) {
    let (a, bx) = (i.get_arg_a() + 1, i.get_arg_bx());

    if vm.is_integer(a) && vm.is_integer(a + 1) {
        let init = vm.to_integer(a);
        let limit = vm.to_integer(a + 1);
        let step = vm.to_integer(a + 2);

        if step == 0 {
            panic!("for statement with step=0");
        }
        let mut count = limit - init;
        if step > 0 {
            if step != 1 {
                count /= step;
            }
        } else {
            count = init - limit;
            count /= -step;
        }
        vm.push_integer(count);
        vm.replace(a + 1);
    } else {
        if !is_number_for_loop(vm, a) {
            panic!("for statement with non-numeric limit");
        }
        let init = vm.to_number(a);
        let limit = vm.to_number(a + 1);
        let step = vm.to_number(a + 2);
        if step == 0f64 {
            panic!("for statement with step=0");
        }
        if 0f64 < step && limit < init || step < 0f64 && init < limit {
            vm.add_pc(bx + 1);
        } else {
            vm.push_number(limit);
            vm.replace(a + 1);
            vm.push_number(step);
            vm.replace(a + 2);
            vm.push_number(init);
            vm.replace(a);
            vm.push_number(init);
            vm.replace(a + 3);
        }
    }
}

fn is_number_for_loop(vm: &mut dyn LuaVM, a: isize) -> bool {
    vm.is_number(a) || vm.is_number(a + 1) || vm.is_number(a + 2)
}

#[cfg(test)]
mod tests {
    use crate::{api::LuaAPI, binary::chunk::Prototype, state::LuaState};

    use super::*;
    #[test]
    fn test_for_prep_with_integer() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_integer(0);
        vm.push_integer(10);
        vm.push_integer(2);
        vm.push_integer(0);
        assert!(vm.stack.get(1).to_integer().unwrap() == 0);
        assert!(vm.stack.get(2).to_integer().unwrap() == 10);
        assert!(vm.stack.get(3).to_integer().unwrap() == 2);
        assert!(vm.stack.get(4).to_integer().unwrap() == 0);
        for_prep(0b00000000000000000_00000000_1001010, &mut vm);
        assert!(vm.stack.get(1).to_integer().unwrap() == 0);
        assert!(vm.stack.get(2).to_integer().unwrap() == 5);
        assert!(vm.stack.get(3).to_integer().unwrap() == 2);
        assert!(vm.stack.get(4).to_integer().unwrap() == 0);
    }

    #[test]
    fn test_for_prep_with_number() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_number(1.1f64);
        vm.push_number(10f64);
        vm.push_number(1f64);
        vm.push_number(0f64);
        assert!(vm.pc == 0);
        assert!(vm.stack.get(1).to_number().unwrap() == 1.1f64);
        assert!(vm.stack.get(2).to_number().unwrap() == 10f64);
        assert!(vm.stack.get(3).to_number().unwrap() == 1f64);
        assert!(vm.stack.get(4).to_number().unwrap() == 0f64);
        for_prep(0b00000000000000000_00000000_1001010, &mut vm);
        assert!(vm.pc == 0);
        assert!(vm.stack.get(1).to_number().unwrap() == 1.1f64);
        assert!(vm.stack.get(2).to_number().unwrap() == 10f64);
        assert!(vm.stack.get(3).to_number().unwrap() == 1f64);
        assert!(vm.stack.get(4).to_number().unwrap() == 1.1f64);
    }

    #[test]
    fn test_for_loop_with_integer() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_integer(0);
        vm.push_integer(10);
        vm.push_integer(2);
        vm.push_integer(0);
        vm.add_pc(1);
        assert!(vm.pc() == 1);
        assert!(vm.stack.get(1).to_integer().unwrap() == 0);
        assert!(vm.stack.get(2).to_integer().unwrap() == 10);
        assert!(vm.stack.get(3).to_integer().unwrap() == 2);
        assert!(vm.stack.get(4).to_integer().unwrap() == 0);
        for_prep(0b00000000000000000_00000000_1001010, &mut vm);
        assert!(vm.stack.get(1).to_integer().unwrap() == 0);
        assert!(vm.stack.get(2).to_integer().unwrap() == 5);
        assert!(vm.stack.get(3).to_integer().unwrap() == 2);
        assert!(vm.stack.get(4).to_integer().unwrap() == 0);
        for_loop(0b0_0000000000000001_00000000_1001001, &mut vm);
        assert!(vm.pc() == 0);
        assert!(vm.stack.get(1).to_integer().unwrap() == 2);
        assert!(vm.stack.get(2).to_integer().unwrap() == 4);
        assert!(vm.stack.get(3).to_integer().unwrap() == 2);
        assert!(vm.stack.get(4).to_integer().unwrap() == 2);
        for_loop(0b00000000000000000_00000000_1001001, &mut vm);
        assert!(vm.stack.get(1).to_integer().unwrap() == 4);
        assert!(vm.stack.get(2).to_integer().unwrap() == 3);
        assert!(vm.stack.get(3).to_integer().unwrap() == 2);
        assert!(vm.stack.get(4).to_integer().unwrap() == 4);
        for_loop(0b00000000000000000_00000000_1001001, &mut vm);
        assert!(vm.stack.get(1).to_integer().unwrap() == 6);
        assert!(vm.stack.get(2).to_integer().unwrap() == 2);
        assert!(vm.stack.get(3).to_integer().unwrap() == 2);
        assert!(vm.stack.get(4).to_integer().unwrap() == 6);
        for_loop(0b00000000000000000_00000000_1001001, &mut vm);
        assert!(vm.stack.get(1).to_integer().unwrap() == 8);
        assert!(vm.stack.get(2).to_integer().unwrap() == 1);
        assert!(vm.stack.get(3).to_integer().unwrap() == 2);
        assert!(vm.stack.get(4).to_integer().unwrap() == 8);
        for_loop(0b00000000000000000_00000000_1001001, &mut vm);
        assert!(vm.stack.get(1).to_integer().unwrap() == 10);
        assert!(vm.stack.get(2).to_integer().unwrap() == 0);
        assert!(vm.stack.get(3).to_integer().unwrap() == 2);
        assert!(vm.stack.get(4).to_integer().unwrap() == 10);
    }
}
