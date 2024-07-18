use std::collections::HashMap;

use config::{Config, ConfigError, File};
use log::error;
use serde::Deserialize;

use crate::programs::{
    hyprland::Hyprland, kitty::Kitty, nvim::Nvim, swww::Swww, tmux::Tmux, zsh::Zsh,
};

use super::colorscheme::Colorscheme;

#[derive(Debug, Deserialize)]
struct General {
    pub colorscheme: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    general: General,
    colorschemes: HashMap<String, Colorscheme>,
    hyprland: Hyprland,
    kitty: Kitty,
    swww: Swww,
    tmux: Tmux,
    zsh: Zsh,
    nvim: Nvim,
}

impl Settings {
    pub fn new(preset: &str) -> Result<Self, ConfigError> {
        let con = Config::builder()
            .add_source(File::with_name("config/default.toml"))
            .add_source(File::with_name("config/debug.toml"))
            .add_source(File::with_name("config/colorschemes/default.toml"))
            .add_source(File::with_name(&format!("config/presets/{}.toml", &preset)))
            .build()?;

        con.try_deserialize()
    }

    pub fn apply(&self) {
        let colorscheme = self.colorschemes.get(&self.general.colorscheme);

        if let Err(e) = self.hyprland.apply(colorscheme) {
            error!("Error applying Hyprland settings: {:?}", e);
        }

        self.kitty.apply(colorscheme);
        self.swww.apply();
        self.tmux.apply(colorscheme);
        self.zsh.apply(colorscheme);
        self.nvim.apply();
    }
}
