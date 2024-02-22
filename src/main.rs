use std::{
    env,
    fs::File,
    io::{self, Read},
};

use binary::chunk::{Constant, Prototype};
use vm::instruction::Instruction;
use Constant::*;

use crate::vm::instruction::MAXARG_C;

mod api;
mod binary;
mod state;
mod vm;

fn main() -> io::Result<()> {
    if env::args().count() > 1 {
        let filename = env::args().nth(1).unwrap();
        let mut file = File::open(filename)?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        let proto = binary::undump(data);
        list(&proto);
    }
    Ok(())
}

fn list(f: &Prototype) {
    print_header(f);
    print_code(f);
    print_detail(f);
    for p in &(f.protos) {
        list(p);
    }
}

fn print_header(f: &Prototype) {
    let func_type = if f.line_defined > 0 {
        "function"
    } else {
        "main"
    };
    let vararg_flag = if f.is_vararg > 0 { "+" } else { "" };
    let source = f.source.as_ref().map(|x| x.as_str()).unwrap_or("");
    //let source = f.source.clone().unwrap_or(String::new());

    print!("\n{}", func_type);
    print!(" <{}:{},{}>", source, f.line_defined, f.last_line_defined);
    print!(" ({} instructions at {:?})\n", f.code.len(), get_void(&f));
    print!("{}{} params", f.num_params, vararg_flag);
    print!(", {} slots", f.max_stack_size);
    print!(", {} upvalues", f.upvalues.len());
    print!(", {} locals", f.loc_vars.len());
    print!(", {} constants", f.constants.len());
    print!(", {} functions\n", f.protos.len());
}

fn print_code(f: &Prototype) {
    for pc in 0..f.code.len() {
        let line = get_funcline(f, pc);
        let instr = f.code[pc];
        print!("\t{}\t[{}]\t{} \t", pc + 1, line, instr.opname());
        print_operands(f, pc, instr);
        println!("");
    }
}

