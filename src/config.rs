use crate::cli;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub input_directory: PathBuf,
    pub output_directory: PathBuf,
}

impl Default for Configuration {
    fn default() -> Self {
        let appdata_folder = env::var("APPDATA").expect("Failed to get AppData folder");
        let mut input_directory = PathBuf::from(appdata_folder);
        input_directory.push(r"Inphonik\RX1200\Collections\Factory Collection");

        let root_folder = env::var("HOMEDRIVE").expect("Failed to get drive root folder");
        let output_directory = env::current_dir().unwrap_or_else(|_| PathBuf::from(root_folder));

        Configuration {
            input_directory,
            output_directory,
        }
    }
}

impl Configuration {
    pub fn load(file_path: &str) -> Self {
        match fs::read_to_string(file_path) {
            Ok(s) => toml::from_str(&s).expect("Failed to parse config file"),
            Err(_) => Configuration::default(),
        }
    }

    pub fn save(&self, file_path: &str) {
        let toml_string = toml::to_string(self).expect("Failed to serialize config");
        fs::write(file_path, toml_string).expect("Failed to write config file");
    }
}

pub fn run_configuration() -> anyhow::Result<(PathBuf, PathBuf)> {
    let config_file_name = "config.toml";
    // fs::remove_file(config_file_name);

    let mut configuration = Configuration::load(config_file_name);

    if let Some(path) = cli::directory_selector(
        "Enter input directory:",
        &configuration.input_directory,
        false,
    ) {
        configuration.input_directory = path;
    }
    println!();

    if let Some(path) = cli::directory_selector(
        "Enter output directory:",
        &configuration.output_directory,
        true,
    ) {
        configuration.output_directory = path;
    }
    println!();

    configuration.save(config_file_name);

    Ok((
        configuration.input_directory,
        configuration.output_directory,
    ))
}
