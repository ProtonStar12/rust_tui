use ratatui::{
    prelude::*,
    widgets::{Block,List,ListItem,Borders,Paragraph},
};
use crate::state::todo::Todo;
use crate::state::app::InputMode;

pub struct TodoListRenderer;

impl TodoListRenderer {
    pub fn render_todo_list(todos: &[Todo]) -> List {
        let items: Vec<ListItem> = todos
        .iter()
        .map(|todo| {
            let content = format!(
                "{} {}",
                if todo.completed {"*"} else {" "},
                todo.title
            );
            ListItem::new(content)
        })
        .collect();
    List::new(items)
    .block(Block::default().borders(Borders::ALL).title("Todos"))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))

    }

    pub fn render_input_block(input: &str, input_mode: InputMode) -> Paragraph {
        Paragraph::new(input.to_string())
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Add todo")
            .style(match input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
                InputMode::Browser => Style::default(),
                InputMode::Player => Style::default(),
            }))
    }
}