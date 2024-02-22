use crate::binary::chunk;

use crate::api::r#type::VType;

pub struct Reader {
    data: Vec<u8>,
    pos: usize,
}

impl Reader {
    pub fn new(data: Vec<u8>) -> Self {
        Reader { data, pos: 0 }
    }

    pub fn read_byte(&mut self) -> u8 {
        let b = self.data[self.pos];
        self.pos += 1;
        b
    }

    fn read_u32(&mut self) -> u32 {
        let a0 = self.read_byte() as u32;
        let a1 = self.read_byte() as u32;
        let a2 = self.read_byte() as u32;
        let a3 = self.read_byte() as u32;
        (a3 << 24) | (a2 << 16) | (a1 << 8) | a0
    }

    fn read_u64(&mut self) -> u64 {
        let a0 = self.read_u32() as u64;
        let a1 = self.read_u32() as u64;
        (a1 << 32) | a0
    }

    fn read_lua_integer(&mut self) -> i64 {
        self.read_u64() as i64
    }

    fn read_lua_number(&mut self) -> f64 {
        use std::f64; // TODO
        f64::from_bits(self.read_u64())
    }

    fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut vec = Vec::new();
        for _i in 0..n {
            vec.push(self.read_byte());
        }
        vec
    }

    fn read_size(&mut self) -> usize {
        let mut x = 0usize;
        let mut b;
        let mut limit = std::usize::MAX;
        limit >>= 7;
        loop {
            b = self.read_byte();
            if x >= limit {
                panic!("integer overflow");
            }
            x = (x << 7) | ((b & 0x7f) as usize);
            if b & 0x80 != 0 {
                break;
            }
        }
        x
    }

    fn read_string(&mut self) -> String {
        self.read_string0().unwrap_or_else(|| String::new())
    }

    fn read_string0(&mut self) -> Option<String> {
        let size = self.read_size();
        if size == 0 {
            return None;
        }
        let bytes = self.read_bytes(size - 1);
        let string = String::from_utf8(bytes);
        string.ok() // Some(string.unwrap())
    }

    fn read_vec<T, F>(&mut self, f: F) -> Vec<T>
    where
        F: Fn(&mut Reader) -> T,
    {
        let n = self.read_size();
        let mut vec = Vec::with_capacity(n);
        for _i in 0..n {
            vec.push(f(self));
        }
        vec
    }

    pub fn check_header(&mut self) {
        assert_eq!(
            self.read_bytes(4),
            chunk::LUA_SIGNATURE,
            "not a precompiled chunk!"
        );
        assert_eq!(self.read_byte(), chunk::LUAC_VERSION, "version mismatch!");
        assert_eq!(self.read_byte(), chunk::LUAC_FORMAT, "format mismatch!");
        assert_eq!(self.read_bytes(6), chunk::LUAC_DATA, "corrupted!");
        assert_eq!(
            self.read_byte(),
            chunk::INSTRUCTION_SIZE,
            "instruction size mismatch!"
        );
        assert_eq!(
            self.read_byte(),
            chunk::LUA_INTEGER_SIZE,
            "lua_Integer size mismatch!"
        );
        assert_eq!(
            self.read_byte(),
            chunk::LUA_NUMBER_SIZE,
            "lua_Number size mismatch!"
        );
        assert_eq!(
            self.read_lua_integer(),
            chunk::LUAC_INT,
            "endianness mismatch!"
        );
        assert_eq!(
            self.read_lua_number(),
            chunk::LUAC_NUM,
            "float format mismatch!"
        );
    }

    pub fn read_proto(&mut self) -> chunk::Prototype {
        self.read_proto0(None)
    }

    fn read_proto0(&mut self, parent_source: Option<String>) -> chunk::Prototype {
        let source = self.read_string0().or(parent_source);
        chunk::Prototype {
            source: source.clone(), // debug
            line_defined: self.read_size(),
            last_line_defined: self.read_size(),
            num_params: self.read_byte(),
            is_vararg: self.read_byte(),
            max_stack_size: self.read_byte(),
            code: self.read_vec(|r| r.read_u32()),
            constants: self.read_vec(|r| r.read_constant()),
            upvalues: self.read_vec(|r| r.read_upvalue()),
            protos: self.read_vec(|r| r.read_proto0(source.clone())),
            line_info: self.read_vec(|r| r.read_byte() as i8), // debug
            abs_line_info: self.read_vec(|r| r.read_abs_line_info()), // debug
            loc_vars: self.read_vec(|r| r.read_loc_var()),     // debug
            upvalue_names: self.read_vec(|r| r.read_string()), // debug
        }
    }

    fn read_constant(&mut self) -> chunk::Constant {
        let tag = self.read_byte();
        match VType::from_u8(tag) {
            Some(VType::VNil) => chunk::Constant::Nil,
            Some(VType::VFalse) => chunk::Constant::Boolean(false),
            Some(VType::VTrue) => chunk::Constant::Boolean(true),
            Some(VType::VNumInt) => chunk::Constant::Integer(self.read_lua_integer()),
            Some(VType::VNumFlt) => chunk::Constant::Number(self.read_lua_number()),
            Some(VType::VShrStr) => chunk::Constant::Str(self.read_string()),
            Some(VType::VLngStr) => chunk::Constant::Str(self.read_string()),
            _ => panic!("unexpected tag: {tag:?}"),
        }
    }

    fn read_upvalue(&mut self) -> chunk::Upvalue {
        chunk::Upvalue {
            instack: self.read_byte(),
            idx: self.read_byte(),
            kind: self.read_byte(),
        }
    }

    fn read_abs_line_info(&mut self) -> chunk::AbsLineInfo {
        chunk::AbsLineInfo {
            pc: self.read_size(),
            line: self.read_size(),
        }
    }

    fn read_loc_var(&mut self) -> chunk::LocVar {
        chunk::LocVar {
            var_name: self.read_string(),
            start_pc: self.read_size(),
            end_pc: self.read_size(),
        }
    }
}
