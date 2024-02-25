pub const OP_MOVE: u8 = 0b0000000;
pub const OP_LOADI: u8 = 0b0000001;
pub const OP_LOADF: u8 = 0b0000010;
pub const OP_LOADK: u8 = 0b0000011;
pub const OP_LOADKX: u8 = 0b0000100;
pub const OP_LOADFALSE: u8 = 0b0000101;
pub const OP_LFALSESKIP: u8 = 0b0000110;
pub const OP_LOADTRUE: u8 = 0b0000111;
pub const OP_LOADNIL: u8 = 0b0001000;
pub const OP_GETUPVAL: u8 = 0b0001001;
pub const OP_SETUPVAL: u8 = 0b0001010;
pub const OP_GETTABUP: u8 = 0b0001011;
pub const OP_GETTABLE: u8 = 0b0001100;
pub const OP_GETI: u8 = 0b0001101;
pub const OP_GETFIELD: u8 = 0b0001110;
pub const OP_SETTABUP: u8 = 0b0001111;
pub const OP_SETTABLE: u8 = 0b0010000;
pub const OP_SETI: u8 = 0b0010001;
pub const OP_SETFIELD: u8 = 0b0010010;
pub const OP_NEWTABLE: u8 = 0b0010011;
pub const OP_SELF: u8 = 0b0010100;
pub const OP_ADDI: u8 = 0b0010101;
pub const OP_ADDK: u8 = 0b0010110;
pub const OP_SUBK: u8 = 0b0010111;
pub const OP_MULK: u8 = 0b0011000;
pub const OP_MODK: u8 = 0b0011001;
pub const OP_POWK: u8 = 0b0011010;
pub const OP_DIVK: u8 = 0b0011011;
pub const OP_IDIVK: u8 = 0b0011100;
pub const OP_BANDK: u8 = 0b0011101;
pub const OP_BORK: u8 = 0b0011110;
pub const OP_BXORK: u8 = 0b0011111;
pub const OP_SHRI: u8 = 0b0100000;
pub const OP_SHLI: u8 = 0b0100001;
pub const OP_ADD: u8 = 0b0100010;
pub const OP_SUB: u8 = 0b0100011;
pub const OP_MUL: u8 = 0b0100100;
pub const OP_MOD: u8 = 0b0100101;
pub const OP_POW: u8 = 0b0100110;
pub const OP_DIV: u8 = 0b0100111;
pub const OP_IDIV: u8 = 0b0101000;
pub const OP_BAND: u8 = 0b0101001;
pub const OP_BOR: u8 = 0b0101010;
pub const OP_BXOR: u8 = 0b0101011;
pub const OP_SHL: u8 = 0b0101100;
pub const OP_SHR: u8 = 0b0101101;
pub const OP_MMBIN: u8 = 0b0101110;
pub const OP_MMBINI: u8 = 0b0101111;
pub const OP_MMBINK: u8 = 0b0110000;
pub const OP_UNM: u8 = 0b0110001;
pub const OP_BNOT: u8 = 0b0110010;
pub const OP_NOT: u8 = 0b0110011;
pub const OP_LEN: u8 = 0b0110100;
pub const OP_CONCAT: u8 = 0b0110101;
pub const OP_CLOSE: u8 = 0b0110110;
pub const OP_TBC: u8 = 0b0110111;
pub const OP_JMP: u8 = 0b0111000;
pub const OP_EQ: u8 = 0b0111001;
pub const OP_LT: u8 = 0b0111010;
pub const OP_LE: u8 = 0b0111011;
pub const OP_EQK: u8 = 0b0111100;
pub const OP_EQI: u8 = 0b0111101;
pub const OP_LTI: u8 = 0b0111110;
pub const OP_LEI: u8 = 0b0111111;
pub const OP_GTI: u8 = 0b1000000;
pub const OP_GEI: u8 = 0b1000001;
pub const OP_TEST: u8 = 0b1000010;
pub const OP_TESTSET: u8 = 0b1000011;
pub const OP_CALL: u8 = 0b1000100;
pub const OP_TAILCALL: u8 = 0b1000101;
pub const OP_RETURN: u8 = 0b1000110;
pub const OP_RETURN0: u8 = 0b1000111;
pub const OP_RETURN1: u8 = 0b1001000;
pub const OP_FORLOOP: u8 = 0b1001001;
pub const OP_FORPREP: u8 = 0b1001010;
pub const OP_TFORPREP: u8 = 0b1001011;
pub const OP_TFORCALL: u8 = 0b1001100;
pub const OP_TFORLOOP: u8 = 0b1001101;
pub const OP_SETLIST: u8 = 0b1001110;
pub const OP_CLOSURE: u8 = 0b1001111;
pub const OP_VARARG: u8 = 0b1010000;
pub const OP_VARARGPREP: u8 = 0b1010001;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum OpMode {
    IABC,
    IABx,
    IAsBx,
    IAx,
    IsJ,
}

#[allow(dead_code)]
impl OpMode {
    pub fn from_u8(value: u8) -> OpMode {
        match value {
            0 => OpMode::IABC,
            1 => OpMode::IABx,
            2 => OpMode::IAsBx,
            3 => OpMode::IAx,
            4 => OpMode::IsJ,
            _ => panic!("Invalid OpMode value: {}", value),
        }
    }
}

