use std::rc::Rc;

use crate::{
    api::{op::ArithOp, r#type::Type, LuaAPI, LuaVM},
    binary::chunk::{Constant, Prototype},
    state::arith_ops::arith,
    vm::instruction::Instruction,
};

use super::{closure::Closure, lua_stack::LuaStack, lua_value::LuaValue};

/// `LuaState` 是一个用于表示 Lua 状态的结构体。
#[derive(Debug)]
pub struct LuaState {
    pub(crate) frames: Vec<LuaStack>,
}

impl LuaState {
    pub fn new() -> LuaState {
        let proto = Rc::new(Prototype::default());
        let closure = Rc::new(Closure::new(Rc::clone(&proto)));
        let frame = LuaStack::new(proto.max_stack_size as usize, closure);
        LuaState {
            frames: vec![frame],
        }
    }

    pub fn new_with_proto(proto: Rc<Prototype>) -> LuaState {
        let closure = Rc::new(Closure::new(Rc::clone(&proto)));
        let frame = LuaStack::new(proto.max_stack_size as usize, closure);
        LuaState {
            frames: vec![frame],
        }
    }

    pub(crate) fn stack_mut(&mut self) -> &mut LuaStack {
        self.frames.last_mut().unwrap() // TODO
    }

    pub(crate) fn stack(&self) -> &LuaStack {
        self.frames.last().unwrap() // TODO
    }

    pub(crate) fn push_frame(&mut self, frame: LuaStack) {
        self.frames.push(frame);
    }

    pub(crate) fn pop_frame(&mut self) -> LuaStack {
        self.frames.pop().unwrap()
    }
}

impl LuaVM for LuaState {
    /// 获取当前的程序计数器（pc）的值。
    fn pc(&self) -> isize {
        self.stack().pc
    }

    /// 将程序计数器（pc）增加指定的值。
    fn add_pc(&mut self, n: isize) {
        self.stack_mut().pc += n;
    }

    /// 获取当前程序计数器（pc）指向的指令，并将程序计数器（pc）向前移动一位。
    fn fetch(&mut self) -> u32 {
        let i = self.stack().closure.proto.code[self.pc() as usize];
        self.stack_mut().pc += 1;
        i
    }

    /// 获取指定索引的常量，并将其推送到栈顶。
    fn get_const(&mut self, idx: isize) {
        let c = &self.stack().closure.proto.constants[idx as usize];
        let val = match c {
            Constant::Nil => LuaValue::Nil,
            Constant::Boolean(b) => LuaValue::Boolean(*b),
            Constant::Integer(i) => LuaValue::Integer(*i),
            Constant::Number(n) => LuaValue::Number(*n),
            Constant::Str(s) => LuaValue::Str((*s).clone()),
        };
        self.stack_mut().push(val);
    }

    /// 获取指定的 RK 值。
    ///
    /// 如果 RK 值大于 0xFF，那么它是一个常量索引，此时将获取该常量并推送到栈顶；
    /// 否则，它是一个寄存器索引，此时将推送该寄存器的值到栈顶。
    ///
    fn get_rk(&mut self, rk: isize) {
        if rk > 0xFF {
            self.get_const(rk & 0xFF);
        } else {
            self.push_value(rk + 1);
        }
    }
}

impl LuaAPI for LuaState {
    /// 获取栈顶的索引。
    fn get_top(&self) -> isize {
        self.stack().top()
    }

    /// 将一个相对索引转换为绝对索引。
    fn abs_index(&self, idx: isize) -> isize {
        self.stack().abs_index(idx)
    }

    /// 检查栈是否有足够的空间来存储 `n` 个元素，如果没有则分配更多的空间。
    fn check_stack(&mut self, n: usize) -> bool {
        self.stack_mut().check(n);
        true
    }

    /// 从栈顶弹出 `n` 个值。
    fn pop(&mut self, n: usize) {
        for _ in 0..n {
            self.stack_mut().pop();
        }
    }

    /// 将 `from_idx` 索引处的值复制到 `to_idx` 索引处。
    fn copy(&mut self, from_idx: isize, to_idx: isize) {
        let val = self.stack().get(from_idx);
        self.stack_mut().set(to_idx, val);
    }

    /// 将 `idx` 索引处的值推送到栈顶。
    fn push_value(&mut self, idx: isize) {
        let val = self.stack().get(idx);
        self.stack_mut().push(val);
    }

    /// 用栈顶的值替换 `idx` 索引处的值。
    fn replace(&mut self, idx: isize) {
        let val = self.stack_mut().pop();
        self.stack_mut().set(idx, val);
    }

    /// 在 `idx` 索引处插入一个值，该值是栈顶的值。
    fn insert(&mut self, idx: isize) {
        self.rotate(idx, 1);
    }

    /// 移除 `idx` 索引处的值。
    fn remove(&mut self, idx: isize) {
        self.rotate(idx, -1);
        self.pop(1);
    }

    /// 旋转栈中从 `idx` 开始的 `n` 个元素。
    fn rotate(&mut self, idx: isize, n: isize) {
        let abs_idx = self.stack().abs_index(idx);
        if abs_idx < 0 || !self.stack().is_valid(abs_idx) {
            panic!("invalid index!");
        }

        let t = self.stack().top() - 1; /* end of stack segment being rotated */
        let p = abs_idx - 1; /* start of segment */
        let m = if n >= 0 { t - n } else { p - n - 1 }; /* end of prefix */
        self.stack_mut().reverse(p as usize, m as usize); /* reverse the prefix with length 'n' */
        self.stack_mut().reverse(m as usize + 1, t as usize); /* reverse the suffix */
        self.stack_mut().reverse(p as usize, t as usize); /* reverse the entire segment */
    }

    /// 设置栈顶的索引。
    fn set_top(&mut self, idx: isize) {
        let new_top = self.stack().abs_index(idx);
        if new_top < 0 {
            panic!("stack underflow!");
        }

        let n = self.stack().top() - new_top;
        if n > 0 {
            for _ in 0..n {
                self.stack_mut().pop();
            }
        } else if n < 0 {
            for _ in n..0 {
                self.stack_mut().push(LuaValue::Nil);
            }
        }
    }

    /// 获取 `tp` 类型的名称。
    fn type_name(&self, tp: i8) -> &str {
        match Type::from_i8(tp) {
            Some(Type::None) => "no value",
            Some(Type::Nil) => "nil",
            Some(Type::Boolean) => "boolean",
            Some(Type::LightUserData) => "userdata",
            Some(Type::Number) => "number",
            Some(Type::String) => "string",
            Some(Type::Table) => "table",
            Some(Type::Function) => "function",
            Some(Type::UserData) => "userdata",
            Some(Type::Thread) => "thread",
            Some(Type::NumTypes) => "number of types",
            Some(Type::Proto) => "proto",
            Some(Type::DeadKey) => "dead key",
            Some(Type::TolalTypes) => "total types",
            _ => panic!("?"),
        }
    }

    /// 获取 `idx` 索引处的值的类型 ID。
    fn type_id(&self, idx: isize) -> i8 {
        if self.stack().is_valid(idx) {
            let i = self.stack().get(idx);
            i.type_id()
        } else {
            Type::None as i8
        }
    }

    /// 检查 `idx` 索引处的值是否为 `None` 类型。
    fn is_none(&self, idx: isize) -> bool {
        self.type_id(idx) == Type::None as i8
    }

    /// 检查 `idx` 索引处的值是否为 `Nil` 类型。
    fn is_nil(&self, idx: isize) -> bool {
        self.type_id(idx) == Type::Nil as i8
    }

    /// 检查 `idx` 索引处的值是否为 `None` 或 `Nil` 类型。
    fn is_none_or_nil(&self, idx: isize) -> bool {
        self.is_none(idx) || self.is_nil(idx)
    }

    /// 检查 `idx` 索引处的值是否为布尔类型。
    fn is_boolean(&self, idx: isize) -> bool {
        self.type_id(idx) == Type::Boolean as i8
    }

    /// 检查 `idx` 索引处的值是否为整数类型。
    fn is_integer(&self, idx: isize) -> bool {
        match self.stack().get(idx) {
            LuaValue::Integer(_) => true,
            _ => false,
        }
    }

    /// 检查 `idx` 索引处的值是否为数字类型。
    fn is_number(&self, idx: isize) -> bool {
        self.to_numberx(idx).is_some()
    }

    /// 检查 `idx` 索引处的值是否为字符串类型。
    fn is_string(&self, idx: isize) -> bool {
        let t = self.type_id(idx);
        t == Type::String as i8 || t == Type::Number as i8
    }

    /// 检查 `idx` 索引处的值是否为表类型。
    fn is_table(&self, idx: isize) -> bool {
        self.type_id(idx) == Type::Table as i8
    }

    /// 检查 `idx` 索引处的值是否为线程类型。
    fn is_thread(&self, idx: isize) -> bool {
        self.type_id(idx) == Type::Thread as i8
    }

    /// 检查 `idx` 索引处的值是否为函数类型。
    fn is_function(&self, idx: isize) -> bool {
        self.type_id(idx) == Type::Function as i8
    }

    /// 将 `idx` 索引处的值转换为布尔值。
    fn to_boolean(&self, idx: isize) -> bool {
        self.stack().get(idx).to_boolean()
    }

    /// 将 `idx` 索引处的值转换为整数。
    fn to_integer(&self, idx: isize) -> i64 {
        self.to_integerx(idx).unwrap()
    }

    /// 尝试将 `idx` 索引处的值转换为整数，如果转换失败则返回 `None`。
    fn to_integerx(&self, idx: isize) -> Option<i64> {
        match self.stack().get(idx) {
            LuaValue::Integer(n) => Some(n),
            _ => None,
        }
    }

    /// 将 `idx` 索引处的值转换为数字。
    fn to_number(&self, idx: isize) -> f64 {
        self.to_numberx(idx).unwrap()
    }

    /// 尝试将 `idx` 索引处的值转换为数字，如果转换失败则返回 `None`。
    fn to_numberx(&self, idx: isize) -> Option<f64> {
        match self.stack().get(idx) {
            LuaValue::Number(n) => Some(n),
            LuaValue::Integer(n) => Some(n as f64),
            _ => None,
        }
    }

    /// 将 `idx` 索引处的值转换为字符串。
    fn to_string(&self, idx: isize) -> String {
        self.to_stringx(idx).unwrap()
    }

    /// 尝试将 `idx` 索引处的值转换为字符串，如果转换失败则返回 `None`。
    fn to_stringx(&self, idx: isize) -> Option<String> {
        match self.stack().get(idx) {
            LuaValue::Str(s) => Some(s),
            LuaValue::Number(n) => Some(n.to_string()),
            LuaValue::Integer(n) => Some(n.to_string()),
            _ => None,
        }
    }

    /// 将一个 `Nil` 值推送到栈顶。
    fn push_nil(&mut self) {
        self.stack_mut().push(LuaValue::Nil);
    }

    /// 将一个布尔值推送到栈顶。
    fn push_boolean(&mut self, b: bool) {
        self.stack_mut().push(LuaValue::Boolean(b));
    }

    /// 将一个整数推送到栈顶。
    fn push_integer(&mut self, n: i64) {
        self.stack_mut().push(LuaValue::Integer(n));
    }

    /// 将一个数字推送到栈顶。
    fn push_number(&mut self, n: f64) {
        self.stack_mut().push(LuaValue::Number(n));
    }

    /// 将一个字符串推送到栈顶。
    fn push_string(&mut self, s: String) {
        self.stack_mut().push(LuaValue::Str(s));
    }

    /// 对栈顶的两个元素进行算术运算，并将结果推送到栈顶。
    fn arith(&mut self, op: u8) {
        if op != ArithOp::UNM as u8 && op != ArithOp::BNOT as u8 {
            let b = self.stack_mut().pop();
            let a = self.stack_mut().pop();
            if let Some(result) = arith(&a, &b, op) {
                self.stack_mut().push(result);
                return;
            }
        } else {
            let a = self.stack_mut().pop();
            if let Some(result) = arith(&a, &a, op) {
                self.stack_mut().push(result);
                return;
            }
        }
        panic!("arithmetic error!");
    }

    /// 比较 `idx1` 和 `idx2` 索引处的两个值。
    fn compare(&mut self, idx1: isize, idx2: isize, op: u8) -> bool {
        if !self.stack().is_valid(idx1) || !self.stack().is_valid(idx2) {
            return false;
        } else {
            let a = self.stack().get(idx1);
            let b = self.stack().get(idx2);
            if let Some(result) = super::cmp_ops::compare(&a, &b, op) {
                return result;
            }
            panic!("comparison error!")
        }
    }

    /// 获取 `idx` 索引处的值的长度。
    fn len(&mut self, idx: isize) {
        let val = self.stack().get(idx);
        match val {
            LuaValue::Str(s) => self.stack_mut().push(LuaValue::Integer(s.len() as i64)),
            LuaValue::Table(t) => self
                .stack_mut()
                .push(LuaValue::Integer(t.borrow().len() as i64)),
            _ => panic!("length error!"),
        }
    }

    /// 连接栈顶的 `n` 个字符串，并将结果推送到栈顶。
    fn concat(&mut self, n: isize) {
        if n == 0 {
            self.stack_mut().push(LuaValue::Str("".to_string()));
        } else if n >= 2 {
            for _ in 1..n {
                if self.is_string(-1) && self.is_string(-2) {
                    let s2 = self.to_string(-1);
                    let mut s1 = self.to_string(-2);
                    s1.push_str(&s2);
                    self.stack_mut().pop();
                    self.stack_mut().pop();
                    self.stack_mut().push(LuaValue::Str(s1));
                } else {
                    panic!("concatenation error!");
                }
            }
        }
        // n == 1, do nothing
    }

    /// 创建一个新的空表，并将其推送到栈顶。
    fn new_table(&mut self) {
        self.create_table(0, 0);
    }

    /// 创建一个新的指定容量的空表，并将其推送到栈顶。
    fn create_table(&mut self, narr: usize, nrec: usize) {
        let t = LuaValue::new_table(narr, nrec);
        self.stack_mut().push(t);
    }

    /// 从栈顶弹出一个键，然后从栈顶弹出一个表，然后将该键对应的值推送到栈顶。
    fn get_table(&mut self, idx: isize) -> i8 {
        let t = self.stack().get(idx);
        let k = self.stack_mut().pop();
        self.get_table_impl(&t, &k)
    }

    /// 从栈顶弹出一个键，然后从栈顶弹出一个表，然后将该键对应的值推送到栈顶。
    fn get_field(&mut self, idx: isize, k: &str) -> i8 {
        let t = self.stack().get(idx);
        let k = LuaValue::Str(k.to_string());
        self.get_table_impl(&t, &k)
    }

    /// 从栈顶弹出一个键，然后从栈顶弹出一个表，然后将该键对应的值推送到栈顶。
    fn get_i(&mut self, idx: isize, i: i64) -> i8 {
        let t = self.stack().get(idx);
        let k = LuaValue::Integer(i);
        self.get_table_impl(&t, &k)
    }

    /// 将栈顶的值弹出，并将其设置为表的值。
    fn set_table(&mut self, idx: isize) {
        let t = self.stack().get(idx);
        let v = self.stack_mut().pop();
        let k = self.stack_mut().pop();
        Self::set_table_impl(&t, k, v);
    }

    /// 将栈顶的值弹出，并将其设置为表的值。
    fn set_field(&mut self, idx: isize, k: &str) {
        let t = self.stack().get(idx);
        let v = self.stack_mut().pop();
        let k = LuaValue::Str(k.to_string());
        Self::set_table_impl(&t, k, v);
    }

    /// 将栈顶的值弹出，并将其设置为表的值。
    fn set_i(&mut self, idx: isize, i: i64) {
        let t = self.stack().get(idx);
        let v = self.stack_mut().pop();
        let k = LuaValue::Integer(i);
        Self::set_table_impl(&t, k, v);
    }

    fn load(&mut self, chunk: Vec<u8>, chunk_name: &str, mode: &str) -> u8 {
        let proto = crate::binary::undump(chunk);
        let closure = LuaValue::new_lua_closure(proto);
        self.stack_mut().push(closure);
        0
    }

    fn call(&mut self, nargs: usize, nresults: isize) {
        let val = self.stack().get(-(nargs as isize + 1));
        if let LuaValue::Function(c) = val {
            let source = c.proto.source.clone().unwrap(); // TODO
            let line = c.proto.line_defined;
            let last_line = c.proto.last_line_defined;
            println!("call {}<{},{}>", source, line, last_line);
            self.call_lua_closure(nargs, nresults, c);
        } else {
            panic!("not function!");
        }
    }
}

impl LuaState {
    fn get_table_impl(&mut self, t: &LuaValue, k: &LuaValue) -> i8 {
        if let LuaValue::Table(tbl) = t {
            let v = tbl.borrow().get(k);
            let type_id = v.type_id();
            self.stack_mut().push(v);
            type_id
        } else {
            panic!("not a table!") // todo
        }
    }

    fn set_table_impl(t: &LuaValue, k: LuaValue, v: LuaValue) {
        if let LuaValue::Table(tbl) = t {
            tbl.borrow_mut().put(k, v);
        } else {
            panic!("not a table!");
        }
    }

    fn call_lua_closure(&mut self, nargs: usize, nresults: isize, c: Rc<Closure>) {
        let nregs = c.proto.max_stack_size as usize;
        let nparams = c.proto.num_params as usize;
        let is_vararg = c.proto.is_vararg == 1;

        // create new lua stack
        let mut new_stack = LuaStack::new(nregs + 20, c);

        // pass args, pop func
        let mut args = self.stack_mut().pop_n(nargs);
        self.stack_mut().pop(); // pop func
        if nargs > nparams {
            // varargs
            for i in nparams..nargs {
                new_stack.varargs.push(args.pop().unwrap());
            }
            if is_vararg {
                new_stack.varargs.reverse();
            } else {
                new_stack.varargs.clear();
            }
        }
        new_stack.push_n(args, nparams as isize);
        new_stack.set_top(nregs as isize);

        // run closure
        self.push_frame(new_stack);
        self.run_lua_closure();
        new_stack = self.pop_frame();

        // return results
        if nresults != 0 {
            let nrets = new_stack.top() as usize - nregs;
            let results = new_stack.pop_n(nrets);
            self.stack_mut().check(nrets);
            self.stack_mut().push_n(results, nresults);
        }
    }

    fn run_lua_closure(&mut self) {
        loop {
            let instr = self.fetch();
            instr.execute(self);
            // print_stack(instr.opname(), self);
            if instr.opcode() == crate::vm::opcodes::OP_RETURN {
                break;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::api::op::CmpOp;

    use super::*;

    #[test]
    fn test_is_none_or_nil() {
        let mut lua_state = LuaState::new();
        lua_state.push_nil();
        assert_eq!(lua_state.is_none_or_nil(lua_state.get_top()), true);
        lua_state.push_boolean(false);
        assert_eq!(lua_state.is_none_or_nil(lua_state.get_top()), false);
    }

    #[test]
    fn test_is_boolean() {
        let mut lua_state = LuaState::new();
        lua_state.push_nil();
        assert_eq!(lua_state.is_boolean(lua_state.get_top()), false);
        lua_state.push_boolean(false);
        assert_eq!(lua_state.is_boolean(lua_state.get_top()), true);
    }

    #[test]
    fn test_is_integer() {
        let mut lua_state = LuaState::new();
        lua_state.push_nil();
        assert_eq!(lua_state.is_integer(lua_state.get_top()), false);
        lua_state.push_integer(1);
        assert_eq!(lua_state.is_integer(lua_state.get_top()), true);
    }

    #[test]
    fn test_is_number() {
        let mut lua_state = LuaState::new();
        lua_state.push_nil();
        assert_eq!(lua_state.is_number(lua_state.get_top()), false);
        lua_state.push_number(1 as f64);
        assert_eq!(lua_state.is_number(lua_state.get_top()), true);
    }

    #[test]
    fn test_is_string() {
        let mut lua_state = LuaState::new();
        lua_state.push_nil();
        assert_eq!(lua_state.is_string(lua_state.get_top()), false);
        lua_state.push_string("hello".to_string());
        assert_eq!(lua_state.is_string(lua_state.get_top()), true);
    }

    #[test]
    fn test_to_boolean() {
        let mut lua_state = LuaState::new();
        lua_state.push_boolean(false);
        assert_eq!(lua_state.to_boolean(lua_state.get_top()), false);
        lua_state.push_boolean(true);
        assert_eq!(lua_state.to_boolean(lua_state.get_top()), true);
    }

    #[test]
    fn test_to_integer() {
        let mut lua_state = LuaState::new();
        lua_state.push_integer(1);
        assert_eq!(lua_state.to_integer(lua_state.get_top()), 1);
    }

    #[test]
    fn test_to_integerx() {
        let mut lua_state = LuaState::new();
        lua_state.push_integer(1);
        assert_eq!(lua_state.to_integerx(lua_state.get_top()), Some(1));
        lua_state.push_integer(2);
        assert_eq!(lua_state.to_integerx(lua_state.get_top()), Some(2));
        lua_state.push_nil();
        assert_eq!(lua_state.to_integerx(lua_state.get_top()), None);
    }

    #[test]
    fn test_to_number() {
        let mut lua_state = LuaState::new();
        lua_state.push_number(1 as f64);
        assert_eq!(lua_state.to_number(lua_state.get_top()), 1 as f64);
    }

    #[test]
    fn test_to_numberx() {
        let mut lua_state = LuaState::new();
        lua_state.push_number(1 as f64);
        assert_eq!(lua_state.to_numberx(lua_state.get_top()), Some(1 as f64));
        lua_state.push_number(2 as f64);
        assert_eq!(lua_state.to_numberx(lua_state.get_top()), Some(2 as f64));
        lua_state.push_nil();
        assert_eq!(lua_state.to_numberx(lua_state.get_top()), None);
    }

    #[test]
    fn test_to_string() {
        let mut lua_state = LuaState::new();
        lua_state.push_string("hello".to_string());
        assert_eq!(lua_state.to_string(lua_state.get_top()), "hello");
    }

    #[test]
    fn test_to_stringx() {
        let mut lua_state = LuaState::new();
        lua_state.push_string("hello".to_string());
        assert_eq!(
            lua_state.to_stringx(lua_state.get_top()),
            Some("hello".to_string())
        );
        lua_state.push_nil();
        assert_eq!(lua_state.to_stringx(2), None);
    }

    #[test]
    fn test_push_nil() {
        let mut lua_state = LuaState::new();
        lua_state.push_nil();
        assert_eq!(lua_state.type_id(lua_state.get_top()), Type::Nil as i8);
    }

    #[test]
    fn test_push_boolean() {
        let mut lua_state = LuaState::new();
        lua_state.push_boolean(true);
        assert_eq!(lua_state.type_id(lua_state.get_top()), Type::Boolean as i8);
    }

    #[test]
    fn test_push_integer() {
        let mut lua_state = LuaState::new();
        lua_state.push_integer(42);
        assert_eq!(lua_state.type_id(lua_state.get_top()), Type::Number as i8);
    }

    #[test]
    fn test_push_number() {
        let mut lua_state = LuaState::new();
        lua_state.push_number(3.14);
        assert_eq!(lua_state.type_id(lua_state.get_top()), Type::Number as i8);
    }

    #[test]
    fn test_push_string() {
        let mut lua_state = LuaState::new();
        lua_state.push_string("hello".to_string());
        assert_eq!(lua_state.type_id(lua_state.get_top()), Type::String as i8);
    }

    #[test]
    fn test_arith() {
        let mut lua_state = LuaState::new();
        lua_state.push_integer(1);
        lua_state.push_integer(2);
        lua_state.arith(ArithOp::ADD as u8);
        assert_eq!(lua_state.to_integer(lua_state.get_top()), 3);
    }

    #[test]
    fn test_compare() {
        let mut lua_state = LuaState::new();
        lua_state.push_integer(1);
        lua_state.push_integer(2);
        assert_eq!(lua_state.compare(-1, -2, 0), false);
    }

    #[test]
    fn test_len() {
        let mut lua_state = LuaState::new();
        lua_state.push_string("hello".to_string());
        lua_state.len(lua_state.get_top());
        assert_eq!(lua_state.to_integer(lua_state.get_top()), 5);
    }

    #[test]
    fn test_clac() {
        let mut ls = LuaState::new();

        ls.push_integer(1);
        ls.push_string("2.0".to_string());
        ls.push_string("3.0".to_string());
        ls.push_number(4.0);
        print_stack(&ls);

        ls.arith(ArithOp::ADD as u8);
        print_stack(&ls);
        ls.arith(ArithOp::BNOT as u8);
        print_stack(&ls);
        ls.len(2);
        print_stack(&ls);
        ls.concat(3);
        print_stack(&ls);
        let x = ls.compare(1, 2, CmpOp::EQ as u8);
        ls.push_boolean(x);
        print_stack(&ls);
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

    #[test]
    fn test_lua_state_new() {
        let lua_state = LuaState::new();
        assert_eq!(lua_state.pc(), 0);
        assert_eq!(lua_state.stack().top(), 0);
    }

    #[test]
    fn test_lua_state_pc() {
        let lua_state = LuaState::new();
        assert_eq!(lua_state.pc(), 0);
    }

    #[test]
    fn test_lua_state_add_pc() {
        let mut lua_state = LuaState::new();
        lua_state.add_pc(5);
        assert_eq!(lua_state.pc(), 5);
    }

    #[test]
    fn test_lua_state_table() {
        let mut ls = LuaState::new();
        ls.new_table();
        ls.push_integer(1);
        ls.push_integer(2);
        ls.set_table(1);
        ls.push_integer(1);
        ls.get_table(1);
        assert_eq!(ls.to_integer(-1), 2);
    }
}
