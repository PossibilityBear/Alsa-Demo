use crate::notes::Notes;
use crate::waves::{WaveSettings, Wave, WaveFunc};
use std::f32::consts::PI;



pub struct SinWave {
    func: WaveFunc
}

impl SinWave {
    pub fn new(settings: WaveSettings) -> Self {
        SinWave {
            func: WaveFunc::new(Self::sin_wave, Self::sin_wave_period, settings),
        }
    }

    fn sin_wave(settings: &WaveSettings, x: f32) -> i16 {
        (settings.amp * (((2.0 * PI * settings.freq) / settings.sample_rate as f32) * x).sin()) as i16
    }

    //get the period length for frame sizing
    fn sin_wave_period(settings: &WaveSettings) -> usize {
        println!("Sample rate {} / Freq {}", settings.sample_rate, settings.freq);
        (settings.sample_rate as f32  / settings.freq ) as usize
    }
}

impl Wave for SinWave {
    fn call_wave_func(&self, x: f32) -> i16 {
        self.func.call_wave_func(x)
    }

    fn get_period(&self) -> usize {
        self.func.get_period()
    }

    fn set_sample_rate(&mut self, sample_rate: u32) {
        self.func.set_sample_rate(sample_rate);
    }
}