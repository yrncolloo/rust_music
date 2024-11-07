use std::io;

use crossterm::event::{self, Event};

#[derive(Debug, Default)]
pub struct App{
    pub exit: bool
}

impl App {
    pub fn new() -> Self {
        App { 
            exit: false
        }
        
    }
    
    pub fn handle_events(&mut self) -> io::Result<()>{
        if let Event::Key(key) = event::read()?{
            match key.code {
                event::KeyCode::Char('q') => self.exit(),

                _=> {}
            }
        }
        Ok(())
        
    }

    fn exit(&mut self) {
        self.exit = true;
        
    }
}
