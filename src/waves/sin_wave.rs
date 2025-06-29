use std::f32::consts::PI;
use crate::waves::Wave;
use crate::notes::Notes;


pub struct SinWave {
    frequncy: f32,
    amplitude: f32,
    duration: f32,
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