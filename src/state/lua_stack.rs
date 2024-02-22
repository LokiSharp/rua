use super::lua_value::LuaValue;

pub struct LuaStack {
    vec: Vec<LuaValue>,
}

impl LuaStack {
    pub fn new(size: usize) -> Self {
        LuaStack {
            vec: Vec::with_capacity(size),
        }
    }

    pub fn top(&self) -> isize {
        self.vec.len() as isize
    }

    pub fn check(&mut self, n: usize) {
        self.vec.reserve(n);
    }

    pub fn push(&mut self, val: LuaValue) {
        self.vec.push(val);
    }

    pub fn pop(&mut self) -> LuaValue {
        self.vec.pop().unwrap()
    }

    pub fn abs_index(&self, idx: isize) -> isize {
        if idx >= 0 {
            idx
        } else {
            idx + self.top() + 1
        }
    }

    pub fn is_valid(&self, idx: isize) -> bool {
        let abs_idx = self.abs_index(idx);
        abs_idx > 0 && abs_idx <= self.top()
    }

    pub fn get(&self, idx: isize) -> LuaValue {
        let abs_idx = self.abs_index(idx);
        if abs_idx > 0 && abs_idx <= self.top() {
            let idx = abs_idx as usize - 1;
            self.vec[idx].clone() // TODO
        } else {
            LuaValue::Nil
        }
    }

    pub fn set(&mut self, idx: isize, val: LuaValue) {
        let abs_idx = self.abs_index(idx);
        if abs_idx > 0 && abs_idx <= self.top() {
            let idx = abs_idx as usize - 1;
            self.vec[idx] = val;
        } else {
            panic!("invalid index!");
        }
    }

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
        let stack = LuaStack::new(10);
        assert_eq!(stack.top(), 0);
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack = LuaStack::new(10);
        stack.push(LuaValue::Number(42.0));
        assert_eq!(stack.top(), 1);
        stack.pop();
        assert_eq!(stack.top(), 0);
    }

    #[test]
    fn test_abs_index() {
        let stack = LuaStack::new(10);
        assert_eq!(stack.abs_index(1), 1);
        assert_eq!(stack.abs_index(-1), 0);
    }

    #[test]
    fn test_is_valid() {
        let mut stack = LuaStack::new(10);
        stack.push(LuaValue::Boolean(true));
        assert_eq!(stack.is_valid(1), true);
        assert_eq!(stack.is_valid(2), false);
        assert_eq!(stack.is_valid(-1), true);
        assert_eq!(stack.is_valid(-2), false);
    }

    #[test]
    fn test_get_and_set() {
        let mut stack = LuaStack::new(10);
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
        let mut stack = LuaStack::new(10);
        stack.push(LuaValue::Number(1.0));
        stack.push(LuaValue::Number(2.0));
        stack.push(LuaValue::Number(3.0));
        stack.reverse(0, 2);
        assert_eq!(stack.get(1), LuaValue::Number(3.0));
        assert_eq!(stack.get(2), LuaValue::Number(2.0));
        assert_eq!(stack.get(3), LuaValue::Number(1.0));
    }
}
