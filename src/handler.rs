use std::sync::Arc;

use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tokio::sync::Mutex;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app_arc: Arc<Mutex<App>>) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            let app_arc_local = app_arc.clone();
            tokio::spawn(async move {
                let mut app_local = app_arc_local.lock().await;
                app_local.quit();
            });
        },
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            let app_arc_local = app_arc.clone();
            tokio::spawn(async move {
                let mut app_local = app_arc_local.lock().await;
                app_local.quit();
            });
        },
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
            tokio::spawn(async move {
                App::load(app_arc_local).await;
            });
        }
        KeyCode::Enter => {
            let app_arc_local = app_arc.clone();
            tokio::spawn(async move {
                let app_local = app_arc_local.lock().await;
                app_local.open_selected();
            });
        },
        _ => {}
    }
    Ok(())
}
