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

pub fn layout2(area: Rect) -> (Vec<Rect>,Vec<Rect>,Vec<Rect>,Vec<Rect>) {
   let main_chunks= Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(50),
Constraint::Percentage(50)])
    .split(area).to_vec();

    let left_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
    .split(main_chunks[0]).to_vec();
 let right_chunks = Layout::default()

 .direction(Direction::Vertical)
 .constraints([Constraint::Percentage(50),
 Constraint::Percentage(50)])
 .split(main_chunks[1]).to_vec();
    let image_vinyl_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(75),
    Constraint::Percentage(12),
    Constraint::Percentage(6),
    Constraint::Percentage(7)])
    .split(left_chunks[0]).to_vec();

    (main_chunks,left_chunks,right_chunks,image_vinyl_chunks)
}

