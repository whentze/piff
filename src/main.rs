#![feature(step_trait, integer_atomics)]

extern crate cpal;
extern crate termion;
#[macro_use]
extern crate lazy_static;

mod definitions;
mod widgets;
mod input;
mod synth;

use cpal::{EventLoop, SampleFormat, UnknownTypeBuffer};

use termion::{async_stdin, clear, cursor};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::TermRead;

use std::cmp::min;
use std::io::{self, stdin, stdout, Read, Write};
use std::thread;
use std::time::{SystemTime, Duration};
use std::sync::mpsc::channel;

use input::key_to_command;
use input::Command::*;
use widgets::{Keyboard, Scope};
use synth::Synth;


fn main() {
    // hilarious amounts of cpal boilerplate
    let event_loop = EventLoop::new();
    let endpoint = cpal::default_endpoint().unwrap();
    let mut supported_formats_range = endpoint.supported_formats().unwrap();
    let format = supported_formats_range
        .find(|format| format.data_type == SampleFormat::F32)
        .unwrap()
        .with_max_samples_rate();
    let samples_rate = format.samples_rate.0 as f32;
    let voice_id = event_loop.build_voice(&endpoint, &format).unwrap();
    event_loop.play(voice_id);
    let (tx, rx) = channel();

    let mut synth = Synth::new(rx, samples_rate);

    thread::spawn(move || {
        event_loop.run(move |_voice_id, buffer| match buffer {
            UnknownTypeBuffer::F32(mut buffer) => {
                for sample in buffer.chunks_mut(format.channels.len()) {
                    let value = synth.next_sample();
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            _ => {
                unreachable!();
            }
        })
    });

    let ((orig_x, orig_y), mut stdout) = stdout_setup();
    let mut stdin = async_stdin().keys();

    let mut scope = Scope::new();
    let mut keyboard = Keyboard::new();
    loop {
        while let Some(Ok(key)) = stdin.next() {
            let command = key_to_command(key);
            match command {
                PlayNote(n) => {
                    tx.send(command).unwrap();
                    keyboard.highlight(n);
                }
                SetWaveform(_) | ChangeDecay(_) => {
                    tx.send(command).unwrap();
                }
                Quit => {
                    write!(stdout, "{}{}{}", cursor::Show, cursor::Goto(orig_x, orig_y), clear::AfterCursor).unwrap();
                    std::io::stdout().flush().unwrap();
                    std::process::exit(0)
                },
                Nothing => {}
            };
        }
        write!(stdout, "{}{}", cursor::Goto(orig_x, orig_y), scope).unwrap();
        write!(stdout, "{}{}", cursor::Goto(orig_x + 20, orig_y + 24), keyboard.paint()).unwrap();
        scope.update();
        keyboard.update();
        thread::sleep(Duration::from_millis(40));
    }
}

fn stdout_setup() -> ((u16, u16), RawTerminal<io::Stdout>) {
    let termsize = termion::terminal_size().unwrap();
    let app_size = (80, 24);
    let mut stdout = stdout().into_raw_mode().unwrap();
    let origin = cursor_pos(&mut stdout).unwrap();
    let orig_x = origin.0;
    let orig_y = min(origin.1, termsize.1 - app_size.1);
    for _ in 0..app_size.1 { println!() };
    write!(stdout, "{}", cursor::Hide).unwrap();
    ((orig_x, orig_y), stdout)
}

fn cursor_pos(stdout: &mut RawTerminal<io::Stdout>) -> io::Result<(u16, u16)> {
    let mut stdin = stdin();

    write!(stdout, "\x1B[6n")?;
    stdout.flush()?;

    let mut buf: [u8; 1] = [0];
    let mut read_chars = Vec::new();

    let timeout = Duration::from_millis(100);
    let now = SystemTime::now();

    while buf[0] != b'R' && now.elapsed().unwrap() < timeout {
        if stdin.read(&mut buf)? > 0 {
            read_chars.push(buf[0]);
        }
    }
    if read_chars.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "Cursor position detection timed out."));
    }

    read_chars.pop();
    let read_str = String::from_utf8(read_chars).unwrap();
    let beg = read_str.rfind('[').unwrap();
    let coords: String = read_str.chars().skip(beg + 1).collect();
    let mut nums = coords.split(';');

    let cy = nums.next()
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let cx = nums.next()
        .unwrap()
        .parse::<u16>()
        .unwrap();

    Ok((cx, cy))
}