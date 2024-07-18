use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Cache {
    preset: String,
}

impl Cache {

    pub fn get_preset(&self) -> &String {
        &self.preset
    }

    pub fn create_cache(preset: String) {
        let base_dirs = xdg::BaseDirectories::new().unwrap();

        let mut cache_path = base_dirs.create_cache_directory("themer").unwrap();
        cache_path.push("themer_cache.json");

        debug!("Creating cache at {}", cache_path.display());
        if Path::new(&cache_path).exists() {
            let old_file = fs::read_to_string(&cache_path).unwrap();
            let mut old_cache: Cache = serde_json::from_str(&old_file).unwrap();

            old_cache.preset = preset;

            let mut file = File::create(&cache_path).unwrap();
            file.write_all(serde_json::to_string(&old_cache).unwrap().as_bytes())
                .unwrap();
        } else {
            let cache = Cache { preset };
            let mut file = File::create(&cache_path).unwrap();
            file.write_all(serde_json::to_string(&cache).unwrap().as_bytes())
                .unwrap();
        }
    }

    pub fn get_cache() -> Option<Cache> {
        let base_dirs = xdg::BaseDirectories::new().unwrap();

        let mut cache_path = base_dirs.create_cache_directory("themer").unwrap();
        cache_path.push("themer_cache.json");

        if Path::new(&cache_path).exists() {
            let old_file = fs::read_to_string(&cache_path).unwrap();
            let old_cache: Cache = serde_json::from_str(&old_file).unwrap();

            Some(old_cache)
        } else {
            None
        }
    }
}
