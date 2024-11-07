pub mod app;
mod ui;

use std::io;

use app::App;
use ratatui::{prelude::Backend, Terminal};
use ui::ui;

pub fn launch_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>{
    loop {
        terminal.draw(|frame| ui(frame, app))?;
        app.handle_events()?;
        if app.exit {
            break;
        }
    }

    Ok(false)

}
