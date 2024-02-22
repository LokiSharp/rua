use crate::{api::LuaAPI, binary::object::Type};

use super::{lua_stack::LuaStack, lua_value::LuaValue};

pub struct LuaState {
    stack: LuaStack,
}

impl LuaState {
    pub fn new() -> LuaState {
        LuaState {
            stack: LuaStack::new(20),
        }
    }
}

impl LuaAPI for LuaState {
    fn get_top(&self) -> isize {
        self.stack.top()
    }

    fn abs_index(&self, idx: isize) -> isize {
        self.stack.abs_index(idx)
    }

    fn check_stack(&mut self, n: usize) -> bool {
        self.stack.check(n);
        true
    }

    fn pop(&mut self, n: usize) {
        for _ in 0..n {
            self.stack.pop();
        }
    }

    fn copy(&mut self, from_idx: isize, to_idx: isize) {
        let val = self.stack.get(from_idx);
        self.stack.set(to_idx, val);
    }

    fn push_value(&mut self, idx: isize) {
        let val = self.stack.get(idx);
        self.stack.push(val);
    }

    fn replace(&mut self, idx: isize) {
        let val = self.stack.pop();
        self.stack.set(idx, val);
    }

    fn insert(&mut self, idx: isize) {
        self.rotate(idx, 1);
    }

    fn remove(&mut self, idx: isize) {
        self.rotate(idx, -1);
        self.pop(1);
    }

    fn rotate(&mut self, idx: isize, n: isize) {
        let abs_idx = self.stack.abs_index(idx);
        if abs_idx < 0 || !self.stack.is_valid(abs_idx) {
            panic!("invalid index!");
        }

        let t = self.stack.top() - 1; /* end of stack segment being rotated */
        let p = abs_idx - 1; /* start of segment */
        let m = if n >= 0 { t - n } else { p - n - 1 }; /* end of prefix */
        self.stack.reverse(p as usize, m as usize); /* reverse the prefix with length 'n' */
        self.stack.reverse(m as usize + 1, t as usize); /* reverse the suffix */
        self.stack.reverse(p as usize, t as usize); /* reverse the entire segment */
    }

    fn set_top(&mut self, idx: isize) {
        let new_top = self.stack.abs_index(idx);
        if new_top < 0 {
            panic!("stack underflow!");
        }

        let n = self.stack.top() - new_top;
        if n > 0 {
            for _ in 0..n {
                self.stack.pop();
            }
        } else if n < 0 {
            for _ in n..0 {
                self.stack.push(LuaValue::Nil);
            }
        }
    }

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

    fn type_id(&self, idx: isize) -> i8 {
        if self.stack.is_valid(idx) {
            self.stack.get(idx).type_id()
        } else {
            Type::None as i8
        }
    }

    fn is_none(&self, idx: isize) -> bool {
        self.type_id(idx) == Type::None as i8
    }

    fn is_nil(&self, idx: isize) -> bool {
        self.type_id(idx) == Type::Nil as i8
    }

    fn is_none_or_nil(&self, idx: isize) -> bool {
        self.is_none(idx) || self.is_nil(idx)
    }

    fn is_boolean(&self, idx: isize) -> bool {
        self.type_id(idx) == Type::Boolean as i8
    }

    fn is_integer(&self, idx: isize) -> bool {
        match self.stack.get(idx) {
            LuaValue::Integer(_) => true,
            _ => false,
        }
    }

    fn is_number(&self, idx: isize) -> bool {
        self.to_numberx(idx).is_some()
    }

    fn is_string(&self, idx: isize) -> bool {
        let t = self.type_id(idx);
        t == Type::String as i8 || t == Type::Number as i8
    }

    fn is_table(&self, idx: isize) -> bool {
        self.type_id(idx) == Type::Table as i8
    }

    fn is_thread(&self, idx: isize) -> bool {
        self.type_id(idx) == Type::Thread as i8
    }

    fn is_function(&self, idx: isize) -> bool {
        self.type_id(idx) == Type::Function as i8
    }

    fn to_boolean(&self, idx: isize) -> bool {
        self.stack.get(idx).to_boolean()
    }

    fn to_integer(&self, idx: isize) -> i64 {
        self.to_integerx(idx).unwrap()
    }

    fn to_integerx(&self, idx: isize) -> Option<i64> {
        match self.stack.get(idx) {
            LuaValue::Integer(n) => Some(n),
            LuaValue::Number(n) => Some(n as i64),
            _ => None,
        }
    }

    fn to_number(&self, idx: isize) -> f64 {
        self.to_numberx(idx).unwrap()
    }

    fn to_numberx(&self, idx: isize) -> Option<f64> {
        match self.stack.get(idx) {
            LuaValue::Integer(n) => Some(n as f64),
            LuaValue::Number(n) => Some(n),
            _ => None,
        }
    }

    fn to_string(&self, idx: isize) -> String {
        self.to_stringx(idx).unwrap()
    }

    fn to_stringx(&self, idx: isize) -> Option<String> {
        match self.stack.get(idx) {
            LuaValue::Str(s) => Some(s),
            LuaValue::Number(n) => Some(n.to_string()),
            LuaValue::Integer(n) => Some(n.to_string()),
            _ => None,
        }
    }

    fn push_nil(&mut self) {
        self.stack.push(LuaValue::Nil);
    }

    fn push_boolean(&mut self, b: bool) {
        self.stack.push(LuaValue::Boolean(b));
    }

    fn push_integer(&mut self, n: i64) {
        self.stack.push(LuaValue::Integer(n));
    }

    fn push_number(&mut self, n: f64) {
        self.stack.push(LuaValue::Number(n));
    }

    fn push_string(&mut self, s: String) {
        self.stack.push(LuaValue::Str(s));
    }
}
#[cfg(test)]
mod tests {
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
}
