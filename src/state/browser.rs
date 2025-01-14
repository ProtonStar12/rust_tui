use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use anyhow::{Result, Context};
use log::{debug, warn, error};
use serde::Deserialize;
use std::process::{Command, Child, Stdio};

#[derive(Debug, Deserialize)]
pub struct SongMapping {
    mapping: HashMap<String, String>,
}

impl SongMapping {
    pub fn load_from_file(file_path: &str) -> Result<Self> {
        let yaml_data = fs::read_to_string(file_path)?;
        let mapping = serde_yaml::from_str(&yaml_data)?;
        Ok(SongMapping { mapping })
    }

    pub fn get_video_path(&self, song_path: &str) -> Option<&String> {
        let song_filename = Path::new(song_path).file_name()?.to_str()?;
        self.mapping.get(song_filename)
    }
    
}

#[derive(Debug, Clone)]
pub struct MusicItem {
    pub name: String,
    pub is_dir: bool,
    pub video_path: Option<PathBuf>,
    pub music_path: Option<PathBuf>,
}

impl MusicItem {

    pub fn get_music_path(&self) -> Option<String> {
        self.music_path
        .as_ref()
        .map(|path| path.to_string_lossy().into_owned())
    }
    
}
pub struct MusicBrowser {
    pub current_path: PathBuf,
    pub items: Vec<MusicItem>,
    pub selected_index: usize,
    current_player: Option<Child>,
    pub scroll_offset: usize,
}

impl MusicBrowser {
    pub fn new(initial_path: &Path, song_mapping: Option<&SongMapping>) -> Result<Self> {
        if !initial_path.exists() {
            fs::create_dir_all(initial_path)?;
        }
        let items = Self::list_directory(initial_path, song_mapping)?;
        Ok(Self {
            current_path: initial_path.to_path_buf(),
            items,
            selected_index: 0,
            current_player: None,
            scroll_offset: 0, // Initialize scroll offset
        })
    }

    pub fn list_directory(path: &Path, song_mapping: Option<&SongMapping>) -> Result<Vec<MusicItem>> {
        let mut items = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().into_string().unwrap_or_default();
            let is_dir = entry.metadata()?.is_dir();
            let video_path = song_mapping
                .and_then(|mapping| mapping.get_video_path(&name))
                .map(PathBuf::from);
            items.push(MusicItem {
                name,
                is_dir,
                video_path,
                music_path: Some(path),
            });
        }
        Ok(items)
    }

    pub fn select_item(&self) -> Option<&MusicItem> {
        self.items.get(self.selected_index)
    }

    pub fn move_selection_up(&mut self, visible_count: usize) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index;
            }
        }
    }
    pub fn move_selection_down(&mut self, visible_count: usize) {
        if self.selected_index + 1 < self.items.len() {
            self.selected_index += 1;
            if self.selected_index >= self.scroll_offset + visible_count {
                self.scroll_offset = self.selected_index - visible_count + 1;
            }
        }
    }
    pub fn enter_directory(&mut self, dir_name: &str, song_mapping: Option<&SongMapping>) -> Result<()> {
        let new_path = self.current_path.join(dir_name);
        if new_path.is_dir() {
            self.items = Self::list_directory(&new_path, song_mapping)?;
            self.current_path = new_path;
            self.selected_index = 0;
            Ok(())
        } else {
            anyhow::bail!("Not a directory: {}", dir_name)
        }
    }

    pub fn play_selected(&mut self) -> Result<()> {
        if let Some(item) = self.items.get(self.selected_index).cloned() {
            if !item.is_dir {
                self.kill_current_player()?;
                if let Some(video_path) = item.video_path {
                    let process = Command::new("mpvpaper")
                        .args(["-o", "--loop", "eDP-1", video_path.to_str().unwrap()])
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn()?;
                    self.current_player = Some(process);
                }
            }
        }
        Ok(())
    } 
     pub fn get_selected_music_path(&self) -> Option<String> {
        self.select_item()?.get_music_path()
    }


    fn kill_current_player(&mut self) -> Result<()> {
        if let Some(mut player) = self.current_player.take() {
            if player.kill().is_err() {
                Command::new("pkill")
                    .arg("mpvpaper")
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()?
                    .wait()?;
            }
        }
        Ok(())
    }
   
    pub fn cleanup(&mut self) -> Result<()> {
        self.kill_current_player()
    }
}

