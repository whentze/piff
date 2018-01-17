use definitions::*;

use termion::color::{Bg, Fg, Rgb};
use termion::{cursor, style};
use std::fmt::{self, Write};

#[derive(Debug, Clone, Copy)]
struct Key {
    note: Note,
    highlight: u8,
}

pub struct Keyboard {
    keys: Vec<Key>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            keys: (note(C, 2)..note(Ab, 6))
                .map(|n| Key {
                    note: n,
                    highlight: 0,
                })
                .collect(),
        }
    }
    pub fn highlight(&mut self, n: Note) {
        if let Some(key) = self.keys.iter_mut().find(|e| e.note == n) {
            key.highlight = 255;
        }
    }
    pub fn update(&mut self) {
        for key in &mut self.keys {
            key.highlight = key.highlight.saturating_sub(12);
        }
    }
    pub fn paint(&self) -> String {
        let mut res = String::with_capacity(100);
        for keys in self.keys.windows(2) {
            let hl0 = 255 - keys[0].highlight;
            let hl1 = keys[1].highlight;
            let fg = Fg(Rgb(hl0, hl0, 255));
            let bg = Bg(Rgb(0, 0, hl1));
            let bg2 = Bg(Rgb(255 - hl1, 255 - hl1, 255));
            match keys[0].note.class {
                C | D => {
                    write!(res, "{}{}▙", fg, bg).unwrap();
                }
                E => {
                    write!(res, "{}{}▌", fg, bg2).unwrap();
                }
                F => {
                    write!(res, "{}", bg).unwrap();
                }
                G | A | B => {
                    write!(res, "{}▟{}", fg, bg).unwrap();
                }
                _ => (),
            };
        }
        let hl = 255 - self.keys.last().unwrap().highlight;
        let fg = Fg(Rgb(hl, hl, 255));
        write!(res, "{}▟", fg).unwrap();
        res
    }
}

impl fmt::Display for Keyboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for key in &self.keys {
            write!(
                f,
                "{}",
                match key.note.class {
                    C | D => "▙",
                    F => "█",
                    G | A | B => "▟",
                    _ => "",
                }
            )?;
        }
        Ok(())
    }
}


pub struct Scope {
    samples: Vec<f32>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            samples: vec![0.0; HISTORY_LENGTH],
        }
    }
    pub fn update(&mut self) {
        for i in (0..HISTORY_LENGTH).rev() {
            let hist = (*SAMPLE_HISTORY).lock().unwrap();
            self.samples[i] = hist[i];
        }
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", style::Reset)?;
        for y in 0..24 {
            write!(f, "█{}", Fg(Rgb(70, 255, 30)))?;
            for chunk in self.samples.chunks(2) {
                let li = chunk[0] * 20.0 + 11.5;
                let left = if li.floor() as i32 == y {
                    4 - ((li - li.floor()) * 4.0) as i32
                } else {
                    0
                };
                let ri = chunk[1] * 20.0 + 11.5;
                let right = if ri.floor() as i32 == y {
                    4 - ((ri - ri.floor()) * 4.0) as i32
                } else {
                    0
                };
                write!(f, "{}", match (left, right) {
                    (0, 0) => " ", (0, 1) => "⢀", (0, 2) => "⠠", (0, 3) => "⠐", (0, 4) => "⠈",
                    (1, 0) => "⡀", (1, 1) => "⣀", (1, 2) => "⡠", (1, 3) => "⡐", (1, 4) => "⡈",
                    (2, 0) => "⠄", (2, 1) => "⢄", (2, 2) => "⠤", (2, 3) => "⠔", (2, 4) => "⠌",
                    (3, 0) => "⠂", (3, 1) => "⢂", (3, 2) => "⠢", (3, 3) => "⠒", (3, 4) => "⠊",
                    (4, 0) => "⠁", (4, 1) => "⢁", (4, 2) => "⠡", (4, 3) => "⠑", (4, 4) => "⠉",
                    _ => "!",
                })?;
            }
            write!(f, "{}█{}{}", style::Reset, cursor::Down(1), cursor::Left(HISTORY_LENGTH as u16))?;
        }
        Ok(())
    }
}