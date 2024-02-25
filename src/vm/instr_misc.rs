use super::instruction::Instruction;
use crate::api::LuaVM;

// OP_MOVE             A B                 R[A] := R[B]
pub fn _move(i: u32, vm: &mut dyn LuaVM) {
    let (a, b) = (i.get_arg_a() + 1, i.get_arg_b() + 1);
    vm.copy(b, a);
}

// OP_JMP              sJ                  pc += sJ
pub fn jmp(i: u32, vm: &mut dyn LuaVM) {
    let sj = i.get_arg_sj();
    vm.add_pc(sj);
}
#[cfg(test)]
mod tests {
    use crate::{api::LuaAPI, binary::chunk::Prototype, state::LuaState};

    use super::*;
    #[test]
    fn test_move() {
        let mut vm = LuaState::new(10, Prototype::default());
        vm.push_integer(0);
        vm.push_integer(1);
        vm.push_integer(2);
        vm.push_integer(3);
        vm.push_integer(4);
        vm.push_integer(5);
        assert!(vm.stack.get(1).to_integer().unwrap() == 0);
        _move(0b00000000_00000001_0_00000000_0000000, &mut vm);
        assert!(vm.stack.get(1).to_integer().unwrap() == 1);
        _move(0b00000000_00000101_0_00000000_0000000, &mut vm);
        assert!(vm.stack.get(1).to_integer().unwrap() == 5);
    }

    #[test]
    fn test_jmp() {
        let mut vm = LuaState::new(10, Prototype::default());
        assert_eq!(vm.pc, 0);
        jmp(0b1000000000000000000001001_0111000, &mut vm);
        assert_eq!(vm.pc, 10);
        jmp(0b0_111111111111111111110101_0111000, &mut vm);
        assert_eq!(vm.pc, 0);
    }
}
