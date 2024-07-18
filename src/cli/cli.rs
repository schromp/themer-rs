use clap::{Parser, Subcommand};

use crate::{cache::Cache, settings::settings::Settings};

#[derive(Parser)]
#[command(name = "themer")]
pub struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Presets {
        #[command(subcommand)]
        preset_commands: PresetCommands,
    },
}

#[derive(Subcommand)]
enum PresetCommands {
    List,
    Set { preset: Option<String>},
}

impl Cli {
    pub fn run(&self) {
        match &self.command {
            Commands::Presets { preset_commands } => match preset_commands {
                PresetCommands::List => {
                    println!("This is still todo :)");
                }
                PresetCommands::Set { preset } => {
                    println!("set");

                    let cache= Cache::get_cache().unwrap();

                    let preset = match preset {
                        Some(preset) => preset,
                        None => {
                            cache.get_preset()
                        },
                    };

                    let settings =
                        Settings::new(preset).expect("Error while reading in the config.");
                    settings.apply();
                    Cache::create_cache(preset.to_string());
                }
            },
        }
    }
}
