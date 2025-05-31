use std::io::{Write, stdin, stdout};
use termion::{input::TermRead, raw::IntoRawMode};

enum Mode {
    Read,
    Write,
}

pub struct Editor {
    pub text: String,
    cursor_pos: (u16, u16),
    state: Mode,
}

impl Editor {
    pub fn new(text: String) -> Self {
        Editor {
            text,
            state: Mode::Read,
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

    fn read_handle(&mut self, c: termion::event::Key) -> bool {
        match c {
            termion::event::Key::Char('h') => {
                if self.cursor_pos.0 > 1 {
                    self.cursor_pos.0 -= 1;
                }
            }
            termion::event::Key::Char('l') => {
                if self.cursor_pos.0 < self.text.len() as u16 {
                    self.cursor_pos.0 += 1;
                }
            }
            termion::event::Key::Char('k') => {
                if self.cursor_pos.1 > 1 {
                    self.cursor_pos.1 -= 1;
                    let prev_line_start = self.text.rfind('\n').unwrap_or(0);
                    self.cursor_pos.0 = (prev_line_start + 1) as u16;
                }
            }
            termion::event::Key::Char('j') => {
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
            termion::event::Key::Char('i') => {
                self.state = Mode::Write;
            }
            termion::event::Key::Char('q') => return false,
            _ => {}
        }
        true
    }

    fn write_handle(&mut self, c: termion::event::Key) -> bool {
        match c {
            termion::event::Key::Esc => {
                self.state = Mode::Read;
            }
            termion::event::Key::Char('\n') => {
                self.text.push('\n');
                self.cursor_pos.0 = 1;
                self.cursor_pos.1 += 1;
            }
            termion::event::Key::Char(c) => {
                self.text.push(c);
                self.cursor_pos.0 += 1;
            }
            termion::event::Key::Ctrl('c') => return false,
            _ => {}
        }
        true
    }

    pub fn handle_input(&mut self) -> bool {
        let _stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        for c in stdin.keys() {
            if match self.state {
                Mode::Read => self.read_handle(c.unwrap()),
                Mode::Write => self.write_handle(c.unwrap()),
            } {
                return false;
            }
        }
        true
    }
}
