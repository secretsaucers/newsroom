use log::LevelFilter;
use newsroom::app::{App, AppResult};
use newsroom::event::{Event, EventHandler};
use newsroom::handler::handle_key_events;
use newsroom::newsroomcore::newsroomstate::NewsroomTransitions;
use newsroom::tui::Tui;

use std::io;

use tui::backend::CrosstermBackend;
use tui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Setup logging, only used for development
    if cfg!(debug_assertions) {
        let _ = simple_logging::log_to_file("newsroom.log", LevelFilter::Info);
    }

    // Create an application.
    let mut app = App::new();
    let _ = app.tx.send(NewsroomTransitions::FetchMedia(app.settings.sources.clone()));

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250); // Tick event every 250ms, this is the minimum update loop speed
    let mut tui = Tui::new(terminal, events);
    tui.init()?;
    // Start the main loop.
    while app.running {
        tui.draw(&app)?;

        // Handle events
        // TODO: The match below is blocking, so it should be corrected in the future
        // but it will block for at most a tick so it should be imperceptible. 
        // Tokio requires us not to block the main thread so that it can still poll on futures.
        match tui.events.next().await? {
            Event::Tick => {
                app.tick();
            },
            Event::Key(key_event) => handle_key_events(key_event, &app)?,
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {},
        }

        app.poll_and_run_action().await;
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
