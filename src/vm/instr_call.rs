use crate::api::LuaVM;

use super::instruction::Instruction;

// OP_SELF             A B C               R[A+1] := R[B]; R[A] := R[B][RK(C):string]
pub fn _self(i: u32, vm: &mut dyn LuaVM) {
    let (a, b, c) = (i.get_arg_a() + 1, i.get_arg_b() + 1, i.get_arg_c());

    vm.copy(b, a + 1);
    vm.get_rk(c);
    vm.get_table(b);
    vm.replace(a);
}

// OP_CALL             A B C               R[A], ... ,R[A+C-2] := R[A](R[A+1], ... ,R[A+B-1])
pub fn call(i: u32, vm: &mut dyn LuaVM) {
    let (a, b, c) = (i.get_arg_a() + 1, i.get_arg_b(), i.get_arg_c());
    let nargs = push_func_and_args(a, b, vm);
    vm.call(nargs, c - 1);
    pop_results(a, c, vm);
}

// OP_TAILCALL         A B C k             return R[A](R[A+1], ... ,R[A+B-1])
pub fn tail_call(i: u32, vm: &mut dyn LuaVM) {
    let (a, b, c) = (i.get_arg_a() + 1, i.get_arg_b(), 0);
    let nargs = push_func_and_args(a, b, vm);
    vm.call(nargs, c - 1);
    pop_results(a, c, vm);
}

// OP_RETURN           A B C k             return R[A], ... ,R[A+B-2]  (see note)
pub fn _return(i: u32, vm: &mut dyn LuaVM) {
    let (a, b) = (i.get_arg_a() + 1, i.get_arg_b());

    if b == 1 {
        // no return values
    } else if b > 1 {
        // b-1 return values
        vm.check_stack(b as usize - 1);
        for i in a..(a + b - 1) {
            vm.push_value(i);
        }
    } else {
        fix_stack(a, vm);
    }
}

// OP_RETURN0                              return
pub fn return0(_i: u32, _vm: &mut dyn LuaVM) {
    // no results
}

// OP_CLOSURE          A Bx                R[A] := closure(KPROTO[Bx])
pub fn closure(i: u32, vm: &mut dyn LuaVM) {
    let (a, bx) = (i.get_arg_a() + 1, i.get_arg_bx());
    vm.load_proto(bx as usize);
    vm.replace(a);
}

// OP_VARARG           A C                 R[A], R[A+1], ..., R[A+C-2] = vararg
pub fn vararg(i: u32, vm: &mut dyn LuaVM) {
    let (a, c) = (i.get_arg_a() + 1, i.get_arg_c());
    if c != 1 {
        vm.load_vararg(c - 1);
        pop_results(a, c, vm);
    }
}

fn push_func_and_args(a: isize, b: isize, vm: &mut dyn LuaVM) -> usize {
    if b >= 1 {
        vm.check_stack(b as usize);
        for i in a..(a + b) {
            vm.push_value(i);
        }
        b as usize - 1
    } else {
        fix_stack(a, vm);
        vm.get_top() as usize - vm.register_count() - 1
    }
}

fn fix_stack(a: isize, vm: &mut dyn LuaVM) {
    let x = vm.to_integer(-1) as isize;
    vm.pop(1);

    vm.check_stack((x - a) as usize);
    for i in a..x {
        vm.push_value(i);
    }
    vm.rotate(vm.register_count() as isize + 1, x - a);
}

fn pop_results(a: isize, c: isize, vm: &mut dyn LuaVM) {
    if c == 1 {
        // no results
    } else if c > 1 {
        for i in (a..(a + c - 1)).rev() {
            vm.replace(i);
        }
    } else {
        // leave results on stack
        vm.check_stack(1);
        vm.push_integer(a as i64);
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        api::LuaAPI,
        binary::chunk::Prototype,
        state::{lua_value::LuaValue, LuaState},
    };

    use super::*;

    #[test]
    fn test_closure() {
        let mut proto = Prototype::default();
        proto.protos.push(Rc::new(Prototype::default()));
        let mut vm = LuaState::new_with_proto(Rc::new(proto));
        vm.push_nil();
        closure(0b00000000000000000_00000000_1001111, &mut vm);
        assert_eq!(vm.is_function(1), true);
    }

    #[test]
    fn test_vararg() {
        let mut vm = LuaState::new();
        vm.stack_mut().varargs.push(LuaValue::Integer(1));
        vararg(0b00000000_00000000_0_00000000_1010000, &mut vm);
        assert_eq!(vm.to_integer(1), 1);
    }
}
