#![allow(non_upper_case_globals)]

use std::convert::From;
use std::ops::*;
use std::iter::Step;
use std::sync::Mutex;

pub const NUM_VOICES: usize = 4;

pub const HISTORY_LENGTH: usize = 150;
lazy_static! {
    pub static ref SAMPLE_HISTORY : Mutex<Vec<f32>> = {
        Mutex::new(vec![0.0; HISTORY_LENGTH])
    };
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PitchClass {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

pub const CHROMATIC_SCALE: [PitchClass; 12] = [C, Db, D, Eb, E, F, Gb, G, Ab, A, Bb, B];

pub use self::PitchClass::*;

impl From<u8> for PitchClass {
    fn from(u: u8) -> Self {
        CHROMATIC_SCALE[u as usize % 12]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Note {
    pub octave: u8,
    pub class: PitchClass,
}

pub fn note(class: PitchClass, octave: u8) -> Note {
    Note { octave, class }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Hz(pub f32);

impl From<Note> for Hz {
    fn from(n: Note) -> Self {
        // Equal temperament
        Hz(13.75 * (f32::from(n.octave) + f32::from(n.class as u8 + 3) / 12.0).exp2())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Waveform {
    Sine,
    Saw,
    Rect(f32),
}

impl Add<u8> for Note {
    type Output = Note;
    fn add(self, rhs: u8) -> Note {
        Note {
            class: PitchClass::from(self.class as u8 + rhs),
            octave: self.octave + (self.class as u8 + rhs) / 12,
        }
    }
}

impl Sub<u8> for Note {
    type Output = Note;
    fn sub(self, rhs: u8) -> Note {
        Note {
            class: PitchClass::from(self.class as u8 - rhs),
            octave: self.octave + (self.class as u8 - rhs) / 12,
        }
    }
}

impl AddAssign<u8> for Note {
    fn add_assign(&mut self, rhs: u8) {
        self.octave = self.octave + (self.class as u8 + rhs) / 12;
        self.class = PitchClass::from(self.class as u8 + rhs);
    }
}

impl Sub<Note> for Note {
    type Output = i16;
    fn sub(self, rhs: Note) -> i16 {
        12 *  (i16::from(self.octave) - i16::from(rhs.octave))
            + (i16::from(self.class as u8) - i16::from(self.class as u8))
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Note) -> Option<::std::cmp::Ordering> {
        Hz::from(*self).partial_cmp(&Hz::from(*other))
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Note) -> ::std::cmp::Ordering {
        // Just unwrap since Hz will never be NaN
        Hz::from(*self).partial_cmp(&Hz::from(*other)).unwrap()
    }
}

impl Step for Note {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if end > start {
            Some((*end - *start) as usize)
        } else {
            None
        }
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        if count < 256 {
            Some(start + count as u8)
        } else {
            None
        }
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        if count < 256 {
            Some(start + count as u8)
        } else {
            None
        }
    }
}
