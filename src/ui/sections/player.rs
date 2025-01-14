use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style,Color},
    widgets::{Paragraph,Block, Borders},
    Frame,
    text::Span,
};
use std::fmt::Write;
use ratatui_image::Image;
use crate::state::player::MusicPlayer;

pub struct PlayerRenderer;

impl PlayerRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render_player(&self, music_player: &MusicPlayer, frame: &mut Frame, area: Rect) {
        // Split the area for image viewer and vinyl
        let sections = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Image Viewer Section
        let image_block = Block::default()
            .borders(Borders::ALL)
            .title_bottom("| Playing...     |");
        let mut image_area = image_block.inner(sections[0]);
        image_area.x += music_player.image_offset.0;
        image_area.y += music_player.image_offset.1;
        frame.render_widget(image_block, sections[0]);

        if let Some(image_static) = &music_player.image_static {
            let image = Image::new(image_static);
            frame.render_widget(image, image_area);
        }

       
        // Vinyl Section
        frame.render_widget(music_player.draw_vinyl(sections[1]), sections[1]);
    }

    pub fn name(&self,music_player: &MusicPlayer,frame: &mut Frame,area: Rect) {
        let name_text = music_player.get_name();

        let actual = format!(" > {}",name_text);
        let name_paragragh = Paragraph::new(actual)
        .style(Style::default().fg(Color::Blue))
        .block(Block::default().borders(Borders::NONE));

    frame.render_widget(name_paragragh, area);
    }
    
    pub fn info(&self, music_player: &MusicPlayer, frame: &mut Frame, area: Rect) {
        let sec = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Adjusted to split evenly
            Constraint::Percentage(50),
        ])
        .split(area);
    
  // Calculate total duration text
let total_duration_text = music_player.total_duration.map_or("Unknown".to_string(), |d| {
    let minutes = d.as_secs() / 60;
    let seconds = d.as_secs() % 60;
    format!("{:02}:{:02}", minutes, seconds)
});

// Calculate elapsed time text
let elapsed_time_text = music_player.total_duration.map_or("Unknown".to_string(), |d| {
    let elapsed_secs = (d.as_secs_f64() * music_player.get_playback_progress() as f64) as u64;
    let minutes = elapsed_secs / 60;
    let seconds = elapsed_secs % 60;
    format!("{:02}:{:02}", minutes, seconds)
});

// Combine elapsed time and total duration into one text
let combined_info_text = format!(
    "{} | {}",
    elapsed_time_text, total_duration_text
);

// Create and render a paragraph with both elapsed and total duration information
let info_paragraph = Paragraph::new(combined_info_text)
    .style(Style::default().fg(Color::Blue))
    .block(Block::default().borders(Borders::NONE));

// Render the paragraph
frame.render_widget(info_paragraph, sec[0]);
    
        
    }
    pub fn render_progress_bar(&self, music_player: &MusicPlayer, frame: &mut Frame, area: Rect) {
        let playback_block = Block::default();
        frame.render_widget(playback_block, area);

        let progress_bar = music_player.draw_progress_bar(area);
        frame.render_widget(progress_bar, area);
    }
}
