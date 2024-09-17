use std::io;

use crossterm::event::MouseEventKind;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

pub mod app;
pub mod custom_table;
pub mod event;
pub mod handler;
pub mod table;
pub mod tui;
pub mod ui;

// fn execute_query(&mut self) -> DuckResult<()> {
//     let mut stmt = self.connection.prepare(&self.input)?;
//     self.results = stmt.query_arrow([])?.collect();
//     Ok(())
// }

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new()?;

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
                handle_key_events(key_event, &mut app);
            }
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollDown => {
                    app.scroll_vertical(1);
                }
                MouseEventKind::ScrollUp => {
                    app.scroll_vertical(-1);
                }
                MouseEventKind::ScrollRight => {
                    app.scroll_horizontal(1);
                }
                MouseEventKind::ScrollLeft => {
                    app.scroll_horizontal(-1);
                }
                _ => {}
            },
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
