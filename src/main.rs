mod waves;
mod notes;
mod playback;

use playback::Playback;
use crate::waves::{sin_wave::SinWave, Wave};


use textplots::{self, AxisBuilder, Chart, Plot, Shape, LineStyle};

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
