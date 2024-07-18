use hyprland::{keyword::Keyword, shared::HyprError};
use log::{debug, info};
use serde::Deserialize;

use crate::colorscheme::Colorscheme;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Hyprland {
    enable: bool,
    opacity: f32,
    blur: Blur,
    rounding: i32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Blur {
    pub enabled: bool,
    pub size: i32,
    pub passes: i32,
}

impl Hyprland {
    pub fn apply_hyprland(&self, colorscheme: Option<&Colorscheme>) -> Result<(), HyprError> {

        if !self.enable {
            debug!("Skipping Hyprland because it is not enabled");
            return Ok(());
        }
        debug!("Setting Hyprland settings");

        match colorscheme {
            Some(colorscheme) => {
                // Border Color
                debug!("Setting Hyprland <border_color>");
                // TODO: allow multi colors
                Keyword::set("general:col.active_border", colorscheme.base0d)?; //  and base08
                Keyword::set("general:col.inactive_border", colorscheme.base04)?;
            }
            None => {
                info!("Skipping Hyprland <borders> because no colorscheme defined.")
            }
        }

        // Opacity
        debug!("Setting Hyprland <opacity>");
        Keyword::set("decoration:active_opacity", self.opacity)?;

        // Blur
        if self.blur.enabled {
            debug!("Setting Hyprland <blur> to enabled and applying settings");
            Keyword::set("decoration:blur:enabled", self.blur.enabled.to_string())?;
            Keyword::set("decoration:blur:size", self.blur.size.to_string())?;
            Keyword::set("decoration:blur:passes", self.blur.passes.to_string())?;
        } else {
            debug!("Setting Hyprland <blur> to disabled");
            Keyword::set("decoration:blur:enabled", self.blur.enabled.to_string())?;
        }

        // Rounding
        debug!("Setting Hyprland <rounding>");
        Keyword::set("decoration:rounding", self.rounding)?;

        Ok(())
    }
}
