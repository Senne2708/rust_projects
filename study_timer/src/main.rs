// src/main.rs
use std::{io, sync::Arc};
use crossterm::terminal;
use study_timer::{
    Config,
    timer::{countdown::Timer, display::DisplayManager},
    audio::player::AudioPlayer,
};

fn main() -> io::Result<()> {
    let config = Config {
        db_path: "timer.db".to_string(),
        assets_path: "assets".to_string(),
    };

    terminal::enable_raw_mode()?;
    
    let display = DisplayManager::new();
    let duration = display.select_duration()?;
    
    let audio_player = AudioPlayer::new(&config.assets_path);
    let timer = Timer::new(duration);
    
    let audio = Arc::new(AudioPlayer::new("assets"));
    timer.start(&display, Arc::clone(&audio))?;
    
    terminal::disable_raw_mode()?;
    Ok(())
}
