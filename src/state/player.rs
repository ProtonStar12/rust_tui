use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::{Duration, Instant};
use rodio::{source, Decoder, OutputStream, Sink, Source};
use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui_image::{picker::Picker, protocol::Protocol};
use image::DynamicImage;
use crossterm::event::{KeyCode,KeyEvent};

pub struct MusicPlayer {
    pub image_static: Option<Protocol>,
    pub image_offset: (u16, u16),
    pub vinyl_angle: f32,
    pub vinyl_speed: f32,
    pub is_paused: bool,
    pub sink: Option<Sink>,
    pub _stream: OutputStream,
    pub stream_handle: rodio::OutputStreamHandle,
    pub audio_file: String,
    pub total_duration: Option<Duration>,
    pub start_time: Option<Instant>,
    pub elapsed_before_pause: Duration,  // Total time elapsed before current pause
    pub pause_start: Option<Instant>,  
}


impl MusicPlayer {

    pub fn new(image: DynamicImage, audio_file: &str) -> Result<Self, anyhow::Error> {
        // Initialize Picker and Image Protocol
        let mut picker = Picker::from_query_stdio()?;
        picker.set_background_color(Some(image::Rgb([0, 0, 0])));
        let size = Rect::new(0, 0, 42, 42);
        let image_static = picker
            .new_protocol(image.clone(), size, ratatui_image::Resize::Fit(None))
            .ok();

        // Initialize Rodio Playback
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        
        // Get duration without consuming the source
        let file = File::open(audio_file)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader)?;
        let total_duration = source.total_duration();
        
        // Create a new source for playback
        let file = File::open(audio_file)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader)?;
        sink.append(source);
        sink.play();
        sink.set_volume(1.0);

        Ok(Self {
           
            image_static,
            image_offset: (10, 3),
            vinyl_angle: 0.0,
            vinyl_speed: 0.1,
            is_paused: false,
            sink: Some(sink),
            _stream: stream,
            stream_handle,
            audio_file: audio_file.to_string(),
            total_duration,
            start_time: Some(Instant::now()),
            elapsed_before_pause: Duration::from_secs(0),
            pause_start: None,
        })
    }

    pub fn reload_audio(&mut self) -> Result<(), Box<dyn Error>> {
        let sink = Sink::try_new(&self.stream_handle)?;
        let file = File::open(&self.audio_file)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader)?;
        sink.append(source);
        sink.play();
        sink.set_volume(1.0);
        self.sink = Some(sink);
        self.start_time = Some(Instant::now());
        self.elapsed_before_pause = Duration::from_secs(0);
        self.pause_start = None;
        self.is_paused = false;
        Ok(())
    }
    pub fn get_name(&self) -> String {
        let extract_name = |file_path: &str| {
            Path::new(file_path)
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(|s| s.to_string()) // Convert to owned `String`
        };
    
        extract_name(&self.audio_file).unwrap_or_else(|| "not found".to_string())
    }
    
    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('h') => {
                if self.image_offset.0 > 0 {
                    self.image_offset.0 -= 1;
                }
            }
            KeyCode::Char('l') => self.image_offset.0 += 1,
            KeyCode::Char('k') => {
                if self.image_offset.1 > 0 {
                    self.image_offset.1 -= 1;
                }
            }
            KeyCode::Char('j') => self.image_offset.1 += 1,
            KeyCode::Char(' ') => {
                if let Some(sink) = &self.sink {
                    if self.is_paused {
                        // Resuming playback
                        sink.play();
                        if let Some(pause_time) = self.pause_start {
                            // Add the pause duration to elapsed_before_pause
                            self.elapsed_before_pause += pause_time.elapsed();
                        }
                        self.pause_start = None;
                        self.start_time = Some(Instant::now());
                    } else {
                        // Pausing playback
                        sink.pause();
                        self.pause_start = Some(Instant::now());
                        // When pausing, update elapsed_before_pause with the time since last start/resume
                        if let Some(start) = self.start_time {
                            self.elapsed_before_pause += start.elapsed();
                        }
                    }
                    self.is_paused = !self.is_paused;
                }
            }
            KeyCode::Char('r') => {
                if let Err(e) = self.reload_audio() {
                    eprintln!("Error reloading audio: {}", e);
                }
            }
            KeyCode::Char('+') => {
                self.vinyl_speed += 0.05;
            }
            KeyCode::Char('-') => {
                if self.vinyl_speed > 0.05 {
                    self.vinyl_speed -= 0.05;
                }
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        if !self.is_paused {
            self.vinyl_angle += self.vinyl_speed;
        }
        log::info!("Vinyl angle: {}", self.vinyl_angle);
    }


  
    pub fn get_playback_progress(&self) -> f32 {
        if let (Some(start_time), Some(total_duration)) = (self.start_time, self.total_duration) {
            let current_elapsed = if self.is_paused {
                // If paused, just use the time elapsed before pause
                self.elapsed_before_pause
            } else {
                // If playing, add the current segment to previously elapsed time
                self.elapsed_before_pause + start_time.elapsed()
            };
            
            let progress = current_elapsed.as_secs_f32() / total_duration.as_secs_f32();
            return progress.min(1.0);
        }
        0.0
    }

    pub fn draw_vinyl(&self, area: Rect) -> Paragraph<'static> {
        let vinyl_radius = 14;
        let center = vinyl_radius as f32;
        let vertical_scale = 2.0;

        let x_offset = -1;
        let y_offset = 12;

        let mut vinyl_lines = Vec::new();

        for y in 0..vinyl_radius * 2 {
            let mut line_spans = Vec::new();
            for x in (vinyl_radius - 3)..(vinyl_radius * 2 + 3) {
                let dx = (x as f32 - center) + x_offset as f32;
                let dy = ((y as f32 - center) * vertical_scale) + y_offset as f32;
                let dist = (dx * dx + dy * dy).sqrt();
                let angle = dy.atan2(dx) + self.vinyl_angle;

                let span = if dist < vinyl_radius as f32 {
                    if dist > (vinyl_radius as f32 * 0.6) {
                        if angle.cos() > 0.5 || angle.cos() < -0.5 {
                            Span::styled("▓", Style::default().fg(Color::White))
                        } else {
                            Span::styled("░", Style::default().fg(Color::Gray))
                        }
                    } else if dist > (vinyl_radius as f32 * 0.4) {
                        Span::raw(" ")
                    } else {
                        Span::styled("●", Style::default().fg(Color::Gray))
                    }
                } else {
                    Span::raw(" ")
                };

                line_spans.push(span);
            }
            vinyl_lines.push(Line::from(line_spans));
        }

        Paragraph::new(vinyl_lines)
            .block(Block::default().borders(Borders::ALL).title("Vinyl"))
    }

    pub fn draw_progress_bar(&self, area: Rect) -> Gauge<'static> {
        let progress = (self.get_playback_progress() * 100.0) as u16;

        Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Playback Progress"))
            .gauge_style(Style::default().fg(Color::Green))
            .percent(progress)
    }

    pub fn cleanup(&mut self) {
        if let Some(ref mut sink) = self.sink {
            sink.stop(); // Stop audio playback
        }
        self.sink = None;
        self.start_time = None;
        self.total_duration = None;
        self.elapsed_before_pause = Duration::from_secs(0);
        self.pause_start = None;
        self.is_paused = false;
    }
}
    
