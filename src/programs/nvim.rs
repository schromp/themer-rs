use std::{fs::File, io::Write, process::Command};

use log::{debug, error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Nvim {
    enable: bool,
    path: String,
    theme: String,
    transparent: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct NvimConfig {
    theme: String,
    transparency: bool,
}

impl Nvim {
    pub fn apply(&self) {
        if !self.enable {
            debug!("Skipping nvim because it is not enabled");
            return;
        }

        let config = NvimConfig {
            theme: self.theme.clone(),
            transparency: self.transparent,
        };

        let config_json = serde_json::to_string(&config).unwrap();

        debug!("Writing to nvim theme file {}", self.path);
        match File::create(&self.path) {
            Ok(mut config_file) => {
                if let Err(e) = config_file.write_all(config_json.as_bytes()) {
                    error!("Error while writing to file {}: {}", self.path, e);
                }
            }
            Err(e) => error!("Error while writing to file {}: {}", self.path, e),
        }

        self.refresh(&self.theme);
    }

    fn refresh(&self, theme: &str) {
        let base_dirs = xdg::BaseDirectories::new().unwrap();

        let runtime_dir = base_dirs.get_runtime_directory().unwrap();

        let cmd = format!("execute('colorscheme {}')", theme);

        for socket in runtime_dir.read_dir().unwrap() {
            let sock_path = socket.unwrap().path().to_string_lossy().to_string();
            if sock_path.contains("nvim") {
                debug!("Setting nvim colorscheme for file {}", sock_path);
                if let Err(e) = Command::new("nvim")
                    .args(["--server", &sock_path, "--remote-expr", &cmd])
                    .output()
                {
                    error!(
                        "Error calling nvim remote server with path {}: {}",
                        sock_path, e
                    );
                }
            }
        }
    }
}
