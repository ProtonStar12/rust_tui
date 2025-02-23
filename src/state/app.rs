use anyhow::Result;
use ratatui::Terminal;
use ratatui::Frame;
use ratatui::backend::Backend;
use crate::state::todo::{Todo,TodoManager};
use crate::ui::layout::layout2;
use crate::ui::sections::music_browser::BrowserRenderer;
use std::path::PathBuf;
use std::error::Error;
use ratatui::widgets::{Paragraph,Block,Borders};
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::process::{Stdio,Command};
use std::io::BufReader;
use std::io::BufRead;
use ratatui::widgets::canvas::{Canvas,Line};
//
use crate::ui::layout::create_layout;
use crate::ui::sections::todo_list::TodoListRenderer;
use crate::state::event::EventHandler;
use crate::state::browser::MusicBrowser;
use crate::state::browser::{MusicItem,SongMapping};
use crate::ui::sections::player::PlayerRenderer;
use image::DynamicImage;
use super::browser;
use super::player::MusicPlayer;
use crate::utils::art::get_album_art;
use crate::state::event::VISIBLE_COUNT;


pub struct App {
    todos: Vec<Todo>,
    input: String,
    input_mode: InputMode,
    next_id: usize,
    pub music_browser: Option<MusicBrowser>,
    pub song_mapping: Option<SongMapping>,
    pub music_player: Option<MusicPlayer>,
    pub cava_bars:  Option<Vec<u8>>,
    pub from_player: bool,
    pub last_browser_index: Option<usize>,
    
}
#[derive(Clone, Copy)]
pub(crate) enum InputMode {
    Normal,
    Editing,
    Browser,
    Player,
}

impl App {
    pub fn new() -> Self {
       

        Self {
            todos: Vec::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
            next_id: 1,
            music_browser: None,
            song_mapping: SongMapping::load_from_file("/home/vinay/songs.yaml").ok(),
            music_player: None,
            cava_bars: Some(vec![0;32]),
            from_player: false,
            last_browser_index: None,
           

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
            music_browser: None,
            song_mapping: SongMapping::load_from_file("/home/vinay/songs.yaml").ok(),
            music_player:None,
            cava_bars: Some(vec![0;32]),
            from_player: false,
            last_browser_index: None,
           



        }
    }
    pub fn run(&mut self,terminal: &mut Terminal<impl Backend>) -> Result<()> {
         let frame_time = Duration::from_millis(16);
      
         // Spawn a thread to run CAVA and update bars
       
     
        loop {
            let start = std::time::Instant::now();
       
        
              if let Some(ref mut player) = self.music_player {
                player.update();
           
            }
            terminal.draw(|frame| self.render(frame))?;
         
            thread::sleep(Duration::from_millis(50));

            
             
             if EventHandler::handle_event(self)? {
                break;
             }
             let elapsed = start.elapsed();
            if elapsed < frame_time {
                thread::sleep(frame_time - elapsed);
            }
        }
        Ok(())
    }


    fn render(& mut self, frame: &mut Frame) {
        
        match self.input_mode { 
            InputMode::Normal | InputMode::Editing => {
                let layout = create_layout(frame.area());
                let todo_list = TodoListRenderer::render_todo_list(&self.todos);
        frame.render_widget(todo_list, layout[1]);

        let input_block = TodoListRenderer::render_input_block(&self.input, self.input_mode);
        frame.render_widget(input_block, layout[0]);

            }

            InputMode::Browser | InputMode::Player => {
                let area = frame.area();
                let (main_chunks, left_chunks, right_chunks, image_vinyl_chunks) = layout2(area);
                if let Some(browser) = &self.music_browser {
                    // Pass browser items, browser reference, input mode, and visible count to the renderer
                    let list = BrowserRenderer::render_browser(&browser.items, browser, self.input_mode, VISIBLE_COUNT);
                    frame.render_widget(list, right_chunks[0]);
                } else {
                    let error_widget = Paragraph::new("No browser available")
                        .block(Block::default().title("Error").borders(Borders::ALL));
                    frame.render_widget(error_widget, main_chunks[1]);
                }
            
                if let Some(music) = &mut self.music_player {
                    let player_render = PlayerRenderer::new();
                    player_render.render_player(music, frame, image_vinyl_chunks[0]);
                    player_render.name(music, frame, image_vinyl_chunks[1]);
                    player_render.info(music, frame, image_vinyl_chunks[2]);
                    player_render.render_progress_bar(music, frame, image_vinyl_chunks[3]);
                    // music.update();
                }

            }
            
                
                
            

            
        }
  
    }

    pub fn initialize_music_player_from_browser(&mut self) -> Result<()> {
        // Ensure the MusicBrowser is initialized
        if let Some(browser) = &self.music_browser {
            // Get the selected music path
            if let Some(song_path) = browser.get_selected_music_path() {
                // Dynamically get album art based on the song path
                let album_art = get_album_art(&song_path).or_else(|_| {
                    anyhow::bail!("Failed to retrieve album art for the song: {}", song_path)
                })?;
                

                // Initialize the music player
                self.music_player = Some(MusicPlayer::new(album_art, &song_path)?);

                return Ok(());
            } else {
                anyhow::bail!("No song selected in the music browser");
            }
        } else {
            anyhow::bail!("Music browser is not initialized");
        }
    }

    pub fn handle_music_browser_exit(app_state: &mut App) -> Result<(), Box<dyn Error>> {
        app_state.clear_music_browser()?;
        if let Some(browser)= &mut  app_state.music_browser {
            browser.kill_current_player();
        }
        app_state.set_input_mode(InputMode::Normal);
        Ok(())
    }

    pub fn show_music_browser(&mut self) -> Result<()> {
        let browser = MusicBrowser::new(
            &PathBuf::from("/home/vinay/Music/"),
            self.song_mapping.as_ref(),
        )?;
        self.music_browser = Some(browser);
        self.input_mode = InputMode::Browser;
        Ok(())
    }

    pub fn clear_music_browser(&mut self) -> Result<()> {
        if let Some(browser) = &mut self.music_browser {
            browser.cleanup()?;
        }
        self.music_browser = None;
        Ok(())
    }
    pub fn cleanup_music_player(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(player) = &mut self.music_player {
            player.cleanup();
        }
        self.music_player = None;
        Ok(())
    }


    pub fn kill_vod(&mut self) {
        if let Ok(mut child) = Command::new("pkill")
            .arg("mpvpaper")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            let _ = child.wait(); // Wait for pkill to complete
        }
    }
    
    pub fn handle_player_exit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.cleanup_music_player()?;
        self.set_input_mode(InputMode::Normal);
        Ok(())
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

    pub fn cleanup(&mut self) -> Result<()> {
        self.clear_music_browser()
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