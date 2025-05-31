use std::io::{Read, Write, stdout};
use termion::AsyncReader;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

enum Mode {
    Read,
    Write,
}

pub struct Editor {
    pub text: String,
    cursor_pos: (u16, u16),
    state: Mode,
    stdin: AsyncReader,
    stdout: RawTerminal<std::io::Stdout>,
}

impl Editor {
    pub fn new(text: String) -> Self {
        Editor {
            text,
            state: Mode::Read,
            cursor_pos: (1, 1),
            stdin: termion::async_stdin(),
            stdout: stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn run(&mut self) {
        self.display();
        while self.handle_input() {}
    }

    fn display(&mut self) {
        write!(
            self.stdout,
            "{}{}{}",
            termion::clear::All,
            self.text,
            termion::cursor::Goto(self.cursor_pos.0, self.cursor_pos.1)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    fn read_handle(&mut self, c: Key) -> bool {
        match c {
            Key::Char('h') => {
                if self.cursor_pos.0 > 1 {
                    self.cursor_pos.0 -= 1;
                }
            }
            Key::Char('l') => {
                if self.cursor_pos.0 < self.text.len() as u16 {
                    self.cursor_pos.0 += 1;
                }
            }
            Key::Char('k') => {
                if self.cursor_pos.1 > 1 {
                    self.cursor_pos.1 -= 1;
                    let prev_line_start = self.text.rfind('\n').unwrap_or(0);
                    self.cursor_pos.0 = (prev_line_start + 1) as u16;
                }
            }
            Key::Char('j') => {
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
            Key::Char('i') => {
                self.state = Mode::Write;
            }
            Key::Ctrl('c') | Key::Char('q') => return false,
            _ => {}
        }
        true
    }

    fn write_handle(&mut self, c: Key) -> bool {
        match c {
            Key::Esc => {
                self.state = Mode::Read;
            }
            Key::Char('\n') => {
                self.text.push('\n');
                self.text.push('\r');
                self.cursor_pos.0 = 1;
                self.cursor_pos.1 += 1;
            }
            Key::Char(c @ ('0'..='9' | 'a'..='f' | 'A'..='F' | ' ' | '\t')) => {
                self.text.push(c);
                self.cursor_pos.0 += 1;
            }
            Key::Ctrl('c') => return false,
            _ => {}
        }
        true
    }

    fn handle_input(&mut self) -> bool {
        for c in self.stdin.by_ref().keys().collect::<Vec<_>>() {
            if !match self.state {
                Mode::Read => self.read_handle(c.unwrap()),
                Mode::Write => self.write_handle(c.unwrap()),
            } {
                return false;
            }
            self.display();
        }
        true
    }
}
