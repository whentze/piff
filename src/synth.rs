use std::sync::mpsc::Receiver;
use std::f32::consts::PI;

use definitions::*;
use input::Command;

#[derive(Debug)]
pub struct Synth {
    rx: Receiver<Command>,
    waveform: Waveform,
    decay: f32,
    sample_rate: f32,
    voice_index: usize,
    sum_count: usize,
    sum_value: f32,
    internal_history: Vec<f32>,
    voices: [Voice; NUM_VOICES],
}

impl Synth {
    pub fn new(rx: Receiver<Command>, sample_rate: f32) -> Self {
        Synth {
            rx,
            sample_rate,
            waveform: Waveform::Sine,
            decay: 0.99993,
            voice_index: 0,
            sum_count: 0,
            sum_value: 0.0,
            internal_history: vec![0.0; HISTORY_LENGTH],
            voices: <[Voice; NUM_VOICES]>::default(),
        }
    }
    pub fn next_sample(&mut self) -> f32 {
        match self.rx.try_recv() {
            Ok(Command::PlayNote(note)) => {
                let v = &mut self.voices[self.voice_index];
                v.freq = Hz::from(note).0;
                v.phase = 0.0;
                v.amp = 1.0 / NUM_VOICES as f32;
                self.voice_index = (self.voice_index + 1) % NUM_VOICES;
            }
            Ok(Command::SetWaveform(waveform)) => {
                self.waveform = waveform;
            }
            Ok(Command::ChangeDecay(change)) => {
                self.decay = self.decay.powf(change);
                if self.decay < 0.99 {
                    self.decay = 0.99;
                }
            }
            _ => {}
        };
        let mut res = 0.0;
        for v in &mut self.voices {
            v.phase = (v.phase + v.freq / self.sample_rate) % 1.0;
            res += v.amp * match self.waveform {
                Waveform::Sine => (v.phase * 2.0 * PI).sin(),
                Waveform::Saw => 2.0 * v.phase - 1.0,
                Waveform::Rect(duty_cycle) => {
                    if v.phase < duty_cycle {
                        1.0
                    } else {
                        -1.0
                    }
                }
            };
            v.amp *= self.decay;
        }
        self.sum_value += res;
        self.sum_count += 1;
        if self.sum_count == 10 {
            if self.internal_history.len() < HISTORY_LENGTH {
                self.internal_history.push(self.sum_value/10.0);
            } else {
                *(*SAMPLE_HISTORY).lock().unwrap() = self.internal_history.clone();
                self.internal_history.clear();
            }
            self.sum_value = 0.0;
            self.sum_count = 0;
        }
        res
    }
}

#[derive(Clone, Debug, Default)]
struct Voice {
    phase: f32,
    freq: f32,
    amp: f32,
}
