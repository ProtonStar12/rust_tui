use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};
use simplelog::*;
use std::{error::Error, fs::File, io};


use crate::state::browser::{MusicItem,MusicBrowser};
use crate::state::app::InputMode;
use crate::state::app::App;


fn init_logger() {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("debug.log").unwrap(),
    )
    .unwrap();
}

pub struct BrowserRenderer ;

impl BrowserRenderer {


    pub fn render_browser<'a>(items: &'a[MusicItem], browser: &'a MusicBrowser, input_mode: InputMode) -> List<'a>  {
        let things: Vec<ListItem> = items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                // Determine the prefix based on the item's type
                let prefix = if item.is_dir {
                    "[DIR]"
                } else if item.video_path.is_some() {
                    "[MUSIC+VIDEO]"
                } else {
                    "[MUSIC]"
                };
    
                // Apply highlight style for the selected item
                let style = if i == browser.selected_index {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                };
    
                // Create the list item with prefix and name
                ListItem::new(format!("{} {}", prefix, item.name)).style(style)
            })
            .collect();
            List::new(things)
            .block(Block::default().borders(Borders::ALL).title("Music Browser"))
            .style(match input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default(),
                InputMode::Browser => Style::default().fg(Color::Blue),
                InputMode::Player => Style::default(),
            })
    }
    
    
}

