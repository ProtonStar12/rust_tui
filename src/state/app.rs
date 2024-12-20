use anyhow::Result;
use ratatui::Terminal;
use ratatui::Frame;
use ratatui::backend::Backend;
use crate::state::todo::{Todo,TodoManager};
//
use crate::ui::layout::create_layout;
use crate::ui::sections::todo_list::TodoListRenderer;
use crate::state::event::EventHandler;


pub struct App {
    todos: Vec<Todo>,
    input: String,
    input_mode: InputMode,
    next_id: usize,
}
#[derive(Clone, Copy)]
pub(crate) enum InputMode {
    Normal,
    Editing,
}

impl App {
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
            next_id: 1,

        }
    }

    pub fn with_todos(todos: Vec<Todo>) -> Self {
        let next_id = todos
        .iter()
        .map(|todo| todo.id)
        .max()
        .map(|id| id+1)
        .unwrap_or(1);

        Self {
            todos,
            input: String::new(),
            input_mode: InputMode::Normal,
            next_id,
        }
    }

    pub fn run(&mut self,terminal: &mut Terminal<impl Backend>) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
             
             if EventHandler::handle_event(self)? {
                break;
             }
        }
        Ok(())
    }


    fn render(&self, frame: &mut Frame) {
        let layout = create_layout(frame.area());
        let todo_list = TodoListRenderer::render_todo_list(&self.todos);
        frame.render_widget(todo_list, layout[1]);

        let input_block = TodoListRenderer::render_input_block(&self.input, self.input_mode);
        frame.render_widget(input_block, layout[0]);
    }

    pub fn get_input_mode(&self) -> InputMode {
        return  self.input_mode;
    }
    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }
    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
    }
    pub fn push_to_input(&mut self , c: char) {
        self.input.push(c);
    }
    pub fn pop_from_input(&mut self) {
        self.input.pop();
    }
}


impl TodoManager for App {
    fn add_todo(&mut self, title: String) {
        let todo = Todo::new(self.next_id,title);
        self.todos.push(todo);
        self.next_id += 1;
    }

    fn remove_todo(&mut self, id: usize) {
        self.todos.retain(|todo| todo.id != id);
    }

    fn toggle_todo(&mut self,id:usize) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.toggle_complete();
        }
    }
    fn get_todos(&self) -> &Vec<Todo> {
        &self.todos
    }
}