// src/timer/countdown.rs
use std::io;
use std::thread;
use std::time::{Duration, Instant};

use crate::audio::player::AudioPlayer;
use crate::database::database::DatabaseManager;
use crate::timer::display::DisplayManager;
use std::sync::Arc;

pub struct Timer {
    duration: Duration,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }

    pub fn start(
        &self,
        display: &DisplayManager,
        audio: Arc<AudioPlayer>,
        database_manager: &DatabaseManager,
    ) -> io::Result<()> {
        let duration = self.duration.clone();
        let audio_clone = Arc::clone(&audio);

        let rain_handle = thread::spawn(move || {
            audio_clone.loop_rain_noise(duration);
        });

        self.countdown(display)?;
        rain_handle.join().unwrap();

        let hours = duration.as_secs() as f64 / 3600.0;
        let rounded_hours = hours.round() as u64;
        let _ = database_manager.insert_data(rounded_hours, true);

        Ok(())
    }

    fn countdown(&self, display: &DisplayManager) -> io::Result<()> {
        let start = Instant::now();

        loop {
            if display.check_for_exit()? {
                break;
            }

            let elapsed = start.elapsed();

            if elapsed >= self.duration {
                display.show_finished_message()?;
                AudioPlayer::play_finished_sound();
                break;
            }

            let remaining = self.duration.checked_sub(elapsed).unwrap_or(Duration::ZERO);
            display.update_timer(remaining)?;
        }

        Ok(())
    }
}
