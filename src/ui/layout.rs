use ratatui::{prelude::*,widgets::*,layout::{Constraint,Direction,Layout}};

pub fn create_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),
        Constraint::Min(0),

    ])
    .split(area).to_vec()
}