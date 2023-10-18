#![allow(warnings, unused)]
mod ui;
mod app;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::sync::Mutex;
use std::{error::Error, io, sync::Arc};
use tui::{
    backend::{Backend, CrosstermBackend}, Terminal,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: app::App) -> io::Result<()> {
    // Main loop
    
    // Create an arc mutex to App so that we can access it nicely from other threads
    let app_arc = Arc::new(Mutex::new(app));

    // Handle user inputs, delegating each to an async
    loop {
        // let app_arc_local = app_arc.clone();
        // tokio::spawn(async move {
        //     let mut app_local = app_arc_local.lock().await;
        //     terminal.draw(|f| ui::ui(f, &mut app_local))?;
        // });

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => {
                    let app_arc_local = app_arc.clone();
                    tokio::spawn(async move {
                        let mut app_local = app_arc_local.lock().await;
                        app_local.next();
                    });
                },
                KeyCode::Up => {
                    let app_arc_local = app_arc.clone();
                    tokio::spawn(async move {
                        let mut app_local = app_arc_local.lock().await;
                        app_local.previous();
                    });
                },
                KeyCode::Char('l') => {
                    let app_arc_local = app_arc.clone();
                    tokio::spawn(async move {
                        let mut app_local = app_arc_local.lock().await;
                        app_local.load();
                    });
                },
                _ => {},
            }
        }
    }
}

