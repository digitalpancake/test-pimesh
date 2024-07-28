use std::io;

use crate::tui;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame,
};

#[derive(Debug, Default)]
pub struct App {
    selected: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => self.select_prev(),
            KeyCode::Down => self.select_next(),
            KeyCode::Char('q') => self.exit(),
            _ => {},
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn select_next(&mut self) {
        self.selected += 1
    }
    fn select_prev(&mut self) {
        self.selected -= 1
    }


}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("PiMesh CLI".bold());
        let instructions = Title::from(Line::from(vec![
            " Next".into(),
            "<Down>".green().bold(),
            " Previous".into(),
            "<Up>".green().bold(),
            " Quit".into(),
            "<Q>".green().bold(),
        ]));

        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::ROUNDED);

        let selected_text = Text::from(vec![Line::from(vec![
                                                       "Value:".into(),
                                                       self.selected.to_string().blue(),
        ])]);

        Paragraph::new(selected_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
