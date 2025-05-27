mod io;
mod file;

use std::fs::OpenOptions;

use crate::io::*;
use crate::file::*;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let file = args.get(1).expect("Please provide a file name");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(file)
        .expect("Failed to open file");
    let mut text = load(&file);

    while handle_input(&mut text) {
        display(&text);
    }

    save(
        file,
        &text,
    );
}