pub const OPCODES: &'static [OpCode] = &[
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_MOVE"),
    opcode(0, 0, 0, 0, 1, OpMode::IAsBx, "OP_LOADI"),
    opcode(0, 0, 0, 0, 1, OpMode::IAsBx, "OP_LOADF"),
    opcode(0, 0, 0, 0, 1, OpMode::IABx, "OP_LOADK"),
    opcode(0, 0, 0, 0, 1, OpMode::IABx, "OP_LOADKX"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_LOADFALSE"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_LFALSESKIP"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_LOADTRUE"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_LOADNIL"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_GETUPVAL"),
    opcode(0, 0, 0, 0, 0, OpMode::IABC, "OP_SETUPVAL"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_GETTABUP"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_GETTABLE"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_GETI"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_GETFIELD"),
    opcode(0, 0, 0, 0, 0, OpMode::IABC, "OP_SETTABUP"),
    opcode(0, 0, 0, 0, 0, OpMode::IABC, "OP_SETTABLE"),
    opcode(0, 0, 0, 0, 0, OpMode::IABC, "OP_SETI"),
    opcode(0, 0, 0, 0, 0, OpMode::IABC, "OP_SETFIELD"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_NEWTABLE"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_SELF"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_ADDI"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_ADDK"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_SUBK"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_MULK"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_MODK"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_POWK"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_DIVK"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_IDIVK"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_BANDK"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_BORK"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_BXORK"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_SHRI"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_SHLI"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_ADD"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_SUB"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_MUL"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_MOD"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_POW"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_DIV"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_IDIV"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_BAND"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_BOR"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_BXOR"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_SHL"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_SHR"),
    opcode(1, 0, 0, 0, 0, OpMode::IABC, "OP_MMBIN"),
    opcode(1, 0, 0, 0, 0, OpMode::IABC, "OP_MMBINI"),
    opcode(1, 0, 0, 0, 0, OpMode::IABC, "OP_MMBINK"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_UNM"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_BNOT"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_NOT"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_LEN"),
    opcode(0, 0, 0, 0, 1, OpMode::IABC, "OP_CONCAT"),
    opcode(0, 0, 0, 0, 0, OpMode::IABC, "OP_CLOSE"),
    opcode(0, 0, 0, 0, 0, OpMode::IABC, "OP_TBC"),
    opcode(0, 0, 0, 0, 0, OpMode::IsJ, "OP_JMP"),
    opcode(0, 0, 0, 1, 0, OpMode::IABC, "OP_EQ"),
    opcode(0, 0, 0, 1, 0, OpMode::IABC, "OP_LT"),
    opcode(0, 0, 0, 1, 0, OpMode::IABC, "OP_LE"),
    opcode(0, 0, 0, 1, 0, OpMode::IABC, "OP_EQK"),
    opcode(0, 0, 0, 1, 0, OpMode::IABC, "OP_EQI"),
    opcode(0, 0, 0, 1, 0, OpMode::IABC, "OP_LTI"),
    opcode(0, 0, 0, 1, 0, OpMode::IABC, "OP_LEI"),
    opcode(0, 0, 0, 1, 0, OpMode::IABC, "OP_GTI"),
    opcode(0, 0, 0, 1, 0, OpMode::IABC, "OP_GEI"),
    opcode(0, 0, 0, 1, 0, OpMode::IABC, "OP_TEST"),
    opcode(0, 0, 0, 1, 1, OpMode::IABC, "OP_TESTSET"),
    opcode(0, 1, 1, 0, 1, OpMode::IABC, "OP_CALL"),
    opcode(0, 1, 1, 0, 1, OpMode::IABC, "OP_TAILCALL"),
    opcode(0, 0, 1, 0, 0, OpMode::IABC, "OP_RETURN"),
    opcode(0, 0, 0, 0, 0, OpMode::IABC, "OP_RETURN0"),
    opcode(0, 0, 0, 0, 0, OpMode::IABC, "OP_RETURN1"),
    opcode(0, 0, 0, 0, 1, OpMode::IABx, "OP_FORLOOP"),
    opcode(0, 0, 0, 0, 1, OpMode::IABx, "OP_FORPREP"),
    opcode(0, 0, 0, 0, 0, OpMode::IABx, "OP_TFORPREP"),
    opcode(0, 0, 0, 0, 0, OpMode::IABC, "OP_TFORCALL"),
    opcode(0, 0, 0, 0, 1, OpMode::IABx, "OP_TFORLOOP"),
    opcode(0, 0, 1, 0, 0, OpMode::IABC, "OP_SETLIST"),
    opcode(0, 0, 0, 0, 1, OpMode::IABx, "OP_CLOSURE"),
    opcode(0, 1, 0, 0, 1, OpMode::IABC, "OP_VARARG"),
    opcode(0, 0, 1, 0, 1, OpMode::IABC, "OP_VARARGPREP"),
    opcode(0, 0, 0, 0, 0, OpMode::IAx, "OP_EXTRAARG"),
];

const fn opcode(
    mm: u8,
    ot: u8,
    it: u8,
    t: u8,
    a: u8,
    opmode: OpMode,
    name: &'static str,
) -> OpCode {
    OpCode {
        mm,
        ot,
        it,
        t,
        a,
        opmode,
        name,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OpCode {
    pub mm: u8,
    pub ot: u8,
    pub it: u8,
    pub t: u8,
    pub a: u8,
    pub opmode: OpMode,
    pub name: &'static str,
}
