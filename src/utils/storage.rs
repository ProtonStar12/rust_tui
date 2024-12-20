use anyhow::{Result,Context};
use serde_json;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

use crate::state::todo::Todo;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TodoStorage;

impl TodoStorage {
    pub fn save_todos(todos: &[Todo], path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(todos)
        .context("failde to serialize")?;
        let mut file = File::create(path)
        .context("Failed to create file")?;
        file.write_all(json.as_bytes())
        .context("failed to write")?;
        

        Ok(())

    }

    pub fn load_todos(path: &Path) -> Result<Vec<Todo>> {
        if !path.exists() {
            return Ok(Vec::new());
        }
        let mut file = File::open(path)
        .context("failed to open storage")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
        .context("failed to read ")?;
        let todos = serde_json::from_str(&contents)
        .context("failed to deserialize todos")?;
        
        Ok(todos)
    }
}