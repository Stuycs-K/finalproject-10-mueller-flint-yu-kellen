mod io;
mod file;

use crate::io::*;
use crate::file::*;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let file = args.get(1).expect("Please provide a file name");
    let file = std::fs::File::open(file).expect("Failed to open file");
    let mut text = load(&file);

    while handle_input(&mut text) {
        display(&text);
    }

    save(
        file,
        &text,
    );
}
