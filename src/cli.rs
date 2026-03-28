use std::fs;
use std::path::{Path, PathBuf, MAIN_SEPARATOR, MAIN_SEPARATOR_STR};
use inquire::Select;
use inquire::autocompletion::{Autocomplete, Replacement};
use inquire::{CustomUserError, Text};
use inquire::ui::{Color, RenderConfig, StyleSheet, Styled, Attributes};

#[derive(Clone, Default)]
pub struct FilePathCompleter;

impl Autocomplete for FilePathCompleter {
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
            // .map_err(|e| CustomUserError::from(e))?
            .map_err(CustomUserError::from)?
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

fn make_render_config1() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::empty();
    render_config.prompt_prefix = Styled::new("");
    render_config.answered_prompt_prefix = Styled::new("");
    render_config.prompt = StyleSheet::new().with_fg(Color::LightGreen).with_attr(Attributes::BOLD);
    render_config.text_input = StyleSheet::new().with_fg(Color::DarkGreen);
    render_config.answer = StyleSheet::new().with_fg(Color::DarkBlue);
    render_config.option = StyleSheet::new().with_fg(Color::DarkGrey);
    render_config.selected_option = Some(StyleSheet::new().with_fg(Color::DarkGrey).with_attr(Attributes::BOLD));
    render_config.help_message = StyleSheet::new().with_fg(Color::DarkBlue);
    render_config.highlighted_option_prefix = Styled::new(">").with_fg(Color::DarkBlue);
    render_config
}

fn make_render_config2() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::empty();
    render_config.prompt_prefix = Styled::new("");
    render_config.answered_prompt_prefix = Styled::new("");
    render_config.prompt = StyleSheet::new().with_fg(Color::DarkBlue);
    render_config.answer = StyleSheet::new().with_fg(Color::DarkBlue);
    render_config.selected_option = Some(StyleSheet::new().with_attr(Attributes::BOLD));
    render_config.help_message = StyleSheet::new().with_fg(Color::DarkBlue);
    render_config.highlighted_option_prefix = Styled::new(">").with_fg(Color::DarkBlue);
    render_config
}

pub fn directory_selector(message: &str, initial_path: &Path, allow_creation: bool) -> Option<PathBuf> {

    let render_config = make_render_config1();
    
    let mut initial_path = initial_path.to_str()?;
    if initial_path.starts_with("\\\\?\\") {
        initial_path = initial_path.strip_prefix("\\\\?\\").unwrap();
    }

    loop {
        let answer = Text::new(message)
            .with_autocomplete(FilePathCompleter)
            .with_initial_value(initial_path)
            .with_render_config(render_config)
            .prompt();
    
        match answer {
            Ok(path) => {
                let path = PathBuf::from(path);
                if path.is_dir() { return Some(path.canonicalize().unwrap()); }

                if allow_creation {
                    println!("That's not an existing directory...");
                    let message = "Select an option:";
                    let options = vec![
                        format!("Create new directory \"{}\"", path.display()),
                        "Select different directory".to_string(),
                    ];
                    let render_config = make_render_config2();
                    if let Ok(a) = Select::new(message, options).with_render_config(render_config).without_filtering().without_help_message().prompt() {
                        if a == "Select different directory" {
                            println!("Ok let's try again:")
                        } else if fs::create_dir(&path).is_ok() {
                            println!("Successfully created \"{}\"", path.display());
                            return Some(path);
                        } else {
                            println!("Could not create \"{}\", try again...", path.display());
                        }
                    }
                } else {
                    println!("That's not an existing directory, try again...");
                }
            },
            Err(e) => {
                println!("Error: {e}");
                println!("Try again...");
            },
        }
    }
}
