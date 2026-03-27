#![allow(warnings, unused)]

// use std::env;
use std::fs;
use std::path::Path;
use quick_xml::{de, se};
use rxtd::*;
mod cli;

fn main() {
    println!();

    let (input_directory, output_directory) = cli::run_configuration();
    
    let rx_files = cli::collect_rx_files(&input_directory);
    
    println!("Found {} RX1200 presets 😎\nLet's go!", rx_files.len());

    for rx_file in rx_files {
        if let Err(e) = convert_preset(&rx_file, &output_directory) {
            eprintln!("Failed to convert {}: {e}", rx_file.display());
        }
    }

    println!("Done!\nEnjoy the rest of your day 🥰");
    println!();
}

fn convert_preset(rx_file: &Path, output_directory: &Path) -> anyhow::Result<()> {
        let file_name = rx_file.file_stem().expect("Failed to parse file name");
        print!("   Converting \"{}.rx1200\"... ", file_name.display());
        let rx_xml = fs::read_to_string(&rx_file)?;
        let rx_preset: RxPreset = de::from_str(&rx_xml)?;
        let intermediate_preset = build_intermediate_preset(rx_preset);
        let td_preset = build_td_preset(intermediate_preset);
        let td_xml = se::to_string(&td_preset)?;
        // let td_xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", td_xml);
        
        let mut td_file_path = output_directory.to_path_buf();
        td_file_path.push(file_name);
        td_file_path.set_extension("taldrum");
        
        fs::write(td_file_path, td_xml)?;
        println!("Success!");
        // println!("{:>20}", "Success!");
        Ok(())
}