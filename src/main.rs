mod editor;
mod file;

use std::fs::OpenOptions;

use crate::editor::*;
use crate::file::*;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let file = args.get(1).expect("Please provide a file name");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(file)
        .expect("Failed to open file");

    let text = load(&mut file);
    let mut editor = Editor::new(text);
    editor.run();

    save(&mut file, &editor.text.join(""));

    println!("{}{}", termion::cursor::Goto(1, 1), termion::clear::All);
}
