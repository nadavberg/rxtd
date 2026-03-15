#![allow(warnings, unused)]

use rxtd::*;
use std::ffi::OsStr;
// use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

// use quick_xml::events::{Event, BytesDecl};
// use quick_xml::Writer;
// use std::io::{BufWriter, Write};

#[derive(Debug)]
struct RxFile {
    path: std::path::PathBuf,
    name: String,
}

// fn get_files_in_directory(directory_path: &str) -> Vec<RxFile> {
fn get_files_in_directory(directory_path: &str) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();

    let directory = match std::fs::read_dir(directory_path) {
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

    for file in rx_files {
        let file_name = match file.file_stem().and_then(|name| name.to_str()) {
            Some(name) => name,
            None => continue,
        };
        print!("{}, ", file_name);
    }
    println!();

    // let file_path = r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\Factory Collection\Alive and Kickin.rx1200";
    // let file_path = r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\Factory Collection\Brighton.rx1200";
    // let file_path = r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\Factory Collection\Young Blood 808.rx1200";

    // let file_path = r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\User Collection\a.rx1200";
    // let file_path = r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\User Collection\Alive and Kickinz.rx1200";
    // let file_path = r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\User Collection\One Sine.rx1200";
    // let file_path = r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\User Collection\stere0.rx1200";
    // let file_path = r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\User Collection\sine_pitch.rx1200";
    // let file_path = r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\User Collection\Beat.rx1200";
    let file_path = r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\User Collection\Loops.rx1200";

    // let file_path = r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\Templates Collection\All clear.rx1200";

    let xml_data = std::fs::read_to_string(file_path).expect("Couldn't read source file");

    let file_name = file_path
        .rsplit_once('\\')
        .unwrap()
        .1
        .rsplit_once('.')
        .unwrap()
        .0
        .to_string();

    let rx_preset: RxPreset =
        quick_xml::de::from_str(&xml_data).expect("Failed to parse RX1200 preset");

    let intermediate_preset = build_intermediate_preset(rx_preset);

    let td_preset = build_td_preset(intermediate_preset);

    let raw_xml = quick_xml::se::to_string(&td_preset).expect("Failed to serialize preset");
    let final_xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", raw_xml);

    let mut td_file_path = "converted_presets\\".to_string();
    td_file_path.push_str(&file_name);
    td_file_path.push_str(".taldrum");
    // let td_file_path = file_name + ".taldrum";

    std::fs::write(&td_file_path, final_xml).expect("Failed to write file");
    println!("{file_name}\n");
}
