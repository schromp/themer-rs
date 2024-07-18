use settings::settings::Settings;

mod settings {
    pub mod settings;
    pub mod colorscheme;
}
mod programs {
    pub mod hyprland;
}

fn main() {
    let env = env_logger::Env::default()
        .filter_or("THEMER_LOG_LEVEL", "trace")
        .write_style_or("THEMER_LOG_LEVEL", "always");

    env_logger::init_from_env(env);

    let settings = Settings::new().expect("Error while reading in the config.");

    settings.apply();
}
