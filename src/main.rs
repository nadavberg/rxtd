#![allow(warnings, unused)]

use rxtd::*;

// use quick_xml::events::{Event, BytesDecl};
// use quick_xml::Writer;

use std::fs;
// use std::io::{BufWriter, Write};


fn main() {

    // let path = "AliveandKickin.rx1200";
    // let xml = fs::read_to_string(path).expect("Didn't work");
    
    // let path = r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\Factory Collection\Alive and Kickin.rx1200";
    
    // let xml_data = include_str!(r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\Factory Collection\Alive and Kickin.rx1200");
    // let xml_data = include_str!(r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\Factory Collection\Brighton.rx1200");
    // let xml_data = include_str!(r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\Factory Collection\Young Blood 808.rx1200");
    // let xml_data = include_str!(r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\User Collection\a.rx1200");
    // let xml_data = include_str!(r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\User Collection\Alive and Kickinz.rx1200");
    let xml_data = include_str!(r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\User Collection\One Sine.rx1200");
    // let xml_data = include_str!(r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\Templates Collection\All clear.rx1200");

    // Deserialize XML:
    let rx_preset: RxPreset = quick_xml::de::from_str(xml_data).expect("Failed to parse RX1200 preset");
    
    let intermediate_preset = build_intermediate_preset(rx_preset);

    let td_preset = build_td_preset(intermediate_preset);

    let raw_xml = quick_xml::se::to_string(&td_preset).expect("Failed to serialize preset");
    let final_xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", raw_xml);
    let td_file_path = td_preset.name + ".taldrum";
    fs::write(td_file_path, final_xml).expect("Failed to write file");
}