fn print_operands(f: &Prototype, pc: usize, i: u32) {
    let a = i.get_arg_a();
    let b = i.get_arg_b();
    let c = i.get_arg_c();
    let ax = i.get_arg_ax();
    let bx = i.get_arg_bx();
    let sb = i.get_arg_sb();
    let sc = i.get_arg_sc();
    let sbx = i.get_arg_sbx();
    let sj = i.get_arg_sj();
    let isk = i.get_arg_k() != 0;
    let k = i.get_arg_k();

    match i.opname() {
        "OP_MOVE" => print!("{a} {b}"),
        "OP_LOADI" => print!("{a} {sbx}"),
        "OP_LOADF" => print!("{a} {sbx}"),
        "OP_LOADK" => {
            print!("{a} {bx}");
            print!("\t; ");
            print_const(f, bx as usize);
        }
        "OP_LOADKX" => {
            print!("{a}");
            print!("\t; ");
            print_const(f, f.code[pc + 1].get_arg_ax() as usize);
        }
        "OP_LOADFALSE" => print!("{a}"),
        "OP_LFALSESKIP" => print!("{a}"),
        "OP_LOADTRUE" => print!("{a}"),
        "OP_LOADNIL" => {
            print!("{a} {b}");
            print!("\t; ");
            print!("{} out", b + 1);
        }
        "OP_GETUPVAL" => {
            print!("{a} {b}");
            print!("\t; ");
            print_upval_name(f, b);
        }
        "OP_SETUPVAL" => {
            print!("{a} {b}");
            print!("\t; ");
            print_upval_name(f, b);
        }
        "OP_GETTABUP" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            print_upval_name(f, b);
            print!(" ");
            print_const(f, c as usize);
        }
        "OP_GETTABLE" => print!("{a} {b} {c}"),
        "OP_GETI" => print!("{a} {b} {c}"),
        "OP_GETFIELD" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            print_const(f, c as usize);
        }
        "OP_SETTABUP" => {
            print!("{a} {b} {c}{k}", k = if isk { "k" } else { "" });
            print!("\t; ");
            print_upval_name(f, a);
            print!(" ");
            print_const(f, b as usize);
            if isk {
                print!(" ");
                print_const(f, c as usize);
            }
        }
        "OP_SETTABLE" => {
            print!("{a} {b} {c}{k}", k = if isk { "k" } else { "" });
            if isk {
                print!("\t");
                print_const(f, c as usize);
            }
        }
        "OP_SETI" => {
            print!("{a} {b} {c}{k}", k = if isk { "k" } else { "" });
            if isk {
                print!("\t");
                print_const(f, c as usize);
            }
        }
        "OP_SETFIELD" => {
            print!("{a} {b} {c}{k}", k = if isk { "k" } else { "" });
            print!("\t; ");
            print_upval_name(f, b);
            print!(" ");
            print_const(f, b as usize);
            if isk {
                print!(" ");
                print_const(f, c as usize);
            }
        }
        "OP_NEWTABLE" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            print!("{}", c + f.code[pc + 1].get_arg_ax() * (MAXARG_C + 1));
        }
        "OP_SELF" => {
            print!("{a} {b} {c}{k}", k = if isk { "k" } else { "" });
            if isk {
                print!("\t;");
                print_const(f, c as usize);
            }
        }
        "OP_ADDI" => print!("{a} {b} {sc}"),
        "OP_ADDK" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            print_const(f, c as usize);
        }
        "OP_SUBK" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            print_const(f, c as usize);
        }
        "OP_MULK" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            print_const(f, c as usize);
        }
        "OP_MODK" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            print_const(f, c as usize);
        }
        "OP_POWK" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            print_const(f, c as usize);
        }
        "OP_DIVK" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            print_const(f, c as usize);
        }
        "OP_IDIVK" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            print_const(f, c as usize);
        }
        "OP_BANDK" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            print_const(f, c as usize);
        }
        "OP_BORK" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            print_const(f, c as usize);
        }
        "OP_BXORK" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            print_const(f, c as usize);
        }
        "OP_SHRI" => print!("{a} {b} {sc}"),
        "OP_SHLI" => print!("{a} {b} {sc}"),
        "OP_ADD" => print!("{a} {b} {c}"),
        "OP_SUB" => print!("{a} {b} {c}"),
        "OP_MUL" => print!("{a} {b} {c}"),
        "OP_MOD" => print!("{a} {b} {c}"),
        "OP_POW" => print!("{a} {b} {c}"),
        "OP_DIV" => print!("{a} {b} {c}"),
        "OP_IDIV" => print!("{a} {b} {c}"),
        "OP_BAND" => print!("{a} {b} {c}"),
        "OP_BOR" => print!("{a} {b} {c}"),
        "OP_BXOR" => print!("{a} {b} {c}"),
        "OP_SHL" => print!("{a} {b} {c}"),
        "OP_SHR" => print!("{a} {b} {c}"),
        "OP_MMBIN" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            print!("todo! get_eventname {c}");
        }
        "OP_MMBINI" => {
            print!("{a} {sb} {c} {k}");
            print!("\t; ");
            print!("todo! get_eventname {c}");
            if isk {
                print!(" flip")
            };
        }
        "OP_MMBINK" => {
            print!("{a} {b} {c} {k}");
            print!("\t; ");
            print!("todo! get_eventname {c} ");
            print_const(f, b as usize);
            if isk {
                print!(" flip")
            };
        }
        "OP_UNM" => print!("{a} {b}"),
        "OP_BNOT" => print!("{a} {b}"),
        "OP_NOT" => print!("{a} {b}"),
        "OP_LEN" => print!("{a} {b}"),
        "OP_CONCAT" => print!("{a} {b}"),
        "OP_CLOSE" => print!("{a}"),
        "OP_TBC" => print!("{a}"),
        "OP_JMP" => {
            print!("{sj}");
            print!("\t; ");
            print!("to {}", sj + pc as isize + 2);
        }
        "OP_EQ" => print!("{a} {b} {k}"),
        "OP_LT" => print!("{a} {b} {k}"),
        "OP_LE" => print!("{a} {b} {k}"),
        "OP_EQK" => {
            print!("{a} {b} {k}");
            print!("\t; ");
            print_const(f, b as usize)
        }
        "OP_EQI" => print!("{a} {sb} {k}"),
        "OP_LTI" => print!("{a} {sb} {k}"),
        "OP_LEI" => print!("{a} {sb} {k}"),
        "OP_GTI" => print!("{a} {sb} {k}"),
        "OP_GEI" => print!("{a} {sb} {k}"),
        "OP_TEST" => print!("{a} {k}"),
        "OP_TESTSET" => print!("{a} {b} {k}"),
        "OP_CALL" => {
            print!("{a} {b} {c}");
            print!("\t; ");
            if b == 0 {
                print!("all in ");
            } else {
                print!("{} in ", b - 1);
            }
            if c == 0 {
                print!("all out ");
            } else {
                print!("{} out ", c - 1);
            }
        }
        "OP_TAILCALL" => {
            print!("{a} {b} {c}{k}");
            print!("\t; ");
            print!("{} in ", b - 1);
        }
        "OP_RETURN" => {
            print!("{a} {b} {c}{k}", k = if isk { "k" } else { "" });
            print!("\t; ");
            if b == 0 {
                print!("all out ");
            } else {
                print!("{} out ", b - 1);
            }
        }
        "OP_RETURN0" => {}
        "OP_RETURN1" => print!("{a}"),
        "OP_FORLOOP" => {
            print!("{a} {bx}");
            print!("\t; ");
            print!("to {}", pc as isize - bx + 2);
        }
        "OP_FORPREP" => {
            print!("{a} {bx}");
            print!("\t; ");
            print!("exit to {}", pc as isize + bx + 3);
        }
        "OP_TFORPREP" => {
            print!("{a} {bx}");
            print!("\t; ");
            print!("\tto {}", pc as isize + bx + 2);
        }
        "OP_TFORCALL" => print!("{a} {c}"),
        "OP_TFORLOOP" => {
            print!("{a} {bx}");
            print!("\t; ");
            print!("to {}", pc as isize - bx + 2);
        }
        "OP_SETLIST" => {
            print!("{a} {b} {c}");
            if isk {
                print!("\t; ");
                print!("{}", c + c + f.code[pc + 1].get_arg_ax() * (MAXARG_C + 1));
            }
        }
        "OP_CLOSURE" => {
            print!("{a} {bx}");
            print!("\t; ");
            print!("{:?}", get_void(&f.protos[bx as usize]))
        }
        "OP_VARARG" => {
            print!("{a} {c}");
            print!("\t; ");
            if c == 0 {
                print!("all out ");
            } else {
                print!("{} out ", c - 1);
            }
        }
        "OP_VARARGPREP" => print!("{a}"),
        "OP_EXTRAARG" => print!("{ax}"),
        _ => print!("{a} {b} {c}"),
    }
}

