use hyprland::{keyword::Keyword, shared::HyprError};
use log::{debug, info};
use serde::Deserialize;

use crate::settings::colorscheme::Colorscheme;

#[derive(Debug, Deserialize)]
pub struct Hyprland {
    enable: bool,
    opacity: f32,
    blur: Blur,
    rounding: i32,
}

#[derive(Debug, Deserialize)]
struct Blur {
    pub enable: bool,
    pub size: i32,
    pub passes: i32,
}

impl Hyprland {
    pub fn apply(&self, colorscheme: Option<&Colorscheme>) -> Result<(), HyprError> {
        if !self.enable {
            debug!("Skipping Hyprland because it is not enabled");
            return Ok(());
        }
        debug!("Setting Hyprland settings");

        // Border Color
        match colorscheme {
            Some(colorscheme) => {
                // TODO: allow multi colors
                debug!(
                    "Setting Hyprland <active_border_color> to {}",
                    colorscheme.base0d
                );
                Keyword::set("general:col.active_border", colorscheme.base0d)?; //  and base08

                debug!(
                    "Setting Hyprland <inactive_border_color> to {}",
                    colorscheme.base04
                );
                Keyword::set("general:col.inactive_border", colorscheme.base04)?;
            }
            None => {
                info!("Skipping Hyprland <borders> because no colorscheme defined.")
            }
        }

        // Opacity
        debug!("Setting Hyprland <opacity> to {}", self.opacity);
        Keyword::set("decoration:active_opacity", self.opacity)?;

        // Blur
        if self.blur.enable {
            debug!("Setting Hyprland <blur> to enabled and applying settings");
            Keyword::set("decoration:blur:enabled", self.blur.enable.to_string())?;
            Keyword::set("decoration:blur:size", self.blur.size.to_string())?;
            Keyword::set("decoration:blur:passes", self.blur.passes.to_string())?;
        } else {
            debug!("Setting Hyprland <blur> to disabled");
            Keyword::set("decoration:blur:enabled", self.blur.enable.to_string())?;
        }

        // Rounding
        debug!("Setting Hyprland <rounding> to {}", self.rounding);
        Keyword::set("decoration:rounding", self.rounding)?;

        Ok(())
    }
}
