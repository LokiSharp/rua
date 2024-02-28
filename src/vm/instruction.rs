/*
+-------+-----------------------------------------------------------------+
|       | 3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0 |
|       | 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 |
+-------+-----------------------------------------------------------------+
| iABC  |       C(8)     |      B(8)     |k|     A(8)      |   Op(7)      |
+-------+----------------------------------+---------------+--------------+
| iABx  |             Bx(17)               |     A(8)      |   Op(7)      |
+-------+----------------------------------+---------------+--------------+
| iAsBx |            sBx (signed)(17)      |     A(8)      |   Op(7)      |
+-------+--------------------------------------------------+--------------+
| iAx   |                       Ax(25)                     |   Op(7)      |
+-------+--------------------------------------------------+--------------+
| isJ   |                       sJ (signed)(25)            |   Op(7)      |
+-------+--------------------------------------------------+--------------+
*/

use crate::{
    api::LuaVM,
    vm::{instr_for::*, instr_load::*, instr_misc::*, instr_ops::*, opcodes::*},
};

use super::opcodes::OPCODES;
pub const SIZE_A: isize = 8;
pub const SIZE_B: isize = 8;
pub const SIZE_C: isize = 8;
pub const SIZE_K: isize = 1;
pub const SIZE_BX: isize = SIZE_B + SIZE_C + SIZE_K;
pub const SIZE_AX: isize = SIZE_BX + SIZE_A;
pub const SIZE_SJ: isize = SIZE_BX + SIZE_A;
pub const SIZE_OP: isize = 7;

pub const POS_OP: isize = 0;
pub const POS_A: isize = POS_OP + SIZE_OP;
pub const POS_K: isize = POS_A + SIZE_A;
pub const POS_B: isize = POS_K + SIZE_K;
pub const POS_C: isize = POS_B + SIZE_B;
pub const POS_BX: isize = POS_K;
pub const POS_AX: isize = POS_A;
pub const POS_SJ: isize = POS_A;

pub const MAXARG_A: isize = (1 << SIZE_A) - 1;
pub const MAXARG_B: isize = (1 << SIZE_B) - 1;
pub const MAXARG_C: isize = (1 << SIZE_C) - 1;
pub const MAXARG_K: isize = (1 << SIZE_K) - 1;
pub const MAXARG_BX: isize = (1 << SIZE_BX) - 1;
pub const MAXARG_AX: isize = (1 << SIZE_AX) - 1;
pub const OFFSET_SB: isize = MAXARG_B >> 1;
pub const OFFSET_SC: isize = MAXARG_C >> 1;
pub const OFFSET_SBX: isize = MAXARG_BX >> 1;
pub const OFFSET_SJ: isize = ((1 << SIZE_SJ) - 1) >> 1;

pub trait Instruction {
    fn opname(self) -> &'static str;
    fn opmode(self) -> u8;
    fn opcode(self) -> u8;
    fn get_arg_a(self) -> isize;
    fn get_arg_b(self) -> isize;
    fn get_arg_c(self) -> isize;
    fn get_arg_k(self) -> isize;
    fn get_arg_ax(self) -> isize;
    fn get_arg_bx(self) -> isize;
    fn get_arg_sb(self) -> isize;
    fn get_arg_sc(self) -> isize;
    fn get_arg_sbx(self) -> isize;
    fn get_arg_sj(self) -> isize;
    fn execute(self, vm: &mut dyn LuaVM);
}

