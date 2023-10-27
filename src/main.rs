use log::LevelFilter;
use newsroom::app::{App, AppResult};
use newsroom::event::{Event, EventHandler};
use newsroom::handler::handle_key_events;
use newsroom::tui::Tui;

use std::io;

use tui::backend::CrosstermBackend;
use tui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Setup logging, only used for development

    if cfg!(debug_assertions) {
        simple_logging::log_to_file("newsroom.log", LevelFilter::Info);
    }

    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;
    // Start the main loop.
    while app.running {
        tui.draw(&app)?;

        // Handle events.
        match tui.events.next()? {
            Event::Tick => {
                app.tick();
            },
            Event::Key(key_event) => handle_key_events(key_event, &app)?,
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {},
            Event::Nothing => {},
        }

        app.poll_and_run_action().await;
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
