// #![allow(warnings, unused)]

use rxtd::*;
use std::fs;
use std::path::{Path, PathBuf};
use quick_xml::{de, se};


fn get_files_in_directory(directory_path: impl AsRef<Path>) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let directory = match fs::read_dir(directory_path) {
        Ok(dir) => dir,
        Err(i) => {
            println!("Bad directory! {i}");
            return files;
        }
    };

    for file in directory {
        // Check file:
        let file = match file {
            Ok(f) => f,
            Err(_) => {
                println!("Bad file!");
                continue;
            }
        };

        // Make sure it's not a directory:
        let file_path = file.path();
        if !file_path.is_file() {
            continue;
        }

        // Check extension:
        let has_rx_extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("rx1200"))
            .unwrap_or(false);
        if !has_rx_extension {
            continue;
        }

        files.push(file_path);
    }
    files
}

fn main() {
    println!();

    let directory_path = r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\User Collection";

    let rx_files = get_files_in_directory(directory_path);

    println!("Found {} RX1200 presets:", rx_files.len());

    for rx_file in rx_files {
        let file_name = match rx_file.file_stem().and_then(|name| name.to_str()) {
            Some(name) => name,
            None => continue,
        };
        let rx_xml = fs::read_to_string(&rx_file).expect("Couldn't read source file");
        let rx_preset: RxPreset = de::from_str(&rx_xml).expect("Failed to parse preset");
        let intermediate_preset = build_intermediate_preset(rx_preset);
        let td_preset = build_td_preset(intermediate_preset);
        let td_xml = se::to_string(&td_preset).expect("Failed to serialize preset");
        let td_xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", td_xml);
        let td_file_path = format!("converted_presets\\{file_name}.taldrum");
        fs::write(&td_file_path, td_xml).expect("Failed to write file");
        println!("{}, ", td_file_path);
    }
    println!();
}
