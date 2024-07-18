use clap::Parser;
use cli::cli::Cli;

mod cli {
    pub mod cli;
}
mod settings {
    pub mod colorscheme;
    pub mod settings;
}
mod programs {
    pub mod hyprland;
    pub mod kitty;
    pub mod nvim;
    pub mod swww;
    pub mod tmux;
    pub mod zsh;
}

fn main() {
    let env = env_logger::Env::default()
        .filter_or("THEMER_LOG_LEVEL", "trace")
        .write_style_or("THEMER_LOG_LEVEL", "always");

    env_logger::init_from_env(env);

    let cli = Cli::parse();
    cli.run();
}
