use std::fmt::Debug;
pub mod sin_wave;

pub trait Wave {
    fn wave_func(&self, x: f32, sample_rate: u32) -> f32;

    // fn wave_buffer(self: &Self) -> Vec<i16>;

    fn duration(self: &Self) -> f32;
}

#[derive(Clone, Copy, Debug)]
pub struct GenWaveSettings {
    pub freq: f32,
    pub dur: f32,
    pub amp: f32,
    pub sample_rate: u32,
}

impl GenWaveSettings {
    fn new(freq: f32, dur: f32, amp: f32, sample_rate: u32) -> Self {
        Self {
            freq,
            dur,
            amp,
            sample_rate,
        }
    }
}

pub struct GenWave {
    pub settings: GenWaveSettings,
    wave_func: Box<dyn Fn(&GenWaveSettings, f32) -> i16>,
}

impl Debug for GenWave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("settings: {:?}", self.settings);
        Ok(())
    }
}

impl GenWave {
    pub fn new(
        wave_func: impl Fn(&GenWaveSettings, f32) -> i16 + 'static,
        settings: GenWaveSettings,
    ) -> Self {
        Self {
            settings,
            wave_func: Box::new(wave_func),
        }
    }

    pub fn call_wave_func(&self, x: f32) -> i16 {
        (self.wave_func)(&self.settings, x)
    }
}
