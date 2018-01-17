use definitions::*;
use termion::event::Key;

pub enum Command {
    PlayNote(Note),
    SetWaveform(Waveform),
    ChangeDecay(f32),
    Quit,
    Nothing,
}

pub fn key_to_command(key: Key) -> Command {
    use self::Command::*;
    use termion::event::Key::{Alt, Char, Ctrl, Down, Up};
    match key {
        Char('a') => PlayNote(note(C, 2)),
        Char('w') => PlayNote(note(Db, 2)),
        Char('s') => PlayNote(note(D, 2)),
        Char('e') => PlayNote(note(Eb, 2)),
        Char('d') => PlayNote(note(E, 2)),
        Char('f') => PlayNote(note(F, 2)),
        Char('t') => PlayNote(note(Gb, 2)),
        Char('g') => PlayNote(note(G, 2)),
        Char('z') => PlayNote(note(Ab, 2)),
        Char('h') => PlayNote(note(A, 2)),
        Char('u') => PlayNote(note(Bb, 2)),
        Char('j') => PlayNote(note(B, 2)),
        Char('k') => PlayNote(note(C, 3)),
        Char('o') => PlayNote(note(Db, 3)),
        Char('l') => PlayNote(note(D, 3)),
        Char('p') => PlayNote(note(Eb, 3)),
        Char('ö') => PlayNote(note(E, 3)),
        Char('ä') => PlayNote(note(F, 3)),
        Char('+') => PlayNote(note(Gb, 3)),
        Char('#') => PlayNote(note(G, 3)),

        Char(c) if c.is_uppercase() => {
            if let PlayNote(n) = key_to_command(Key::Char(c.to_lowercase().next().unwrap())) {
                PlayNote(Note {
                    octave: n.octave + 1,
                    ..n
                })
            } else {
                Nothing
            }
        }
        Char('*') => PlayNote(note(Gb, 4)),
        Char('\'') => PlayNote(note(G, 4)),

        Alt(c) => {
            if let PlayNote(n) = key_to_command(Key::Char(c)) {
                PlayNote(Note {
                    octave: n.octave + 2,
                    ..n
                })
            } else {
                Nothing
            }
        }

        Char('1') => SetWaveform(Waveform::Sine),
        Char('2') => SetWaveform(Waveform::Rect(0.5)),
        Char('3') => SetWaveform(Waveform::Saw),
        Char('4') => SetWaveform(Waveform::Rect(0.25)),
        Char('5') => SetWaveform(Waveform::Rect(0.1)),

        Up => ChangeDecay(0.8),
        Down => ChangeDecay(1.25),

        Char('q') | Ctrl('c') => Quit,
        _ => Nothing,
    }
}
