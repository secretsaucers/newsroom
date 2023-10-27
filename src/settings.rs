use std::{fs, error::Error};
use log::{warn, info};

use tui::style::{Style, Color, Modifier};
use toml;
use serde::{Deserialize, Serialize};
use crate::newsroomcore::datasources::DataSources;

/// Struct to store primary settings used for the app
/// Contains both news sources and theme info
#[derive(Debug, Clone)]
pub struct Settings {
    pub theme: Theme,
    pub sources: Vec<DataSources>,
}

/// Struct to store configuration we get from config file
#[derive(Debug, Deserialize, Serialize)]
struct Config {
    theme: String,
    sources: Vec<DataSources>,
}

impl Default for Config {
    /// Creates a default config as a starting point
    /// Use default theme and some standard sources
    fn default() -> Self {
        // Default sources
        let cbc = DataSources {
            name: "cbc".to_string(),
            url: "https://www.cbc.ca/cmlink/rss-topstories".to_string(),
        };
        let cnn = DataSources {
            name: "cnn".to_string(),
            url: "http://rss.cnn.com/rss/cnn_topstories.rss".to_string(),
        };
        let globe: DataSources = DataSources {
            name: "globe and mail".to_string(),
            url: "https://www.theglobeandmail.com/arc/outboundfeeds/rss/category/canada/"
                .to_string(),
        };
        let sources = vec![cbc, cnn, globe];

        // Default theme
        let theme = "default".to_string();
        
        Self { theme, sources }
    }
}

impl Config {
    /// Writes the settings to a file
    fn write_config_to_file(&self, file: &String){
        let res = fs::write(file, toml::to_string(self).unwrap());
        match res {
            Ok(_) => info!("File write successful"),
            Err(e) => warn!("Config write failed \n {}", e),
        };
    }

    /// Loads the config file to a string
    /// 
    /// Arguments
    /// * file - A string representing the config file location with respect to the file root
    fn config_to_toml(file: &String) -> Result<Config, Box<dyn Error>>{
        // Try to load file into a TOML table
        Ok(toml::from_str(&fs::read_to_string(file)?)?)
    }

    /// Returns a string representing the path to the .newsroom.toml file for the particular system
    /// On linux this leads to ~/.newsroom.toml
    /// On Windows this leads to ~/.newstoom.toml
    /// On MacOS this leads to ~/.newstoom.toml
    fn config_path() -> String {
        r"C:\Users\space\.newsroom.toml".to_string()
    }
}

impl Settings{
    /// Creates a new instance of settings
    /// First checks the system for a .newsroom.toml file 
    /// If a .newsroom.toml file is found, we read the file and copy the settings into it
    /// If a .newsroom.toml file is NOT found, we create the file with defaults
    pub fn new() -> Settings {
        info!("Creating settings");
        let config_path = Config::config_path();

        let config = match Config::config_to_toml(&config_path) {
            Ok(loaded_config) => {
                // We've successfully loaded the config file
                info!("Loaded the following config {:#?}", loaded_config);
                loaded_config
            },
            Err(e) => {
                warn!("Couldn't load config file, error follows \n {}", e);
                // Create a default config and save it to the file
                let default_config = Config::default();
                default_config.write_config_to_file(&config_path);
                default_config
            },
        };

        let theme = Theme::new();
        Settings { theme, sources: config.sources }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub root: Style,
    pub content: Style,
    pub app_title: Style,
    pub tabs: Style,
    pub tabs_selected: Style,
    pub borders: Style,
    pub description: Style,
    pub description_title: Style,
    pub keybinding: KeyBinding,
}

impl Theme {
    pub fn new() -> Theme{
        Theme{
            name: "Default".to_string(),
            root: Style::new().bg(DARK_BLUE),
            content: Style::new().bg(DARK_BLUE).fg(LIGHT_GRAY),
            app_title: Style::new()
                .fg(WHITE)
                .bg(DARK_BLUE)
                .add_modifier(Modifier::BOLD),
            tabs: Style::new().fg(MID_GRAY).bg(DARK_BLUE),
            tabs_selected: Style::new()
                .fg(WHITE)
                .bg(DARK_BLUE)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),
            borders: Style::new().fg(LIGHT_GRAY),
            description: Style::new().fg(LIGHT_GRAY).bg(DARK_BLUE),
            description_title: Style::new().fg(LIGHT_GRAY).add_modifier(Modifier::BOLD),
            keybinding: KeyBinding {
                key: Style::new().fg(BLACK).bg(DARK_GRAY),
                description: Style::new().fg(DARK_GRAY).bg(BLACK),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub key: Style,
    pub description: Style,
}

const DARK_BLUE: Color = Color::Rgb(16, 24, 48);
const LIGHT_BLUE: Color = Color::Rgb(64, 96, 192);
const LIGHT_YELLOW: Color = Color::Rgb(192, 192, 96);
const LIGHT_GREEN: Color = Color::Rgb(64, 192, 96);
const LIGHT_RED: Color = Color::Rgb(192, 96, 96);
const RED: Color = Color::Indexed(160);
const BLACK: Color = Color::Indexed(232); // not really black, often #080808
const DARK_GRAY: Color = Color::Indexed(238);
const MID_GRAY: Color = Color::Indexed(244);
const LIGHT_GRAY: Color = Color::Indexed(250);
const WHITE: Color = Color::Indexed(255); // not really white, often #eeeeee