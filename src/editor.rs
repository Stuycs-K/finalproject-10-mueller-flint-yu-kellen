use std::io::{Write, stdin, stdout};

use termion::{input::TermRead, raw::IntoRawMode};

pub struct Editor {
    pub text: String,
    cursor_pos: (u16, u16),
}

impl Editor {
    pub fn new(text: String) -> Self {
        Editor {
            text,
            cursor_pos: (1, 1),
        }
    }

    pub fn display(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            self.text,
            termion::cursor::Goto(self.cursor_pos.0, self.cursor_pos.1)
        )
        .unwrap();
        stdout.flush().unwrap();
    }

    pub fn handle_input(&mut self) -> bool {
        let _stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                termion::event::Key::Char('\n') => {
                    self.text.push('\n');
                    self.cursor_pos.0 = 1;
                    self.cursor_pos.1 += 1;
                }
                termion::event::Key::Char(c) => {
                    self.text.push(c);
                    self.cursor_pos.0 += 1;
                }
                termion::event::Key::Left => {
                    if self.cursor_pos.0 > 1 {
                        self.cursor_pos.0 -= 1;
                    }
                }
                termion::event::Key::Right => {
                    if self.cursor_pos.0 < self.text.len() as u16 {
                        self.cursor_pos.0 += 1;
                    }
                }
                termion::event::Key::Up => {
                    if self.cursor_pos.1 > 1 {
                        self.cursor_pos.1 -= 1;
                        let prev_line_start = self.text.rfind('\n').unwrap_or(0);
                        self.cursor_pos.0 = (prev_line_start + 1) as u16;
                    }
                }
                termion::event::Key::Down => {
                    if self.cursor_pos.1 < self.text.lines().count() as u16 {
                        self.cursor_pos.1 += 1;
                        let next_line_start = self
                            
                            
                            
                            .text
                            .lines()
                            .nth(self.cursor_pos.1 as usize - 1)
                            .map_or(0, |line| line.len());
                        self.cursor_pos.0 = (next_line_start + 1) as u16;
                    }
                }
                termion::event::Key::Ctrl('c') => return false,
                _ => {}
            }
        }
        true
    }
}
