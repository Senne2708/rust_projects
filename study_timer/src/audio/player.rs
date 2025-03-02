use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

pub struct AudioPlayer {
    assets_path: String,
}

impl AudioPlayer {
    pub fn new(assets_path: &str) -> Self {
        Self {
            assets_path: assets_path.to_string(),
        }
    }

    pub fn loop_rain_noise(&self, duration: Duration) {
        let start = Instant::now();

        while start.elapsed() < duration {
            let remaining_time = duration - start.elapsed();
            let play_duration = remaining_time.min(Duration::from_secs(120));

            let mut process = Command::new("afplay")
                .arg(format!("{}/rain_loop.mp3", self.assets_path))
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to play sound");

            thread::sleep(play_duration);

            if start.elapsed() >= duration {
                let _ = process.kill();
                break;
            }
        }
    }

    pub fn play_finished_sound() {
        Command::new("afplay")
            .arg("assets/alarm2.wav")
            .spawn()
            .expect("Failed to play finished sound");
    }
}
