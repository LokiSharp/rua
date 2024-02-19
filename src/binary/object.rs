#[derive(Copy, Clone)]
#[allow(dead_code)]
#[repr(i8)]
pub enum Type {
    None = -0x01,
    Nil = 0x00,
    Boolean = 0x01,
    LightUserData = 0x02,
    Number = 0x03,
    String = 0x04,
    Table = 0x05,
    Function = 0x06,
    UserData = 0x07,
    Thread = 0x08,
    NumTypes = 0x09,
    Proto = Type::NumTypes as i8 + 1,
    DeadKey = Type::NumTypes as i8 + 2,
    OtalTypes = Type::Proto as i8 + 2,
}

#[allow(dead_code)]
impl Type {
    pub fn from_i8(value: i8) -> Option<Self> {
        match value {
            -0x01 => Some(Type::None),
            0x00 => Some(Type::Nil),
            0x01 => Some(Type::Boolean),
            0x02 => Some(Type::LightUserData),
            0x03 => Some(Type::Number),
            0x04 => Some(Type::String),
            0x05 => Some(Type::Table),
            0x06 => Some(Type::Function),
            0x07 => Some(Type::UserData),
            0x08 => Some(Type::Thread),
            0x09 => Some(Type::NumTypes),
            0x0A => Some(Type::Proto),
            0x0B => Some(Type::DeadKey),
            0x0C => Some(Type::OtalTypes),
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
