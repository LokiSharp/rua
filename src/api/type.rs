use super::consts::*;

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
#[repr(i8)]
pub enum Type {
    None = LUA_TNONE,
    Nil = LUA_TNIL,
    Boolean = LUA_TBOOLEAN,
    LightUserData = LUA_TLIGHTUSERDATA,
    Number = LUA_TNUMBER,
    String = LUA_TSTRING,
    Table = LUA_TTABLE,
    Function = LUA_TFUNCTION,
    UserData = LUA_TUSERDATA,
    Thread = LUA_TTHREAD,
    NumTypes = LUA_NUMTYPES,
    Proto = LUA_TPROTO,
    DeadKey = LUA_TDEADKEY,
    TolalTypes = LUA_TOLALTYPES,
}

#[allow(dead_code)]
impl Type {
    pub fn from_i8(value: i8) -> Option<Self> {
        match value {
            LUA_TNONE => Some(Type::None),
            LUA_TNIL => Some(Type::Nil),
            LUA_TBOOLEAN => Some(Type::Boolean),
            LUA_TLIGHTUSERDATA => Some(Type::LightUserData),
            LUA_TNUMBER => Some(Type::Number),
            LUA_TSTRING => Some(Type::String),
            LUA_TTABLE => Some(Type::Table),
            LUA_TFUNCTION => Some(Type::Function),
            LUA_TUSERDATA => Some(Type::UserData),
            LUA_TTHREAD => Some(Type::Thread),
            LUA_NUMTYPES => Some(Type::NumTypes),
            LUA_TPROTO => Some(Type::Proto),
            LUA_TDEADKEY => Some(Type::DeadKey),
            LUA_TOLALTYPES => Some(Type::TolalTypes),
            _ => panic!("unexpected Type value: {value:?}"),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(i8)]
pub enum VType {
    VNil = make_variant(Type::Nil, 0),
    VEmpty = make_variant(Type::Nil, 1),
    VAbstKey = make_variant(Type::Nil, 2),
    VFalse = make_variant(Type::Boolean, 0),
    VTrue = make_variant(Type::Boolean, 1),
    VThread = make_variant(Type::Thread, 0),
    VNumInt = make_variant(Type::Number, 0),
    VNumFlt = make_variant(Type::Number, 1),
    VShrStr = make_variant(Type::String, 0),
    VLngStr = make_variant(Type::String, 1),
    VLightUserData = make_variant(Type::LightUserData, 0),
    VUserData = make_variant(Type::UserData, 0),
    VProto = make_variant(Type::Proto, 0),
    VUpval = make_variant(Type::NumTypes, 0),
    VLCL = make_variant(Type::Function, 0),
    VLCF = make_variant(Type::Function, 1),
    VCCL = make_variant(Type::Function, 2),
    VTable = make_variant(Type::Table, 0),
}

impl VType {
    pub fn from_u8(value: u8) -> Option<Self> {
        for variant in &[
            VType::VNil,
            VType::VEmpty,
            VType::VAbstKey,
            VType::VFalse,
            VType::VTrue,
            VType::VThread,
            VType::VNumInt,
            VType::VNumFlt,
            VType::VShrStr,
            VType::VLngStr,
            VType::VLightUserData,
            VType::VUserData,
            VType::VProto,
            VType::VUpval,
            VType::VLCL,
            VType::VLCF,
            VType::VCCL,
            VType::VTable,
        ] {
            if *variant as u8 == value {
                return Some(*variant);
            }
        }
        panic!("unexpected VType value: {value:?}")
    }
}

const fn make_variant(t: Type, v: i8) -> i8 {
    (t as i8) | ((v) << 4)
}
