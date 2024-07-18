use config::{Config, ConfigError, File};
use log::error;
use serde::Deserialize;

use crate::{colorscheme::Colorscheme, programs::hyprland::Hyprland};

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    colorscheme: Option<Colorscheme>,
    hyprland: Hyprland,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let con = Config::builder()
            .add_source(File::with_name("config/default.toml"))
            // .add_source(File::with_name("~/.config/themer.toml"))
            .build()?;

        con.try_deserialize()
    }

    pub fn apply(&self) {
        if let Err(e) = self.hyprland.apply_hyprland(self.get_colorscheme()) {
            error!("Error applying Hyprland settings: {:?}", e);
        }
    }

    pub fn get_colorscheme(&self) -> Option<&Colorscheme> {
        // TODO: implement handling of no colorscheme specified
        self.colorscheme.as_ref()
    }
}
