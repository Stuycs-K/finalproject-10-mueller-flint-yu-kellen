mod io;
mod file;

use std::fs::OpenOptions;

use crate::io::*;
use crate::file::*;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let mut file = args.get(1).expect("Please provide a file name");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(file)
        .expect("Failed to open file");
    let mut cursor_pos = (1, 1);
    let mut text = load(&mut file);

    while handle_input(&mut text, &mut cursor_pos) {
        display(&text, &cursor_pos);
    }

    save(
        file,
        &text,
    );
}
