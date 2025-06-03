mod editor;
mod file;

use std::fs::OpenOptions;

use crate::editor::*;
use crate::file::*;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let filename = args.get(1).expect("Please provide a file name");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(filename)
        .expect("Failed to open file");

    let text = load(&mut file);
    let mut editor = Editor::new(text);
    editor.run();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
        .expect("Failed to open file");

    save(&mut file, &editor.text.join(""));

    print!("{}{}", termion::cursor::Goto(1, 1), termion::clear::All);
}
