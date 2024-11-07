use std::io;

use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{prelude::CrosstermBackend, Terminal};
use rust_music::{app, launch_app, };
fn main() -> io::Result<()>{
    // Setup the terminal

    enable_raw_mode()?;
    let mut std_error = io::stderr();
    execute!(std_error, EnterAlternateScreen, EnableMouseCapture)?;

    // setup backend 
    let backend = CrosstermBackend::new(std_error);
    let mut terminal = Terminal::new(backend)?;

    // create app
    let mut app = app::App::new();
    launch_app(&mut terminal, &mut app)?;


    // restore the termial
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
        )?;


    Ok(())
}
