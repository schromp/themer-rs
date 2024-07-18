use std::process::Command;

use log::{debug, error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Swww {
    enable: bool,
    wallpaper_dir: String,
    wallpaper: String,
}

impl Swww {
    pub fn apply(&self) {
        if !self.enable {
            debug!("Skipping SWWW because it is not enabled");
            return;
        }

        let path = format!("{}/{}", self.wallpaper_dir, self.wallpaper);
        debug!("Applying wallpaper using swww from path: {}", path);
        if let Err(e) = Command::new("swww").args(["img", &path]).output() {
            error!("Error while applying new wallpaper: {}", e);
        }
    }
}
