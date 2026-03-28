// #![allow(warnings, unused)]

pub mod cli;
pub mod config;
pub mod rx;
pub mod td;
pub mod intermediate;

use std::path::{Path, PathBuf};
use std::fs;
// use quick_xml::se;

pub fn collect_rx_files(directory_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(directory_path)? {
        let file_path = entry?.path();
        if file_path.is_file() {
            let has_rx_extension = file_path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("rx1200"));
            if has_rx_extension { files.push(file_path) }
        }
    }
    Ok(files)
}

pub fn convert_preset(rx_file: &Path, output_directory: &Path) -> anyhow::Result<()> {
        let file_name = rx_file.file_stem().expect("Failed to parse file name");
        print!("   Converting \"{}.rx1200\"... ", file_name.display());
        let rx_preset = rx::Preset::load_from_file(rx_file)?;
        let intermediate = intermediate::Preset::from(rx_preset);
        let td_preset = td::Preset::from(intermediate);
        td_preset.save_to_file(output_directory, file_name)?;
        println!("Success!");
        Ok(())
}