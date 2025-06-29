pub trait Wave {
    fn wave_func(&self, x: f32, sample_rate: u32) -> f32;
    
    // fn wave_buffer(self: &Self) -> Vec<i16>;

    fn duration(self: &Self) -> f32;
}

pub mod sin_wave;