use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn save(file: &mut File, text: &str) {
    let data: Vec<u8> = text.split_whitespace().map(|s| u8::from_str_radix(s, 16).unwrap()).collect::<Vec<u8>>();
    let _ = file.write(&data);
}

// Load a file and convert to hex file
pub fn load(file: &mut File) -> String {
    let mut out = String::new();
    let mut data: Vec<u8> = Vec::<u8>::new();
    file.read_to_end(&mut data).unwrap();
    
    let mut i = 0;
    while i < data.len() {
        let mut s = format!("{:x}", data[i]);
        if s.len() < 2 {
            s.insert(0, '0');
        }
        out.push_str(&s);
        if i < data.len()-1 {
            out.push(' ');
        }
        i += 1;
    }
    println!("{}", out);
    
    out
}
