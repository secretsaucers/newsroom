

use crate::app::{App, AppResult, newsroomcore::newsroomstate::NewsroomTransitions};
use crossterm::event::{KeyCode, KeyEvent};
use log::info;


/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &App) -> AppResult<()> {
    info!("user input: {:#?}", key_event);
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            let _ = app.tx.send(NewsroomTransitions::Quit);
        },
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            let _ = app.tx.send(NewsroomTransitions::Quit);
        },
        KeyCode::Down | KeyCode::Char('j') => {
            let _ = app.tx.send(NewsroomTransitions::Down);
        }
        KeyCode::Up | KeyCode::Char('k') => {
            let _ = app.tx.send(NewsroomTransitions::Up);
        }
        KeyCode::Char('l') => {
            let _ = app.tx.send(NewsroomTransitions::FetchMedia([].to_vec()));
        }
        KeyCode::Enter => {
            app.open_selected();
        },
        _ => {}
    }
    Ok(())
}
