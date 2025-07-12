use std::fmt::Debug;

use crate::SAMPLE_RATE;
pub mod sin_wave;

pub trait Wave {
    fn call_wave_func(&self, x: f32) -> i16;

    fn get_period(&self) -> usize;

    fn set_sample_rate(&mut self, sample_rate: u32);
}


#[derive(Clone, Copy, Debug)]
pub struct WaveSettings {
    pub freq: f32,
    pub amp: f32,
    pub sample_rate: u32,
}

impl WaveSettings {
    pub fn new(freq: f32, amp: f32) -> Self {
        Self {
            freq,
            amp,
            sample_rate: SAMPLE_RATE,
        }
    }
}

pub struct WaveFunc {
    pub settings: WaveSettings,
    wave_func: Box<dyn Fn(&WaveSettings, f32) -> i16>,
    wave_period: Box<dyn Fn(&WaveSettings) -> usize>,
}

impl Debug for WaveFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("settings: {:?}", self.settings);
        Ok(())
    }
}

impl WaveFunc {
    pub fn new(
        wave_func: impl Fn(&WaveSettings, f32) -> i16 + 'static,
        wave_period_func: impl Fn(&WaveSettings) -> usize + 'static,
        settings: WaveSettings,
    ) -> Self {
        Self {
            settings,
            wave_func: Box::new(wave_func),
            wave_period: Box::new(wave_period_func),
        }
    }
}

impl Wave for WaveFunc {
    fn call_wave_func(&self, x: f32) -> i16 {
        (self.wave_func)(&self.settings, x)
    }

    fn get_period(&self) -> usize {
        let period = (self.wave_period)(&self.settings);
        println!("period {}", period);
        period
    }

    fn set_sample_rate(&mut self, sample_rate: u32) {
        self.settings.sample_rate = sample_rate;
    }
}
