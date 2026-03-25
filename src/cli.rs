use std::default;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use inquire::Select;
use quick_xml::se;
use serde::{Serialize, Deserialize};


// pub fn directory_selector(mut directory: impl AsRef<PathBuf>) -> Option<PathBuf> {
pub fn directory_selector(mut directory: &PathBuf) -> Option<PathBuf> {
    // let mut current_directory = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut current_directory = PathBuf::from(directory);

    let confirm_option = "✓ Confirm";
    let parent_option = "📂 ..";

    loop {
        let mut options = vec![confirm_option.to_string()];

        if current_directory.parent().is_some() {
            options.push(parent_option.to_string());
        }

        if let Ok(entries) = fs::read_dir(&current_directory) {
            let mut directories: Vec<String> = entries
                .flatten()
                .filter(|e| {e.path().is_dir()}) // Keep only directories
                .filter(|e| {!e.file_name().to_string_lossy().starts_with('.')})
                .map(|e| format!("📂 {}", e.file_name().to_string_lossy()))
                .collect();
            
            directories.sort();
            options.extend(directories);
        }

        let prompt_message = format!("{}", current_directory.display());
        
        let choice = Select::new(&prompt_message, options).prompt();

        match choice {
            Ok(selection) => {
                if selection == confirm_option {
                    return Some(current_directory)
                } else if selection == parent_option {
                    current_directory.pop();
                } else {
                    current_directory.push(selection.strip_prefix("📂 ").unwrap());
                }
            }
            Err(_) => {
                println!("some problem?");
                return None;
            }
        }
    };
}

pub fn collect_files(directory_path: impl AsRef<Path>) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let directory = match fs::read_dir(directory_path) {
        Ok(dir) => dir,
        Err(e) => {
            println!("Bad directory! {e}");
            return files;
        }
    };

    for file in directory {
        // Check file:
        let file = match file {
            Ok(f) => f,
            Err(e) => {
                println!("Bad file! {e}");
                continue;
            }
        };

        let file_path = file.path();

        // Make sure it's not a directory:
        if !file_path.is_file() {continue}

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
        Configuration {
            input_directory: PathBuf::from(r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\Factory Collection"),
            output_directory: env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
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

use inquire::autocompletion::{Autocomplete, Replacement};
use inquire::{CustomUserError, Text};

#[derive(Clone, Default)]
pub struct FilePathCompleter;

impl Autocomplete for FilePathCompleter {
    // Returns a list of strings to show in the dropdown/list
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        let path = Path::new(input);
        
        // Determine the directory to search and the partial name (stub)
        let (scan_path, stub) = if input.ends_with('\\') || input.is_empty() {
            (input, "")
        } else {
            let parent = path.parent().and_then(|p| p.to_str()).unwrap_or("");
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            (parent, file_name)
        };

        let mut entries = Vec::<String>::new(); 

        if scan_path.is_empty() { return Ok(entries) }

        let stub = stub.to_lowercase();
        
        // Read directory and filter for matches
        entries = fs::read_dir(scan_path)
            .map_err(|e| CustomUserError::from(e))?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let name = entry.file_name().to_string_lossy().into_owned();
                if name.to_lowercase().starts_with(&stub) && entry.file_type().ok()?.is_dir() {
                    Some(format!("{}{}{}", scan_path, if scan_path.is_empty() || scan_path.ends_with('\\') { "" } else { "\\" }, name))
                } else {
                    None
                }
            })
            .collect();

        Ok(entries)
    }

    // Defines what happens when the user presses "Tab"
    // fn get_completion(&mut self, input: &str, highlighted: Option<String>) -> Result<Replacement, CustomUserError> {
    //     Ok(highlighted.map(Replacement::Some).unwrap_or(Replacement::None))
    // }
    fn get_completion(&mut self, _input: &str, highlighted: Option<String>) -> Result<Replacement, CustomUserError> {
        match highlighted {
            Some(mut selection) => {
                // Check if the selection is a directory to append the slash
                if std::path::Path::new(&selection).is_dir() && !selection.ends_with('\\') {
                    selection.push('\\');
                }
                Ok(Replacement::Some(selection))
            }
            None => Ok(Replacement::None),
        }
    }
}


pub fn dsel(p: &PathBuf) {
    let autocompleter = FilePathCompleter::default();

    let answer = Text::new("Enter directory path:")
        .with_autocomplete(autocompleter)
        .with_initial_value(p.to_str().unwrap())
        // .with_help_message("↑↓ to move, tab to autocomplete, enter to submit")
        .prompt();

    match answer {
        Ok(path) => println!("Selected path: {}", path),
        Err(_) => println!("Prompt cancelled or error occurred."),
    }
}
