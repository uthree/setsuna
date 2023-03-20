use std::ffi::OsStr;
use std::fs;

#[derive(Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Position { x, y }
    }
}

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Command,
}

impl Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::Insert => {
                return String::from("Insert");
            }
            Mode::Command => {
                return String::from("Command");
            }
            Mode::Normal => {
                return String::from("Normal");
            }
        }
    }
}

pub struct Editor {
    buffer: Vec<Vec<char>>,
    command_buffer: Vec<char>,
    pub mode: Mode,
    pub screen_begin_row: usize,
    pub cursor: Position,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            buffer: vec![vec![]],
            mode: Mode::Insert,
            screen_begin_row: 0,
            command_buffer: vec![],
            cursor: Position::new(0, 0),
        }
    }
    fn render_status_line(&self, width: usize) -> Vec<char> {
        let mut status_line = Vec::<char>::new();
        for x in (0..width) {
            status_line.push(' ');
        }
        let mut start = 0;
        let mode_string = self.mode.to_string();
        for (x, c) in (start..mode_string.len()).zip(mode_string.chars()) {
            status_line[x] = c;
        }

        status_line
    }

    pub fn render(&self, height: usize, width: usize) -> Vec<Vec<char>> {
        let mut ret = Vec::<Vec<char>>::new();
        for y in (self.screen_begin_row..self.screen_begin_row + height - 2) {
            let mut line = Vec::<char>::new();
            let mut buffline = Vec::<char>::new();
            if self.buffer.len() >= y + 1 {
                buffline = self.buffer[y].clone();
            } else {
                buffline = (0..width).map(|_| ' ').collect()
            }
            for x in (0..width) {
                if buffline.len() >= x + 1 {
                    line.push(buffline[x]);
                } else {
                    line.push(' ');
                }
            }
            ret.push(line);
        }
        ret.push(self.render_status_line(width));
        let mut line = Vec::<char>::new();
        match self.mode {
            Mode::Command => {
                for (_, c) in (0..width).zip(self.command_buffer.clone()) {
                    line.push(c);
                }
            }
            Mode::Insert => {
                for (_, c) in (0..width).zip("-- Insert --".chars()) {
                    line.push(c);
                }
            }
            Mode::Normal => {
                for (_, c) in (0..width).zip("".chars()) {
                    line.push(c);
                }
            }
        }
        ret.push(line);
        // insert status line
        ret
    }

    pub fn load_file(&mut self, path: &OsStr) {
        let mut buff = Vec::<Vec<char>>::new();
        let content = fs::read_to_string(path).unwrap();
        let lines = content.split('\n');
        for line in lines {
            let mut l = Vec::<char>::new();
            for c in line.chars() {
                l.push(c);
            }
            buff.push(l);
        }
        self.buffer = buff;
    }
}
