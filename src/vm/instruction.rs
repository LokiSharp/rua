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

use super::opcodes::OPCODES;
const SIZE_A: isize = 8;
const SIZE_B: isize = 8;
const SIZE_C: isize = 8;
const SIZE_K: isize = 1;
const SIZE_BX: isize = SIZE_B + SIZE_C + SIZE_K;
const SIZE_AX: isize = SIZE_BX + SIZE_A;
const SIZE_SJ: isize = SIZE_BX + SIZE_A;
const SIZE_OP: isize = 7;

const POS_OP: isize = 0;
const POS_A: isize = POS_OP + SIZE_OP;
const POS_K: isize = POS_A + SIZE_A;
const POS_B: isize = POS_K + SIZE_K;
const POS_C: isize = POS_B + SIZE_B;
const POS_BX: isize = POS_K;
const POS_AX: isize = POS_A;
const POS_SJ: isize = POS_A;

const MAXARG_A: isize = (1 << SIZE_A) - 1;
const MAXARG_B: isize = (1 << SIZE_B) - 1;
const MAXARG_C: isize = (1 << SIZE_C) - 1;
const MAXARG_K: isize = (1 << SIZE_K) - 1;
const MAXARG_BX: isize = (1 << SIZE_BX) - 1;
const MAXARG_AX: isize = (1 << SIZE_AX) - 1;
const OFFSET_SB: isize = MAXARG_B >> 1;
const OFFSET_SC: isize = MAXARG_C >> 1;
const OFFSET_SBX: isize = MAXARG_BX >> 1;
const OFFSET_SJ: isize = ((1 << SIZE_SJ) - 1) >> 1;

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
}
