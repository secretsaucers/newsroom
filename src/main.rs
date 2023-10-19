#![allow(warnings, unused)]
mod app;
mod ui;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{info, LevelFilter};
use std::{error::Error, io, sync::Arc};
use tokio::sync::Mutex;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup logging, only used for development
    simple_logging::log_to_file("newsroom.log", LevelFilter::Info);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app: App = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend + std::marker::Send>(
    terminal: &mut Terminal<B>,
    mut app: app::App,
) -> io::Result<()> {
    // Main loop

    // Create an arc mutex to App so that we can access it nicely from other threads
    let app_arc = Arc::new(Mutex::new(app));

    // Handle user inputs, delegating each to an async
    loop {
        // Check if we can grab a lock on the app mutex, if we can use it to update the terminal output
        match app_arc.try_lock() {
            Ok(mut locked_app) => {
                terminal.draw(|f| ui::ui(f, &mut locked_app));
            }
            Err(_) => {}
        }

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                info!("{:#?}", key);
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => {
                        let app_arc_local = app_arc.clone();
                        tokio::spawn(async move {
                            let mut app_local = app_arc_local.lock().await;
                            app_local.next();
                        });
                    }
                    KeyCode::Up => {
                        let app_arc_local = app_arc.clone();
                        tokio::spawn(async move {
                            let mut app_local = app_arc_local.lock().await;
                            app_local.previous();
                        });
                    }
                    KeyCode::Char('l') => {
                        let app_arc_local = app_arc.clone();
                        let t1 = tokio::spawn(async move {
                            let mut app_local = app_arc_local.lock().await;
                            app_local.load().await;
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}
