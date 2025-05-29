use std::io::{stdin, stdout, Write};

use termion::{input::TermRead, raw::IntoRawMode};

pub fn display(text: &str, cursor_pos: &(u16, u16)) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}{}", termion::clear::All, text, termion::cursor::Goto(cursor_pos.0, cursor_pos.1)).unwrap();
    stdout.flush().unwrap();
}

pub fn handle_input(text: &mut String, cursor_pos: &mut (u16, u16)) -> bool { 
    let _stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            termion::event::Key::Char('\n') => {
                text.push('\n');
                cursor_pos.0 = 1;
                cursor_pos.1 += 1;
            }
            termion::event::Key::Char(c) => {
                text.push(c);
                cursor_pos.0 += 1;
            }
            termion::event::Key::Left => {
                if cursor_pos.0 > 1 {
                    cursor_pos.0 -= 1;
                }
            }
            termion::event::Key::Right => {
                if cursor_pos.0 < text.len() as u16 {
                    cursor_pos.0 += 1;
                }
            }
            termion::event::Key::Up => {
                if cursor_pos.1 > 1 {
                    cursor_pos.1 -= 1;
                    let prev_line_start = text.rfind('\n').unwrap_or(0);
                    cursor_pos.0 = (prev_line_start + 1) as u16;
                }
            }
            termion::event::Key::Down => {
                if cursor_pos.1 < text.lines().count() as u16 {
                    cursor_pos.1 += 1;
                    let next_line_start = text.lines().nth(cursor_pos.1 as usize - 1).map_or(0, |line| line.len());
                    cursor_pos.0 = (next_line_start + 1) as u16;
                }
            }
            termion::event::Key::Ctrl('c') => return false,
            _ => {}
        }
    }
    true
}
