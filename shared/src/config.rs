use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use crate::convert_to_absolute_path;

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde_inline_default(PathBuf::from("./trash/"))]
    pub trash_folder_path: PathBuf,

    #[serde_inline_default(PathBuf::from("./socket"))]
    pub socket_path: PathBuf,
}

pub fn read_config() -> Result<Config, std::io::Error> {
    let mut config_file_path = xdg_basedir::get_config_home().expect("$HOME should be set");
    config_file_path = config_file_path.join("trash_daemon/config.toml");
    eprintln!("Attempting to read config file from {:?}", config_file_path);

    if !config_file_path.exists() {
        eprintln!("Config file does not appear to exist, creating...");
        std::fs::create_dir_all(config_file_path.parent().expect("path should have parent (Are you really trying to place a config file at the FS root?)"))?;
        let mut file = File::create(&config_file_path)?;
        let default_config: Config =
            toml::from_str("").expect("empty string should result in default config");

        file.write_all(
            toml::to_string_pretty(&default_config)
                .expect("config should be serializable")
                .as_bytes(),
        )?;
        eprintln!("Default config written to {:?}", config_file_path);
    }
    let mut config: Config = toml::from_str(std::fs::read_to_string(config_file_path)?.as_str())
        .expect("Failed to parse config");

    config.socket_path = convert_to_absolute_path(config.socket_path);
    config.trash_folder_path = convert_to_absolute_path(config.trash_folder_path);

    Ok(config)
}
