// src/lib.rs
use std::time::Duration;

pub mod timer;
pub mod audio;

pub struct Config {
    pub db_path: String,
    pub assets_path: String,
}
