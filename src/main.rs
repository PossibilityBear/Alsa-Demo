use alsa::{Direction, ValueOr};
use alsa::pcm::{PCM, HwParams, Format, Access, State};
use std::f32::consts::PI;
use textplots::{self, AxisBuilder, Chart, Plot, Shape, LineStyle};



const SAMPLE_RATE: u32 = 44_100;

struct Playback {
    pcm: PCM,
}

impl Playback {
    fn new()-> Playback {
        let pcm = PCM::new("default", Direction::Playback, false).unwrap();
        // Open default playback device
        {
            // Set hardware parameters: 44,100 Hz / Mono / 16 bit
            let hwp = HwParams::any(&pcm).unwrap();
            hwp.set_channels(1).unwrap();
            hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).unwrap();
            hwp.set_format(Format::s16()).unwrap();
            hwp.set_access(Access::RWInterleaved).unwrap();
            pcm.hw_params(&hwp).unwrap();
            
            // Make sure we don't start the stream too early
            let hwp = pcm.hw_params_current().unwrap();
            let swp = pcm.sw_params_current().unwrap();
            swp.set_start_threshold(hwp.get_buffer_size().unwrap()).unwrap();
            pcm.sw_params(&swp).unwrap();
        }
        Playback {pcm}
    }
    
    // fn _write_buf(&mut self, buf: &[i16]) {
    //     let io = self.pcm.io_i16().unwrap();
    //     assert_eq!(io.writei(&buf[..]).unwrap(), 1);
    // }

    fn write_wave<T: Wave>(&mut self, wave: &T) {
        let io = self.pcm.io_i16().unwrap();

        let num_samples = wave.duration() as u32 * SAMPLE_RATE;

        // let mut buf = Vec::<i16>::with_capacity(buf_size as usize);
        let mut i = 0;
        while i < num_samples {
            let sample = wave.wave_func(i as f32, SAMPLE_RATE) as i16;
            // could optimize to write in chunks but probably don't need to yet
            assert_eq!(io.writei(&[sample]).unwrap(), 1);
            i += 1;
        }
    }

    fn play(&mut self) {
        // In case the buffer was larger than 2 seconds, start the stream manually.
        if self.pcm.state() != State::Running { self.pcm.start().unwrap() };
        // Wait for the stream to finish playback.
        self.pcm.drain().unwrap();
    }
}

pub trait Wave {
    fn wave_func(&self, x: f32, sample_rate: u32) -> f32;
    
    // fn wave_buffer(self: &Self) -> Vec<i16>;

     fn duration(self: &Self) -> f32;
}

impl Wave for SinWave {
    fn wave_func(&self, x: f32, sample_rate: u32) -> f32 {
        self.amplitude * 
            (
                (
                    (2.0 * PI * self.frequncy) / sample_rate as f32
                )  * x
            ).sin()
    }

    fn duration(self: &Self) -> f32 {
        self.duration
    }
}


pub struct SinWave {
    frequncy: f32,
    amplitude: f32,
    duration: f32,
}

impl Default for SinWave {
    fn default() -> Self {
        SinWave {
            // 240 Min freq with my shiddy speakers
            // 30 min freq with headset
            frequncy: Notes::A.freq(), 
            amplitude: 8_000.0, // with i_16 max is ((2^16) / 2) -1 = 32,767
            duration: 2.0,
        }
    }
}

enum Notes {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl Notes {
    fn freq(&self) -> f32 {
        match self {
            Self::C => 261.63,
            Self::Cs => 277.18,
            Self::D => 293.66,
            Self::Ds => 311.13,
            Self::E => 329.63,
            Self::F => 349.23,
            Self::Fs => 369.99,
            Self::G => 392.00,
            Self::Gs => 415.30,
            Self::A => 440.00,
            Self::As =>466.16,
            Self::B => 493.88,
        }
    }
}


fn main() {
    let mut playback = Playback::new();

    let my_wave: SinWave = SinWave::default();

    playback.write_wave(&my_wave);
    
    // Plot it to terminal
    Chart::new(200, 10, 0.0, 0.5)
        .x_axis_style(LineStyle::None)
        .y_axis_style(LineStyle::None)
        .lineplot(&Shape::Continuous(
            Box::new(|x| {my_wave.wave_func(x, 1)})))
        .display();


    playback.play();
}
