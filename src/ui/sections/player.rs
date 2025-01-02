use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders},
    Frame,
};
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

    pub fn render_progress_bar(&self, music_player: &MusicPlayer, frame: &mut Frame, area: Rect) {
        let playback_block = Block::default().borders(Borders::ALL).title("Playback");
        frame.render_widget(playback_block, area);

        let progress_bar = music_player.draw_progress_bar(area);
        frame.render_widget(progress_bar, area);
    }
}