fn print_detail(f: &Prototype) {
    print_consts(f);
    print_locals(f);
    print_upvals(f)
}

fn print_consts(f: &Prototype) {
    let n = f.constants.len();
    println!("constants ({}) for {:?}:", n, get_void(&f));
    for i in 0..n {
        print!("\t{}\t", i);
        print_type(&f.constants[i]);
        print!("\t");
        print_const(f, i);
        print!("\n")
    }
}

fn print_type(k: &Constant) {
    match k {
        Nil => print!("N"),
        Boolean(_) => print!("B"),
        Number(_) => print!("F"),
        Integer(_) => print!("I"),
        Str(_) => print!("S"),
    }
}

fn print_const(f: &Prototype, i: usize) {
    let k = &f.constants[i];
    match k {
        Nil => print!("nil"),
        Boolean(b) => print!("{b}"),
        Number(x) => print!("{x}"),
        Integer(i) => print!("{i}"),
        Str(s) => print!("{s:?}"),
    }
}

fn print_upval_name(f: &Prototype, i: isize) {
    let name = f
        .upvalue_names
        .get(i as usize)
        .map(|x| x.as_str())
        .unwrap_or("");
    print!("{name}");
}

fn print_locals(f: &Prototype) {
    let n = f.loc_vars.len();
    println!("locals ({}) for {:?}:", n, get_void(&f));
    for i in 0..n {
        let var = &f.loc_vars[i];
        println!(
            "\t{}\t{}\t{}\t{}",
            i,
            var.var_name,
            var.start_pc + 1,
            var.end_pc + 1
        );
    }
}

fn print_upvals(f: &Prototype) {
    let n = f.upvalues.len();
    println!("upvalues ({}) for {:?}:", n, get_void(&f));
    for i in 0..n {
        let upval = &f.upvalues[i];
        let name = f.upvalue_names.get(i).map(|x| x.as_str()).unwrap_or("");
        println!("\t{}\t{}\t{}\t{}", i, name, upval.instack, upval.idx);
    }
}

fn get_baseline(f: &Prototype, pc: usize) -> (isize, isize) {
    if f.abs_line_info.len() == 0 || pc < f.abs_line_info[0].pc {
        (-1, f.line_defined as isize)
    } else {
        let mut i = pc / 128 - 1;
        while i + 1 < f.abs_line_info.len() && pc >= f.abs_line_info[i + 1].pc {
            i += 1;
        }
        (
            f.abs_line_info[i].pc as isize,
            f.abs_line_info[i].line as isize,
        )
    }
}

fn get_funcline(f: &Prototype, pc: usize) -> isize {
    if f.line_info.len() == 0 {
        -1
    } else {
        let (mut basepc, mut base_line) = get_baseline(f, pc);
        while basepc < pc as isize {
            basepc += 1;
            base_line += f.line_info[basepc as usize] as isize;
        }
        base_line
    }
}

fn get_void<T>(f: &T) -> *const T {
    f as *const T
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;
    #[test]
    fn test_undump() {
        let mut file = File::open("lua/all.luac").expect("Failed to open file");
        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("Failed to read file");

        let proto = binary::undump(data);
        list(&proto);
    }
}
