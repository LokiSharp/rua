use std::rc::Rc;

pub mod chunk;
mod reader;

pub fn undump(data: Vec<u8>) -> Rc<chunk::Prototype> {
    let mut r = reader::Reader::new(data);
    r.check_header();
    r.read_byte(); // size_upvalues
    r.read_proto()
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

        let result = undump(data);
        dbg!("{:?}", result);
    }
}
