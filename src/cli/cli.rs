use clap::{Parser, Subcommand};

use crate::settings::settings::Settings;

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
    Set { preset_name: String },
}

impl Cli {
    pub fn run(&self) {
        match &self.command {
            Commands::Presets { preset_commands } => match preset_commands {
                PresetCommands::List => {
                    println!("This is still todo :)");
                }
                PresetCommands::Set { preset_name } => {
                    println!("set");

                    let settings =
                        Settings::new(preset_name).expect("Error while reading in the config.");
                    settings.apply();
                }
            },
        }
    }
}
