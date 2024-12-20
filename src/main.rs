use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, Terminal};
use std::{io::stdout, path::PathBuf};

mod state;
mod ui;
mod utils;

use state::app::App;
use crate::state::todo::TodoManager;
use utils::storage::TodoStorage;

fn main() -> Result<()> {
    // Initialize app with stored data
    let storage_path = PathBuf::from("todos.json");
    let todos = TodoStorage::load_todos(&storage_path).unwrap_or_default();
    let mut app = App::with_todos(todos);

    // Setup terminal
    let mut terminal = setup_terminal()?;
    
    // Run the application
    let result = app.run(&mut terminal);
    
    // Save todos before exit
    if let Err(e) = TodoStorage::save_todos(app.get_todos(), &storage_path) {
        eprintln!("Failed to save todos: {}", e);
    }

    // Cleanup terminal
    restore_terminal()?;
    
    result
}

fn setup_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}