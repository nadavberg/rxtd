#![allow(warnings, unused)]

use rxtd::*;

// use serde::Deserialize;
// use serde::Serialize;

use quick_xml::de::from_str;
use quick_xml::se::to_string;
// use quick_xml::events::{Event, BytesDecl};
// use quick_xml::Writer;

use std::fs;
// use std::io::{BufWriter, Write};


fn main() {

    // let path = "AliveandKickin.rx1200";
    // let xml = fs::read_to_string(path).expect("Didn't work");
    
    // let xml_data = include_str!(r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\Factory Collection\Alive and Kickin.rx1200");
    // let xml_data = include_str!(r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\User Collection\a.rx1200");
    // let xml_data = include_str!(r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\User Collection\Alive and Kickinz.rx1200");
    let xml_data = include_str!(r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\Factory Collection\Young Blood 808.rx1200");
    // let xml_data = include_str!(r"C:\Users\Nadav\AppData\Roaming\Inphonik\RX1200\Collections\Templates Collection\All clear.rx1200");

    // Deserialize XML:
    let rx_preset: RxPreset = from_str(xml_data).expect("Failed to parse RX1200 preset");
    
    // process and sort tags into intermediate struct:
    let mut intermediate_preset = IntermediatePreset::new();
    for tag in rx_preset.tags {
        match tag {
            RxTag::Param(p) => process_param(&p, &mut intermediate_preset),
            RxTag::Samples(s) => process_samples_container(&s, &mut intermediate_preset),
            RxTag::Gui(g) => process_gui_container(&g, &mut intermediate_preset),
        }
    }
    intermediate_preset.assign_midi_keys();


    let mut td_pads = TdPads { items: Vec::new() };
    let mut pad_count: u8 = 0;
    let td_velocity = rx_velocity_to_td_velocity(intermediate_preset.velocity);
    let (td_master_volume, td_pad_volume_adjustment) = rx_master_volume_to_td_master_volume(intermediate_preset.volume);
    for pad in intermediate_preset.pads {
        if pad.inactive {continue}
        // println!("{pad:?}\n");
        let (td_tune, td_finetune) = rx_pitch_speed_finetune_to_td_tune_finetune(pad.pitch, pad.speed, pad.finetune);
        let td_path = if pad.factory_content {r"C:/ProgramData/Inphonik/RX1200".to_owned() + pad.sample_path.as_str()} else {pad.sample_path};
        let td_pad = TdPad {
            colour: rx_color_to_td_color(pad.color),
            volume: rx_level_and_gain_to_td_volume(pad.level, pad.gain),
            pan: pad.pan,
            midikey: pad.midikey,
            mappings: TdMappings {
                mapping:TdMapping {
                    path: td_path,
                    tune: td_tune,
                    finetune: td_finetune,
                    volume: td_pad_volume_adjustment,
                    velocityintensity: td_velocity,
                }
            }
        };
        td_pads.items.push(td_pad);
        pad_count += 1;
    }

    let td_preset = TdPreset {
        version: 13,
        name: rx_preset.name,
        volume: td_master_volume,
        pads: td_pads,
    };

    let raw_xml = to_string(&td_preset).expect("Failed to serialize preset");
    let final_xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", raw_xml);
    let td_file_path = td_preset.name + ".taldrum";
    fs::write(td_file_path, final_xml).expect("Failed to write file");
}
