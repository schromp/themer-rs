use std::collections::HashMap;

use config::{Config, ConfigError, File};
use log::error;
use serde::Deserialize;

use crate::{colorscheme::Colorscheme, programs::hyprland::Hyprland};

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct General {
    pub colorscheme: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    general: General,
    colorschemes: HashMap<String, Colorscheme>,
    hyprland: Hyprland,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let con = Config::builder()
            .add_source(File::with_name("config/default.toml"))
            .add_source(File::with_name("config/debug.toml"))
            .add_source(File::with_name("config/colorschemes/default.toml"))
            .build()?;

        con.try_deserialize()
    }

    pub fn apply(&self) {

        let colorscheme = self.colorschemes.get(&self.general.colorscheme);

        if let Err(e) = self.hyprland.apply_hyprland(colorscheme) {
            error!("Error applying Hyprland settings: {:?}", e);
        }
    }
}
