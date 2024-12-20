use anyhow::Result;
use crossterm::event::{read, Event, KeyCode, KeyEventKind};


pub struct EventHandler;

impl EventHandler {

    pub fn handle_event(app_state: &mut crate::state::app::App) -> Result<bool> {
        match read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                Self::handle_key_event(app_state, key.code)
            }
            _ => Ok(false),
        }
    }

    fn handle_key_event(app_state: &mut crate::state::app::App, key: KeyCode) -> Result<bool> {
        use crate::state::todo::{Todo, TodoManager};
        use crate::state::app::InputMode;

        match(key, app_state.get_input_mode()) {
            (KeyCode::Char('q'), _) => return Ok(true),

            (KeyCode::Char('i'), InputMode::Normal) => {
                app_state.set_input_mode(InputMode::Editing);
            }

            (KeyCode::Enter, InputMode::Editing) => {
                if !app_state.get_input().is_empty() {
                    app_state.add_todo(app_state.get_input().to_string());
                    app_state.clear_input();
                    app_state.set_input_mode(InputMode::Normal);
                }
            }

            //input handling
            (KeyCode::Char(c), InputMode::Editing) => {
                app_state.push_to_input(c);
            }
            (KeyCode::Backspace, InputMode::Editing) => {
                app_state.pop_from_input();
            }

            //toggle
            (KeyCode::Char('t'), InputMode::Normal ) => {
                if let Some(first_todo) = app_state.get_todos().first() {
                    app_state.toggle_todo(first_todo.id);
                }
            }
            _ => {}
        }
        Ok(false)
    }
}