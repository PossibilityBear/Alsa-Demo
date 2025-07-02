use std::f32::consts::PI;
use crate::waves::{GenWaveSettings, Wave};
use crate::notes::Notes;

#[derive(Debug)]
pub struct SinWave {
    pub frequency: f32,
    pub amplitude: f32,
    pub duration: f32,
}

impl Wave for SinWave {
    fn wave_func(&self, x: f32, sample_rate: u32) -> f32 {
        self.amplitude * 
            (
                (
                    (2.0 * PI * self.frequency) / sample_rate as f32
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
            frequency: Notes::A.freq(), 
            amplitude: 8_000.0, // with i_16 max is ((2^16) / 2) -1 = 32,767
            duration: 2.0,
        }
    }
}




pub fn gen_sin_wave(settings: &GenWaveSettings, x: f32) -> i16 {
    (settings.amp * 
        (
            (
                (2.0 * PI * settings.freq) / settings.sample_rate as f32
            )  * x
        ).sin()) as i16
}