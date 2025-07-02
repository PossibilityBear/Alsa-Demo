
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::{border, line},
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::{notes::Notes, waves::{sin_wave::gen_sin_wave, GenWave, GenWaveSettings}};
use textplots::{self, AxisBuilder, Chart, LabelBuilder, LineStyle, Plot, Shape};

#[derive(Debug)]
pub struct WaveWidget {
    pub waves: Vec<GenWave>,
    pub index: usize,
}

impl Default for WaveWidget {
    fn default() -> Self {
        let waves = Notes::scale().into_iter().map(|note| {
            GenWave::new(
                    gen_sin_wave,
                    GenWaveSettings {
                        freq: note.freq(),
                        dur: 2.0,
                        amp: 8000.0,
                        sample_rate: 41000,
                    },
                )
        }).collect();


        Self {waves, index: 0}
    }
}

impl WaveWidget {
    pub fn next_wave(&mut self) {
        self.index += 1;
    }

    pub fn wave(&self) -> &GenWave {
        &self.waves[self.index]
    }
}

impl Widget for &WaveWidget{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(format!(" Wave: {}", self.index).bold());
        let instructions = Line::from(vec![
            " Next ".into(),
            "<Left>".blue().bold(),
            " Prev ".into(),
            "<Right>".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);


        // Plot it to terminal

        let wave = self.wave();

        const WAVE_CHAR_HEIGHT: u32 = 10;
        let mut chart = Chart::new(1000, WAVE_CHAR_HEIGHT, 0.0, 5000.0);
        let shape = &Shape::Continuous(
                Box::new(|x| {wave.call_wave_func(x) as f32}));

        
        let chart = chart
            .x_axis_style(LineStyle::None)
            .y_axis_style(LineStyle::None)
            .x_label_format(textplots::LabelFormat::None)
            .y_label_format(textplots::LabelFormat::None)
            .lineplot(shape);
         
        chart.axis();
        chart.figures();


        Paragraph::new(format!("{}", chart))
            .centered()
            .block(block)
            .render(area, buf);
    }
}