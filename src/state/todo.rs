use chrono::{DateTime, Local};
use serde::Deserialize;
use  serde::Serialize;


#[derive(Serialize,Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: usize, title:String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }
    pub fn toggle_complete(&mut self) {
        self.completed = !self.completed;
    }
}

pub trait TodoManager {
     fn add_todo(&mut self, title:String);
     fn remove_todo(&mut self, id:usize);
     fn toggle_todo(&mut self, id:usize);
     fn get_todos(&self) -> &Vec<Todo>;
    
}