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

    pub fn render_browser<'a>(
        items: &'a [MusicItem],
        browser: &'a MusicBrowser,
        input_mode: InputMode,
        visible_count: usize,
    ) -> List<'a> {
        // Determine the visible range
        let visible_start = browser.scroll_offset;
        let visible_end = (visible_start + visible_count).min(items.len());
        let visible_items = &items[visible_start..visible_end];
    
        let things: Vec<ListItem> = visible_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let absolute_index = i + visible_start;
                let prefix = if item.is_dir {
                    "[DIR]"
                } else if item.video_path.is_some() {
                    ""
                } else {
                    "[MUSIC]"
                };
    
                let style = if absolute_index == browser.selected_index {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                };
    
                let content = format!("{:>70}", format!("{} {}", prefix, item.name));
                ListItem::new(content).style(style)
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

