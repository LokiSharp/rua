mod arith_ops;
mod cmp_ops;
mod lua_stack;
mod lua_state;
pub mod lua_value;
mod math;

pub use self::lua_state::LuaState;
use crate::{api::LuaAPI, binary::chunk::Prototype};

pub fn new_lua_state(stack_size: usize, proto: Prototype) -> LuaState {
    LuaState::new(stack_size, proto)
}

mod tests {
    use crate::api::r#type::Type;

    use super::*;
    #[test]
    fn test_lua_state() {
        let mut ls = new_lua_state(20, Prototype::default());
        ls.push_boolean(true);
        print_stack(&ls);
        ls.push_integer(10);
        print_stack(&ls);
        ls.push_nil();
        print_stack(&ls);
        ls.push_string("hello".to_string());
        print_stack(&ls);
        ls.push_value(-4);
        print_stack(&ls);
        ls.replace(3);
        print_stack(&ls);
        ls.set_top(6);
        print_stack(&ls);
        ls.remove(-3);
        print_stack(&ls);
        ls.set_top(-5);
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
}
