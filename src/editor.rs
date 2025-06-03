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
    pub text: Vec<String>,
    cursor_pos: (u16, u16),
    state: Mode,
    stdin: AsyncReader,
    stdout: RawTerminal<std::io::Stdout>,
}

impl Editor {
    pub fn new(text: String) -> Self {
        Editor {
            text: vec![text],
            state: Mode::Read,
            cursor_pos: (0, 0),
            stdin: termion::async_stdin(),
            stdout: stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn run(&mut self) {
        self.display();
        while self.handle_input() {}
    }

    fn display(&mut self) {
        let term_width = termion::terminal_size().unwrap().0;
        let mut rows_before = 0;
        for line in self.text.iter().take(self.cursor_pos.1 as usize) {
            rows_before += line.len() as u16 / term_width;
            if line.len() as u16 % term_width != 0 {
                rows_before += 1;
            }
        }
        let cursor_row = rows_before + self.cursor_pos.0 / term_width;
        let cursor_col = self.cursor_pos.0 % term_width;
        write!(
            self.stdout,
            "{}{}{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All,
            self.text.join("\n\r"),
            termion::cursor::Goto(cursor_col + 1, cursor_row + 1),
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    fn read_handle(&mut self, c: Key) -> bool {
        match c {
            Key::Char('h') => {
                if self.cursor_pos.0 > 0 {
                    self.cursor_pos.0 -= 1;
                }
            }
            Key::Char('l') => {
                if self.cursor_pos.0 < self.text[self.cursor_pos.1 as usize].len() as u16 {
                    self.cursor_pos.0 += 1;
                }
            }
            Key::Char('k') => {
                if self.cursor_pos.1 > 0 {
                    self.cursor_pos.1 -= 1;
                    self.cursor_pos.0 = self
                        .cursor_pos
                        .0
                        .min(self.text[self.cursor_pos.1 as usize].len() as u16 - 1);
                }
            }
            Key::Char('j') => {
                if self.cursor_pos.1 + 1 < self.text.len() as u16 {
                    self.cursor_pos.1 += 1;
                    self.cursor_pos.0 = self
                        .cursor_pos
                        .0
                        .min(self.text[self.cursor_pos.1 as usize].len() as u16 - 1);
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
                let new =
                    self.text[self.cursor_pos.1 as usize].split_off(self.cursor_pos.0 as usize);
                self.text[self.cursor_pos.1 as usize].push(' ');
                self.text.insert(self.cursor_pos.1 as usize + 1, new);
                self.cursor_pos.1 += 1;
                self.cursor_pos.0 = 0;
            }
            Key::Char(c @ ('0'..='9' | 'a'..='f' | 'A'..='F' | ' ')) => {
                self.text[self.cursor_pos.1 as usize].insert(self.cursor_pos.0 as usize, c);
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
