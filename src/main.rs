use newsroom::app::{App, AppResult};
use newsroom::event::{Event, EventHandler};
use newsroom::handler::handle_key_events;
use newsroom::tui::Tui;
use tokio::sync::Mutex;
use std::io;
use std::sync::Arc;
use tui::backend::CrosstermBackend;
use tui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let app = App::new();
    let app_arc = Arc::new(Mutex::new(app));

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    let mut app_should_run: bool = true;

    // Start the main loop.
    while app_should_run {
        // Clone the ARC MUTEX for use later
        let app_arc_local = app_arc.clone();
        {
            // Run somethings that need access to the App directly
            let app_locked = app_arc_local.lock().await;
            app_should_run = app_locked.running;
            // Render the user interface.
            tui.draw(app_locked)?;
        }

        // Handle events.
        match tui.events.next()? {
            Event::Tick => {
                tokio::spawn(async move {
                    let app_locked = app_arc_local.lock().await;
                    app_locked.tick()
                });
            },
            Event::Key(key_event) => handle_key_events(key_event, app_arc_local.clone())?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}