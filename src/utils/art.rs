use std::error::Error;
use id3::Tag;
use image::{DynamicImage};

pub fn get_album_art(song_path: &str) -> Result<DynamicImage, Box<dyn Error>> {
    let tag = Tag::read_from_path(song_path)?;
    let picture = tag.pictures().next().ok_or("No album art found")?;
    let img = image::load_from_memory(&picture.data)?;
    Ok(img)
}