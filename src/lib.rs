/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

/// Settings.
pub mod settings;

/// Root ui
pub mod root;

/// Tabs
pub mod tabs;

/// Newsroomcore stores all our background app logic in a single module.
/// It provides structures and services for defining news articles and news sources
/// As well as fetching them from RSS channels.
pub mod newsroomcore;