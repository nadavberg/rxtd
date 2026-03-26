use std::env;
use std::fs;
use std::path::{Path, PathBuf, MAIN_SEPARATOR, MAIN_SEPARATOR_STR};
// use inquire::Select;
use serde::{Serialize, Deserialize};

pub fn collect_rx_files(directory_path: impl AsRef<Path>) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let directory = match fs::read_dir(directory_path) {
        Ok(dir) => dir,
        Err(e) => {
            println!("Bad directory! {e}");
            return files;
        }
    };

    for entry in directory {
        // Check file:
        let file = match entry {
            Ok(dir_entry) => dir_entry,
            Err(error) => {
                println!("Bad file! {error}");
                continue;
            }
        };

        let file_path = file.path();

        if !file_path.is_file() {continue} // exclude directories

        // Check extension:
        let has_rx_extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("rx1200"))
            .unwrap_or(false);
        if !has_rx_extension {continue}

        files.push(file_path);
    }
    files
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration{
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
 
        Configuration { input_directory, output_directory, }
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

use inquire::autocompletion::{Autocomplete, Replacement};
use inquire::{CustomUserError, Text};

#[derive(Clone, Default)]
pub struct FilePathCompleter;

impl Autocomplete for FilePathCompleter {
    // Returns a list of strings to show in the dropdown/list
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        let path = PathBuf::from(input);
        
        let mut entries = Vec::<String>::new(); 
        
        if input.ends_with(MAIN_SEPARATOR) && !path.is_dir() { return Ok(entries) }

        
        let (scan_path, stub) = if input.ends_with(MAIN_SEPARATOR) || input.is_empty() {
            (input, "")
        } else {
            let parent = path.parent().and_then(|p| p.to_str()).unwrap_or("");
            let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            (parent, file_name)
        };

        if scan_path.is_empty() { return Ok(entries) }

        let stub_lowercase = stub.to_lowercase();
        
        // Collect subdirectories:
        entries = fs::read_dir(scan_path)
            .map_err(|e| CustomUserError::from(e))?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let name = entry.file_name().to_string_lossy().into_owned();
                let name_lowercase = name.to_lowercase();
                let is_exact_match = name_lowercase == stub_lowercase;
                let is_partial_match = name_lowercase.starts_with(&stub_lowercase);
                // let is_directory = entry.file_type().ok()?.is_dir();
                let is_directory = entry.path().is_dir();
                if !is_exact_match && is_partial_match && is_directory {
                    let separator = if scan_path.ends_with(MAIN_SEPARATOR) { "" } else { MAIN_SEPARATOR_STR };
                    Some(format!("{scan_path}{separator}{name}"))
                } else {
                    None
                }
            })
            .collect();

        Ok(entries)
    }

    fn get_completion(&mut self, _input: &str, highlighted: Option<String>) -> Result<Replacement, CustomUserError> {
        match highlighted {
            Some(mut selection) => {
                // Check if the selection is a directory to append the slash
                if std::path::Path::new(&selection).is_dir() && !selection.ends_with(MAIN_SEPARATOR) {
                    selection.push(MAIN_SEPARATOR);
                }
                Ok(Replacement::Some(selection))
            }
            None => Ok(Replacement::None),
        }
    }
}

pub fn directory_selector(message: &str, initial_path: &Path) -> Option<PathBuf> {
    loop {
        let answer = Text::new(message)
            .with_autocomplete(FilePathCompleter::default())
            .with_initial_value(initial_path.to_str().unwrap())
            // .with_help_message("↑↓ to move, tab to autocomplete, enter to submit")
            .prompt();
    
        match answer {
            Ok(p) => {
                let path = if let Some(s) = p.strip_suffix(MAIN_SEPARATOR) {s} else {&p};
                let path = PathBuf::from(path);
                if path.is_dir() { return Some(path); }
                println!("That ain't no directory! Try again...");
            },
            Err(e) => {
                println!("Problemo!");
                println!("{e}");
                println!("Try again...");
            },
        }
    }
}

pub fn run_configuration() -> (PathBuf, PathBuf) {
    let config_file_name = "config.ini";
    // fs::remove_file(config_file_name);
    
    let mut configuration = Configuration::load(config_file_name);
    
    if let Some(path) = directory_selector("Enter input directory:", &configuration.input_directory) {
        configuration.input_directory = path;
    }
    
    if let Some(path) = directory_selector("Enter output directory:", &configuration.output_directory) {
        configuration.output_directory = path;
    }
           
    configuration.save(config_file_name);

    (configuration.input_directory, configuration.output_directory)
}
