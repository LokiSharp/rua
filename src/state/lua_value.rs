use core::fmt;
use core::hash::Hash;
use std::cell::RefCell;
use std::hash::Hasher;
use std::rc::Rc;

use crate::api::r#type::Type;
use crate::binary::chunk::Prototype;

use super::closure::Closure;
use super::lua_table::LuaTable;

#[derive(Clone)]
pub enum LuaValue {
    Nil,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    Str(String),
    Table(Rc<RefCell<LuaTable>>),
    Function(Rc<Closure>),
}

impl fmt::Debug for LuaValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LuaValue::Nil => write!(f, "nil"),
            LuaValue::Boolean(b) => write!(f, "{}", b),
            LuaValue::Number(n) => write!(f, "{}", n),
            LuaValue::Integer(i) => write!(f, "{}", i),
            LuaValue::Str(s) => write!(f, "{:?}", s),
            LuaValue::Table(_) => write!(f, "table"),
            LuaValue::Function(_) => write!(f, "(function)"),
        }
    }
}

impl PartialEq for LuaValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LuaValue::Nil, LuaValue::Nil) => true,
            (LuaValue::Boolean(b1), LuaValue::Boolean(b2)) => b1 == b2,
            (LuaValue::Number(n1), LuaValue::Number(n2)) => n1 == n2,
            (LuaValue::Integer(i1), LuaValue::Integer(i2)) => i1 == i2,
            (LuaValue::Str(s1), LuaValue::Str(s2)) => s1 == s2,
            (LuaValue::Table(t1), LuaValue::Table(t2)) => Rc::ptr_eq(t1, t2),
            _ => false,
        }
    }
}

impl Eq for LuaValue {}

impl Hash for LuaValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            LuaValue::Nil => 0.hash(state),
            LuaValue::Boolean(b) => b.hash(state),
            LuaValue::Number(n) => n.to_bits().hash(state),
            LuaValue::Integer(i) => i.hash(state),
            LuaValue::Str(s) => s.hash(state),
            LuaValue::Table(t) => t.borrow().hash(state),
            LuaValue::Function(c) => c.hash(state),
        }
    }
}

impl LuaValue {
    pub fn is_nil(&self) -> bool {
        match self {
            LuaValue::Nil => true,
            _ => false,
        }
    }

    pub fn type_id(&self) -> i8 {
        match self {
            LuaValue::Nil => Type::Nil as i8,
            LuaValue::Boolean(_) => Type::Boolean as i8,
            LuaValue::Number(_) => Type::Number as i8,
            LuaValue::Integer(_) => Type::Number as i8,
            LuaValue::Str(_) => Type::String as i8,
            LuaValue::Table(_) => Type::Table as i8,
            LuaValue::Function(_) => Type::Function as i8,
        }
    }

    pub fn to_boolean(&self) -> bool {
        match self {
            LuaValue::Nil => false,
            LuaValue::Boolean(b) => *b,
            _ => true,
        }
    }

    pub fn to_number(&self) -> Option<f64> {
        match self {
            LuaValue::Integer(i) => Some(*i as f64),
            LuaValue::Number(n) => Some(*n),
            LuaValue::Str(s) => s.parse::<f64>().ok(), // TODO
            _ => None,
        }
    }

    pub fn to_integer(&self) -> Option<i64> {
        match self {
            LuaValue::Integer(i) => Some(*i),
            LuaValue::Number(n) => float_to_integer(*n),
            LuaValue::Str(s) => string_to_integer(s),
            _ => None,
        }
    }

    pub fn new_table(narr: usize, nrec: usize) -> LuaValue {
        LuaValue::Table(Rc::new(RefCell::new(LuaTable::new(narr, nrec))))
    }

    pub fn new_lua_closure(proto: Rc<Prototype>) -> LuaValue {
        LuaValue::Function(Rc::new(Closure::new(proto)))
    }
}

fn float_to_integer(n: f64) -> Option<i64> {
    let i = n as i64;
    if i as f64 == n {
        Some(i)
    } else {
        None
    }
}

fn string_to_integer(s: &String) -> Option<i64> {
    if let Ok(i) = s.parse::<i64>() {
        Some(i)
    } else if let Ok(n) = s.parse::<f64>() {
        float_to_integer(n)
    } else {
        None
    }
}
