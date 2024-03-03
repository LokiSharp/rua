use super::instruction::Instruction;
use crate::api::LuaVM;

// OP_GETTABUP         A B C               R[A] := UpValue[B][K[C]:string]
pub fn get_tab_up(i: u32, vm: &mut dyn LuaVM) {
    let (a, c) = (i.get_arg_a() + 1, i.get_arg_c());

    vm.push_global_table();
    vm.get_const(c);
    vm.get_table(-2);
    vm.replace(a);
    vm.pop(1);
}
