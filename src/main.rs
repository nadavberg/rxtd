#![allow(warnings, unused)]

use std::fs;
use std::path::{Path, PathBuf};
use quick_xml::{de, se};

use rxtd::*;
mod config;
mod cli;

fn main() {
    let title = "
██████╗ ████████╗██╗  ██╗██████╗      ██████╗ ██████╗ ███╗   ██╗██╗   ██╗███████╗██████╗ ████████╗
██╔══██╗╚══██╔══╝╚██╗██╔╝██╔══██╗    ██╔════╝██╔═══██╗████╗  ██║██║   ██║██╔════╝██╔══██╗╚══██╔══╝
██████╔╝   ██║    ╚███╔╝ ██║  ██║    ██║     ██║   ██║██╔██╗ ██║██║   ██║█████╗  ██████╔╝   ██║   
██╔══██╗   ██║    ██╔██╗ ██║  ██║    ██║     ██║   ██║██║╚██╗██║╚██╗ ██╔╝██╔══╝  ██╔══██╗   ██║   
██║  ██║   ██║   ██╔╝ ██╗██████╔╝    ╚██████╗╚██████╔╝██║ ╚████║ ╚████╔╝ ███████╗██║  ██║   ██║   
╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═════╝      ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝   ╚═╝   
";
    println!("{title}");

    let (input_directory, output_directory) = config::run_configuration();
    
    let rx_files = collect_rx_files(&input_directory);
    
    let number_of_presets = rx_files.len();

    if number_of_presets > 0 {
        println!("Found {number_of_presets} RX1200 presets 😎");
        println!("Let's go!");
        for rx_file in rx_files {
            if let Err(e) = convert_preset(&rx_file, &output_directory) {
                eprintln!("Failed to convert {}: {e}", rx_file.display());
            }
        }
        println!("Done!");
    } else {
        println!("No RX1200 presets found in input directory 😮");
    }

    println!("Enjoy the rest of your day 🥰");
    println!();
}

// pub fn collect_rx_files(directory_path: impl AsRef<Path>) -> Vec<PathBuf> {
pub fn collect_rx_files(directory_path: &Path) -> Vec<PathBuf> {
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

        if file_path.is_file() {
            let has_rx_extension = file_path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("rx1200"));
            if has_rx_extension {
                files.push(file_path);
            }
        }
    }
    files
}

fn convert_preset(rx_file: &Path, output_directory: &Path) -> anyhow::Result<()> {
        let file_name = rx_file.file_stem().expect("Failed to parse file name");
        print!("   Converting \"{}.rx1200\"... ", file_name.display());
        let rx_xml = fs::read_to_string(&rx_file)?;
        let rx_preset: RxPreset = de::from_str(&rx_xml)?;
        let intermediate_preset = build_intermediate_preset(rx_preset);
        let td_preset = build_td_preset(intermediate_preset);
        let td_xml = se::to_string(&td_preset)?;
        
        let mut td_file_path = output_directory.to_path_buf();
        td_file_path.push(file_name);
        td_file_path.set_extension("taldrum");
        
        fs::write(td_file_path, td_xml)?;
        println!("Success!");
        Ok(())
}