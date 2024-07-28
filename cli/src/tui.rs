use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::Paragraph,
    Terminal,
};
use std::io::{stdout, Result, Stdout};

use crate::app::App;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn run_tui() -> Result<()> {

    let mut terminal = tui_init()?;

    let res = App::default().run(&mut terminal);

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    res
}

fn tui_init() -> Result<Tui> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    Ok(terminal)
}
