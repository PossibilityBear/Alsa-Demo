use std::time::Duration;

use crate::waves::{Wave, WaveFunc};
use crate::SAMPLE_RATE;
use alsa::pcm::{Access, Format, HwParams, PCM, State};
use alsa::{Direction, ValueOr};


pub struct Playback {
    pcm: PCM,
}

impl Playback {
    pub fn new() -> Playback {
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
            swp.set_start_threshold(hwp.get_buffer_size().unwrap())
                .unwrap();
            pcm.sw_params(&swp).unwrap();
        }
        Playback { pcm }
    }

    // fn _write_buf(&mut self, buf: &[i16]) {
    //     let io = self.pcm.io_i16().unwrap();
    //     assert_eq!(io.writei(&buf[..]).unwrap(), 1);
    // }

    pub fn write_wave<W: Wave>(&mut self, wave: &W, duration: f32) {
        let io = self.pcm.io_i16().unwrap();

        let num_samples = duration as u32 * SAMPLE_RATE;

        let frame_size = wave.get_period();    
        
        let num_frames = num_samples / frame_size as u32;

        let mut frame_buf = Vec::<i16>::with_capacity(frame_size as usize);
        
        let mut x = 0;
        while x < frame_size {
            let sample = wave.call_wave_func(x as f32) as i16;
            frame_buf.push(sample);
            x += 1;
        }

        let mut i = 0;
        while i < num_frames {
            for sample in &frame_buf {
                assert_eq!(io.writei(&[*sample]).unwrap(), 1);
            }
            i += 1;
        }
    }

    pub fn play(&mut self) {
        if self.pcm.state() != State::Running {
            self.pcm.start().unwrap()
        };
    }

    pub fn drain(&mut self) {
        self.pcm.drain().unwrap();
    }
}
