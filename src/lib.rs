#![allow(warnings, unused)]

pub mod cli;
pub mod config;
pub mod intermediate;
pub mod rx;
pub mod td;

use std::fs;
use std::path::{Path, PathBuf};
use colored::Colorize;

pub fn collect_rx_files(directory_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let files: Vec<PathBuf> = fs::read_dir(directory_path)?
        .filter_map(Result::ok) // unpack directory entry
        .filter(|entry| {
            entry.file_type().is_ok_and(|file_type| file_type.is_file()) // exclude non-files
        })
        .map(|entry| entry.path()) // convert DirEntry to PathBuf
        .filter(|path| {
            path.extension().is_some_and(|x| x.eq_ignore_ascii_case("rx1200"))  // check for rx1200 extension
        })
        .collect();
    Ok(files)
}

pub fn convert_preset(rx_file: &Path, output_directory: &Path) -> anyhow::Result<()> {
    let file_name = rx_file.file_stem().expect("Failed to parse file name");
    
    let name_str = format!("{}.rx1200", file_name.display());
    let dots_str = ".".repeat(32 - name_str.len());
    print!("    Converting {} {dots_str} ", name_str.blue());
    
    let rx_preset = rx::Preset::load_from_file(rx_file)?;
    let intermediate = intermediate::Preset::from(rx_preset);
    let td_preset = td::Preset::from(intermediate);

    td_preset.save_to_file(output_directory, file_name)?;
    println!("{}", "Great success!".green().bold());
    Ok(())
}

pub fn get_extension(path: &Path) -> &str {
    path.extension().and_then(|x| x.to_str()).unwrap_or("")
}
