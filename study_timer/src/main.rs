use crossterm::terminal;
use std::sync::Arc;
use study_timer::{
    audio::player::AudioPlayer,
    database::database::DatabaseManager,
    timer::{countdown::Timer, display::DisplayManager},
    Config,
};

fn main() -> std::io::Result<()> {
    let config = Config {
        db_path: "timer.db".to_string(),
        assets_path: "assets".to_string(),
    };

    terminal::enable_raw_mode()?;

    let database_manager = DatabaseManager::new(&config.db_path).unwrap();
    DatabaseManager::create_table(&database_manager).unwrap();
    let display = DisplayManager::new();
    let duration = display.select_duration(&database_manager)?;

    let audio_player = AudioPlayer::new(&config.assets_path);
    let timer = Timer::new(duration);

    let audio = Arc::new(audio_player);
    timer.start(&display, Arc::clone(&audio), &database_manager)?;

    terminal::disable_raw_mode()?;
    Ok(())
}
