use alsa::{Direction, ValueOr};
use alsa::pcm::{PCM, HwParams, Format, Access, State};
use std::f32::consts::PI;
use textplots::{self, Chart, Plot, Shape};

const BUFSIZE: usize = (2.0 * PI) as usize;

// should be a factor of BUFSIZE otherwise you get distortion
// due to the period of the wave function not lining cleanly up with the BUFSIZE
const COMPRESSION: u32 = 128; 

const SAMPLE_RATE: u32 = 44_100;

struct Playback {
    pcm: PCM,
}

impl Playback {
    fn setup_pcm()-> Playback {
        let pcm = PCM::new("default", Direction::Playback, false).unwrap();
        // Open default playback device
        {
            // Set hardware parameters: 44100 Hz / Mono / 16 bit
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
    
    fn _write_buf(&mut self, buf: &[i16]) {
        let io = self.pcm.io_i16().unwrap();
        assert_eq!(io.writei(&buf[..]).unwrap(), BUFSIZE);
    }

    fn write_wave<T: Wave>(&mut self, wave: &T) {
        let io = self.pcm.io_i16().unwrap();
        assert_eq!(io.writei(&wave.wave_buffer()).unwrap(), BUFSIZE);
    }

    fn play(&mut self) {
        // In case the buffer was larger than 2 seconds, start the stream manually.
        if self.pcm.state() != State::Running { self.pcm.start().unwrap() };
        // Wait for the stream to finish playback.
        self.pcm.drain().unwrap();
    }
}

pub trait Wave {
    fn wave_func(&self, x: f32) -> f32;
    
    fn wave_buffer<'a>(self: &'a Self) -> Box<[i16]>;
}

impl Wave for SinWave {
    fn wave_func(&self, x: f32) -> f32 {
        self.amplitude * (self.period / (2.0 * PI) * x).sin()
        // ((x * self.freq * PI ) / self.funk).sin() * self.amp
    }

    fn wave_buffer(&self) -> Box<[i16]> {
        let mut buf = [0i16; BUFSIZE];
        for (i, a) in buf.iter_mut().enumerate() {
            *a = self.wave_func(i as f32) as i16;
        }
        Box::new(buf)
    }
}


pub struct SinWave {
    period: f32,
    amplitude: f32,
    funk: f32,
}

impl Default for SinWave {
    fn default() -> Self {
        SinWave {
            funk: COMPRESSION as f32 ,
            // 1.5 Min freq before period misaligns wiht BUFSIZE causing a more sawtooth like sound
            period: 14.0, 
            amplitude: 10_000.0
        }
    }
}



fn main() {
    let mut playback = Playback::setup_pcm();

    // Play it back for 2 seconds.
    for _ in 0..2*SAMPLE_RATE/(BUFSIZE as u32) {
        playback.write_wave(&SinWave::default());
    }
    
    // Plot it to terminal
    Chart::new(200, 20, 0.0, COMPRESSION as f32)
        .lineplot(&Shape::Continuous(
            Box::new(|x| {SinWave::default().wave_func(x)})))
        .display();


    playback.play();
}
