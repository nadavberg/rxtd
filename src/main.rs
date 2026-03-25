#![allow(warnings, unused)]

// use std::env;
use std::fs;
use std::path::{Path, PathBuf};
// use inquire::Select;
use quick_xml::{de, se};
use rxtd::*;

mod cli;

use inquire::{CustomUserError, Text};

use crate::cli::dsel;

fn main() {
    println!();
    
    let config_file_name = "config.ini";
    let mut configuration = cli::Configuration::load(config_file_name);
    
    dsel(&configuration.input_directory);

    // println!("Select input directory:");
    // if let Some(path) = cli::directory_selector(&configuration.input_directory) {
    //     configuration.input_directory = path
    // }

    // println!("Select output directory:");
    // if let Some(path) = cli::directory_selector(&configuration.output_directory) {
    //     configuration.output_directory = path
    // }


    configuration.save(config_file_name);
    /*
    let directory_path = r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\User Collection";

    let rx_files = collect_files(directory_path);

    println!("Found {} RX1200 presets in {directory_path}", rx_files.len());

    for rx_file in rx_files {
        let file_name = rx_file
        .file_stem()
        .and_then(|name| name.to_str())
        .expect("Failed to extract file name");
    print!("Converting \"{file_name}.rx1200\"... ");
    let rx_xml = fs::read_to_string(&rx_file).expect("Couldn't read source file");
    let rx_preset: RxPreset = de::from_str(&rx_xml).expect("Failed to parse preset");
    let intermediate_preset = build_intermediate_preset(rx_preset);
    let td_preset = build_td_preset(intermediate_preset);
    let td_xml = se::to_string(&td_preset).expect("Failed to serialize preset");
    let td_xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", td_xml);
    let td_file_path = format!("converted_presets\\{file_name}.taldrum");
    fs::write(&td_file_path, td_xml).expect("Failed to write file");
    println!("Success!");
    }
    */
    println!();
}
