use std::fs::File;
use std::io::Read;

pub fn save(file: File, text: &str) {
}

// Load a file and convert to hex file
pub fn load(file: &mut File) -> String {
    let mut out = String::new();
    let mut data: Vec<u8> = Vec::<u8>::new();
    file.read_to_end(&mut data);
    return out;
}
