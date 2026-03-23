use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use inquire::Select;
use quick_xml::se;



pub fn directory_selector() -> Option<PathBuf> {
    let mut current_directory = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

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
