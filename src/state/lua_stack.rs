use std::rc::Rc;

use super::{closure::Closure, lua_value::LuaValue};

/// `LuaStack` 是一个用于操作 Lua 栈的结构体。
#[derive(Debug)]
pub struct LuaStack {
    vec: Vec<LuaValue>,
    pub closure: Rc<Closure>,
    pub varargs: Vec<LuaValue>,
    pub pc: isize,
}

impl LuaStack {
    /// 创建一个新的 `LuaStack`，预分配指定大小的空间。
    pub fn new(size: usize, closure: Rc<Closure>) -> Self {
        LuaStack {
            vec: Vec::with_capacity(size),
            closure: closure,
            varargs: Vec::new(),
            pc: 0,
        }
    }

    /// 获取栈顶的索引。
    pub fn top(&self) -> isize {
        self.vec.len() as isize
    }

    /// 检查栈是否有足够的空间来存储 `n` 个元素，如果没有则分配更多的空间。
    pub fn check(&mut self, n: usize) {
        self.vec.reserve(n);
    }

    /// 将一个值推送到栈顶。
    pub fn push(&mut self, val: LuaValue) {
        self.vec.push(val);
    }

    /// 将 n 个值推送到栈顶。
    pub fn push_n(&mut self, mut vals: Vec<LuaValue>, n: isize) {
        vals.reverse();
        let nvals = vals.len();
        let un = if n < 0 { nvals } else { n as usize };

        for i in 0..un {
            if i < nvals {
                self.push(vals.pop().unwrap());
            } else {
                self.push(LuaValue::Nil);
            }
        }
    }

    /// 从栈顶弹出一个值。
    pub fn pop(&mut self) -> LuaValue {
        self.vec.pop().unwrap()
    }

    /// 从栈顶弹出 n 个值。
    pub fn pop_n(&mut self, n: usize) -> Vec<LuaValue> {
        let mut vec = Vec::with_capacity(n);
        for _ in 0..n {
            vec.push(self.pop());
        }
        vec.reverse();
        vec
    }

    /// 将一个相对索引转换为绝对索引。
    pub fn abs_index(&self, idx: isize) -> isize {
        if idx >= 0 {
            idx
        } else {
            idx + self.top() + 1
        }
    }

    /// 检查一个索引是否有效。
    pub fn is_valid(&self, idx: isize) -> bool {
        let abs_idx = self.abs_index(idx);
        abs_idx > 0 && abs_idx <= self.top()
    }

    /// 获取指定索引的值。
    pub fn get(&self, idx: isize) -> LuaValue {
        let abs_idx = self.abs_index(idx);
        if abs_idx > 0 && abs_idx <= self.top() {
            let idx = abs_idx as usize - 1;
            self.vec[idx].clone() // TODO
        } else {
            LuaValue::Nil
        }
    }

    /// 设置指定索引的值。
    pub fn set(&mut self, idx: isize, val: LuaValue) {
        let abs_idx = self.abs_index(idx);
        if abs_idx > 0 && abs_idx <= self.top() {
            let idx = abs_idx as usize - 1;
            self.vec[idx] = val;
        } else {
            panic!("invalid index!");
        }
    }

    /// 设置栈顶的位置。
    pub fn set_top(&mut self, idx: isize) {
        let new_top = self.abs_index(idx);
        if new_top < 0 {
            panic!("stack underflow!");
        }

        let n = self.top() - new_top;
        if n > 0 {
            for _ in 0..n {
                self.pop();
            }
        } else if n < 0 {
            for _ in n..0 {
                self.push(LuaValue::Nil);
            }
        }
    }

    /// 反转栈中从 `from` 到 `to` 的元素。
    pub fn reverse(&mut self, mut from: usize, mut to: usize) {
        while from < to {
            self.vec.swap(from, to);
            from += 1;
            to -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let stack = LuaStack::new(10, Rc::new(Closure::new(Rc::new(Default::default()))));
        assert_eq!(stack.top(), 0);
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack = LuaStack::new(10, Rc::new(Closure::new(Rc::new(Default::default()))));
        stack.push(LuaValue::Number(42.0));
        assert_eq!(stack.top(), 1);
        stack.pop();
        assert_eq!(stack.top(), 0);
    }

    #[test]
    fn test_push_and_pop_n() {
        let mut stack = LuaStack::new(10, Rc::new(Closure::new(Rc::new(Default::default()))));
        stack.push_n(
            vec![
                LuaValue::Number(0.0),
                LuaValue::Number(1.0),
                LuaValue::Number(2.0),
            ],
            3,
        );
        assert_eq!(stack.top(), 3);
        stack.pop_n(3);
        assert_eq!(stack.top(), 0);
    }

    #[test]
    fn test_abs_index() {
        let stack = LuaStack::new(10, Rc::new(Closure::new(Rc::new(Default::default()))));
        assert_eq!(stack.abs_index(1), 1);
        assert_eq!(stack.abs_index(-1), 0);
    }

    #[test]
    fn test_is_valid() {
        let mut stack = LuaStack::new(10, Rc::new(Closure::new(Rc::new(Default::default()))));
        stack.push(LuaValue::Boolean(true));
        assert_eq!(stack.is_valid(1), true);
        assert_eq!(stack.is_valid(2), false);
        assert_eq!(stack.is_valid(-1), true);
        assert_eq!(stack.is_valid(-2), false);
    }

    #[test]
    fn test_get_and_set() {
        let mut stack = LuaStack::new(10, Rc::new(Closure::new(Rc::new(Default::default()))));
        stack.push(LuaValue::Str("hello".to_string()));
        stack.push(LuaValue::Number(42.0));
        assert_eq!(stack.get(1), LuaValue::Str("hello".to_string()));
        assert_eq!(stack.get(2), LuaValue::Number(42.0));
        stack.set(1, LuaValue::Boolean(true));
        stack.set(2, LuaValue::Nil);
        assert_eq!(stack.get(1), LuaValue::Boolean(true));
        assert_eq!(stack.get(2), LuaValue::Nil);
    }

    #[test]
    fn test_reverse() {
        let mut stack = LuaStack::new(10, Rc::new(Closure::new(Rc::new(Default::default()))));
        stack.push(LuaValue::Number(1.0));
        stack.push(LuaValue::Number(2.0));
        stack.push(LuaValue::Number(3.0));
        stack.reverse(0, 2);
        assert_eq!(stack.get(1), LuaValue::Number(3.0));
        assert_eq!(stack.get(2), LuaValue::Number(2.0));
        assert_eq!(stack.get(3), LuaValue::Number(1.0));
    }
}
