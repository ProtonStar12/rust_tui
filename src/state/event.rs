use std::future::IntoFuture;

use anyhow::Result;
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use crossterm::event::{ KeyEvent,KeyModifiers};  // Add this import
use crossterm::event;
use std::time::Duration;

use crate::state::browser::MusicBrowser;
use crate::state::player::MusicPlayer;
use crate::state::app::App;
use crate::utils::art::get_album_art;
use std::path::Path;
use crate::state::app::{ InputMode};
use crate::state::todo::{Todo, TodoManager};

pub struct EventHandler;

impl EventHandler {
    pub fn handle_event(app_state: &mut App) -> Result<bool> {
        // Poll for events with a timeout
        if event::poll(Duration::from_millis(1))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    Self::handle_key_event(app_state, key)
                }
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    fn handle_key_event(app_state: &mut App, key: KeyEvent) -> Result<bool> {
        use crate::state::app::InputMode;

        match (key.code, app_state.get_input_mode()) {
            (KeyCode::Char('q'), _) => return Ok(true),  // Exit the application

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

            (KeyCode::Char('b'), InputMode::Normal|InputMode::Player) => {
                if let Err(e) = app_state.show_music_browser() {
                    log::error!("Failed to show browser: {}", e);
                }
                app_state.set_input_mode(InputMode::Browser);
            }

            // Browser controls
            (KeyCode::Up, InputMode::Browser) => {
                if let Some(browser) = &mut app_state.music_browser {
                    browser.move_selection_up();
                }
            }

            (KeyCode::Down, InputMode::Browser) => {
                if let Some(browser) = &mut app_state.music_browser {
                    browser.move_selection_down();
                }
            }

            (KeyCode::Enter, InputMode::Browser) => {
                if let Some(browser) = &mut app_state.music_browser {
                    let is_dir = browser.select_item().map(|item| item.is_dir).unwrap_or(false);
                    let item_name = browser.select_item().map(|item| item.name.clone());

                    if let Some(name) = item_name {
                        if is_dir {
                            if let Err(e) = browser.enter_directory(&name, app_state.song_mapping.as_ref()) {
                                log::error!("Failed to enter directory: {}", e);
                            }
                        } else {
                            if let Err(e) = browser.play_selected() {
                                log::error!("Failed to play selected item: {}", e);
                            }
                        }
                    }
                }
                if let Err(e) = app_state.initialize_music_player_from_browser()  {
                    log::error!("failed to put player: {}",e);
                    
                }
                

              
            }

            (KeyCode::Char('v'), InputMode::Browser) => {
                app_state.set_input_mode(InputMode::Player);
            }

            // Player controls
            (_, InputMode::Player) => {
                if let Some(player) = &mut app_state.music_player {
                    player.handle_key(key);  // Handle the key event in the player
                }
            }

            // Exiting music browser
            (KeyCode::Char('e'), InputMode::Browser) => {
                if let Err(e) = App::handle_music_browser_exit(app_state) {
                    log::error!("Failed to exit: {}", e);
                }
                if let Err(e) = App::handle_player_exit(app_state){
                    log::error!("failed to exit: {}",e);
                }
                app_state.set_input_mode(InputMode::Normal);
            }

            // Input handling
            (KeyCode::Char(c), InputMode::Editing) => {
                app_state.push_to_input(c);
            }

            (KeyCode::Backspace, InputMode::Editing) => {
                app_state.pop_from_input();
            }

            // Todo toggle
            (KeyCode::Char('t'), InputMode::Normal) => {
                if let Some(first_todo) = app_state.get_todos().first() {
                    app_state.toggle_todo(first_todo.id);
                }
            }

            _ => {}
        }
        Ok(false)
    }
}