impl Instruction for u32 {
    fn opname(self) -> &'static str {
        OPCODES[self.opcode() as usize].name
    }

    fn opmode(self) -> u8 {
        OPCODES[self.opcode() as usize].opmode as u8
    }

    fn opcode(self) -> u8 {
        self as u8 & 0x7F
    }

    fn get_arg_a(self) -> isize {
        (self >> POS_A & MAXARG_A as u32) as isize
    }

    fn get_arg_b(self) -> isize {
        (self >> POS_B & MAXARG_B as u32) as isize
    }

    fn get_arg_c(self) -> isize {
        (self >> POS_C & MAXARG_C as u32) as isize
    }

    fn get_arg_k(self) -> isize {
        (self >> POS_K & MAXARG_K as u32) as isize
    }

    fn get_arg_ax(self) -> isize {
        (self >> POS_AX) as isize
    }

    fn get_arg_bx(self) -> isize {
        (self >> POS_BX) as isize
    }

    fn get_arg_sb(self) -> isize {
        self.get_arg_b() - OFFSET_SB
    }

    fn get_arg_sc(self) -> isize {
        self.get_arg_c() - OFFSET_SC
    }

    fn get_arg_sbx(self) -> isize {
        self.get_arg_bx() - OFFSET_SBX
    }

    fn get_arg_sj(self) -> isize {
        (self >> POS_SJ & MAXARG_AX as u32) as isize - OFFSET_SJ
    }

    fn execute(self, vm: &mut dyn LuaVM) {
        match self.opcode() {
            OP_MOVE => _move(self, vm),
            OP_LOADI => load_i(self, vm),
            OP_LOADF => load_f(self, vm),
            OP_LOADK => load_k(self, vm),
            OP_LOADKX => load_kx(self, vm),
            OP_LOADFALSE => load_false(self, vm),
            OP_LFALSESKIP => load_l_false_skip(self, vm),
            OP_LOADTRUE => load_true(self, vm),
            OP_LOADNIL => load_nil(self, vm),
            OP_ADDI => add_i(self, vm),
            OP_ADDK => add_k(self, vm),
            OP_SUBK => sub_k(self, vm),
            OP_MULK => mul_k(self, vm),
            OP_MODK => _mod_k(self, vm),
            OP_POWK => pow_k(self, vm),
            OP_DIVK => div_k(self, vm),
            OP_IDIVK => idiv_k(self, vm),
            OP_BANDK => band_k(self, vm),
            OP_BORK => bor_k(self, vm),
            OP_BXORK => bxor_k(self, vm),
            OP_SHLI => shl_i(self, vm),
            OP_SHRI => shr_i(self, vm),
            OP_ADD => add(self, vm),
            OP_SUB => sub(self, vm),
            OP_MUL => mul(self, vm),
            OP_MOD => _mod(self, vm),
            OP_POW => pow(self, vm),
            OP_DIV => div(self, vm),
            OP_IDIV => idiv(self, vm),
            OP_BAND => band(self, vm),
            OP_BOR => bor(self, vm),
            OP_BXOR => bxor(self, vm),
            OP_SHL => shl(self, vm),
            OP_SHR => shr(self, vm),
            OP_MMBIN => {}
            OP_MMBINI => {}
            OP_MMBINK => {}
            OP_UNM => unm(self, vm),
            OP_BNOT => bnot(self, vm),
            OP_NOT => not(self, vm),
            OP_LEN => len(self, vm),
            OP_CONCAT => concat(self, vm),
            OP_JMP => jmp(self, vm),
            OP_EQ => eq(self, vm),
            OP_LT => lt(self, vm),
            OP_LE => le(self, vm),
            OP_EQK => eq_k(self, vm),
            OP_EQI => eq_i(self, vm),
            OP_LTI => lt_i(self, vm),
            OP_LEI => le_i(self, vm),
            OP_GTI => gt_i(self, vm),
            OP_GEI => ge_i(self, vm),
            OP_TEST => test(self, vm),
            OP_TESTSET => test_set(self, vm),
            OP_FORLOOP => for_loop(self, vm),
            OP_FORPREP => for_prep(self, vm),
            OP_VARARGPREP => {}
            _ => {
                dbg!(self.opname());
                unimplemented!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::{
        api::{r#type::Type, LuaAPI},
        binary::{self, chunk::Prototype},
        state::{self, LuaState},
        vm::opcodes,
    };

    use super::*;

    #[test]
    fn test_instruction() {
        let mut file = File::open("lua/sum.luac").expect("Failed to open file");

        let mut data = Vec::new();
        let _ = file.read_to_end(&mut data);

        let proto = binary::undump(data);
        lua_main(proto);
    }

    fn lua_main(proto: Prototype) {
        let nregs = proto.max_stack_size;
        let mut ls = state::new_lua_state((nregs + 8) as usize, proto);
        ls.set_top(nregs as isize);
        loop {
            let pc = ls.pc();
            let instr = ls.fetch();
            if instr.opcode() != opcodes::OP_RETURN {
                instr.execute(&mut ls);
                print!("[{:04}] {} \t", pc + 1, instr.opname());
                print_stack(&ls);
            } else {
                break;
            }
        }
    }

    fn print_stack(ls: &LuaState) {
        let top = ls.get_top();
        for i in 1..top + 1 {
            let t = ls.type_id(i);
            match Type::from_i8(t) {
                Some(Type::Boolean) => print!("[{}]", ls.to_boolean(i)),
                Some(Type::Number) => print!("[{}]", ls.to_number(i)),
                Some(Type::String) => print!("[{:?}]", ls.to_string(i)),
                _ => print!("[{}]", ls.type_name(t)), // other values
            }
        }
        println!("");
    }
}
