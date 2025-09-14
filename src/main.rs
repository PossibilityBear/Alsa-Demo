mod playback;
mod ui_controllers;
mod waves;
mod notes;

use crate::waves::sin_wave::{SinWave};
use crate::waves::*;
use crate::playback::Playback;
use crate::ui_controllers::app::App;
use crate::ui_controllers::wave_widget::WaveWidget;

use ratatui;
use alsa::pcm::{Access, Format, HwParams, PCM, State};
use alsa::{Direction, ValueOr};


const SAMPLE_RATE: u32 = 44_100;
const WAVE_CHAR_HEIGHT: u32 = 7;

fn main() {
    let wave = SinWave::new(
        WaveSettings::new(440.0, 8000.0)
    );




    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result;
}