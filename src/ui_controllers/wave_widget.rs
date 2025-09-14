use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{self, Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::{border, line},
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::{
    notes::Notes,
    waves::{Wave, WaveSettings, sin_wave::SinWave},
};
use textplots::{self, AxisBuilder, Chart, LabelBuilder, LineStyle, Plot, Shape};

pub struct WaveWidget {
    pub waves: Vec<Box<dyn Wave>>,
    pub index: usize,
}

impl Default for WaveWidget {
    fn default() -> Self {
        let notes = Notes::scale();
        let mut waves = Vec::<Box<dyn Wave>>::with_capacity(notes.capacity());
        for note in notes {
            let wave = SinWave::new(
                WaveSettings::new(note.freq(), 8000.0)
            );
            waves.push(Box::new(wave));
        }

        Self { waves, index: 0 }
    }
}

impl WaveWidget{
    pub fn next_wave(&mut self) {
        self.index += 1;
    }

    pub fn wave(&self) -> &Box<dyn Wave>{
        &self.waves[self.index]
    }
}

impl Widget for &WaveWidget {
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
        let shape = &Shape::Continuous(Box::new(|x| wave.call_wave_func(x) as f32));

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
