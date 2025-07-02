
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

use crate::{wave_widget::WaveWidget, WAVE_CHAR_HEIGHT};

#[derive(Default)]
pub struct App {
    counter: u8,
    exit: bool,
    waves: WaveWidget,

}



impl App {
    // runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        //draw base app widget taking up full frame
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(WAVE_CHAR_HEIGHT as u16),
                Constraint::Length(3),
            ])
            .split(frame.area());

        frame.render_widget(self, chunks[0]); 

        frame.render_widget(&self.waves, chunks[1]);
    }


    // updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    
    //Handle events related to keystrokes
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left=> self.decrement_counter(),
            KeyCode::Right => {
                let this = &mut *self;
                this.waves.next_wave();
            },
            _ => {}
        }
    }


    fn exit(&mut self) {
        self.exit = true;
    }

    

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" VVaves ".bold());
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::Set{    
                top_left: line::THICK.horizontal,
                top_right: line::THICK.horizontal,    
                bottom_left: "",
                bottom_right: "",
                vertical_left: "",
                vertical_right: "",
                horizontal_top: line::THICK.horizontal,
                horizontal_bottom: "",
            });

        block.render(area, buf);
    }
}
