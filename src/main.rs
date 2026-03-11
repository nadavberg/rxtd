#![allow(warnings, unused)]

// // use std::fs;
use serde::Deserialize;
use serde::Serialize;

use quick_xml::de::from_str;
use quick_xml::se::to_string;
use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;

use std::fs;
// use std::intrinsics::powf64;
use std::io::{BufWriter, Write};


// Root Struct
#[derive(Debug, Deserialize)]
struct RxPreset {
    #[serde(rename = "@name")]
    name: String,
    
    // #[serde(rename = "@author")]
    // author: Option<String>,
    
    // #[serde(rename = "@comment")]
    // comment: Option<String>,
    
    #[serde(rename = "$value")]
    tags: Vec<RxTag>,
}

// Enum to Catch Interleaved Tags:
#[derive(Debug, Deserialize)]
enum RxTag {
    #[serde(rename = "PARAM")]
    Param(Param),
    
    #[serde(rename = "SAMPLES")]
    Samples(Samples),
    
    #[serde(rename = "GUI")]
    Gui(Gui),
}

// Generic Param Tag:
#[derive(Debug, Deserialize)]
struct Param {
    #[serde(rename = "@id")]
    id: String,
    
    #[serde(rename = "@value")]
    value: Option<f64>, 
}

// Samples Container:
#[derive(Debug, Deserialize)]
struct Samples {
    #[serde(rename = "SAMPLE", default)]
    items: Vec<Sample>,
}

// Individual Sample:
#[derive(Debug, Deserialize)]
struct Sample {
    #[serde(rename = "@id")]
    id: String,
    
    #[serde(rename = "@reversed")]
    reversed: bool,
    
    #[serde(rename = "@gain")]
    gain: f64,
    
    #[serde(rename = "@start")]
    start: u64,
    
    #[serde(rename = "@end")]
    end: u64,
    
    #[serde(rename = "REFERENCES")]
    references: Option<References>,
}

// References Container:
#[derive(Debug, Deserialize)]
struct References {
    #[serde(rename = "REFERENCE", default)]
    items: Vec<Reference>,
}

// Individual Reference:
#[derive(Debug, Deserialize)]
struct Reference {
    #[serde(rename = "@type")]
    ref_type: String,
    
    #[serde(rename = "@value")]
    value: String,
}

// GUI Container:
#[derive(Debug, Deserialize)]
struct Gui {
    #[serde(rename = "PARAM", default)]
    params: Vec<Param>, 
}

fn pad_id_to_index(pad_id: &str) -> usize {
    let bytes = pad_id.as_bytes();
    let bank = (bytes[0] - b'a') as usize;
    let pad = (bytes[1] - b'1') as usize;
    8 * bank + pad
}

fn process_param(param: &Param, intermediate: &mut IntermediatePreset) {

    if param.value.is_none() { return; }
    let value = param.value.unwrap();

    let param_name: &str;

    if (!param.id.contains('_')) {
        param_name = &param.id;
        match param_name {
            "volume" => intermediate.volume = value,
            "velocity" => intermediate.velocity = value,
            "layout" => intermediate.layout = (value != 0.0) as bool,
            _ => (),
        }
        return;
    }

    let (a, b) = param.id.split_once('_').expect("foo..."); // e.g. "A_B_C" -> "A", "B_C"
                
    // Polyphony:
    if b.len() == 1 {
        let index = (b.as_bytes()[0] - b'1') as usize;;
        intermediate.polyphony[index] = value as u8;
        return;
    }
    
    let pad_id: &str;

    if a.len() == 2 {
        pad_id = a;
        param_name = b;
    } else {
        let (a, b) = param.id.split_once('_').expect("foo..."); // e.g. "A_B_C" -> "A_B", "C"
        pad_id = b;
        param_name = a;
    }

    let pad_index: usize = pad_id_to_index(pad_id);

    match param_name {
        "pitch" => intermediate.pads[pad_index].pitch = (15.0 * value).round() as i8 - 8,
        "decay" => intermediate.pads[pad_index].decay = value,
        // "level" => intermediate.pads[pad_index].level = value,
        "level" => intermediate.pads[pad_index].level = (15.0 * value).round() as u8,
        "pan" => intermediate.pads[pad_index].pan = value,
        "pad" => intermediate.pads[pad_index].pad = value,
        "output" => intermediate.pads[pad_index].output = value as u8,
        "filter" => intermediate.pads[pad_index].filter = value as u8,
        "finetune" => intermediate.pads[pad_index].finetune = value,
        "gain" => intermediate.pads[pad_index].gain = value,
        "mono" => intermediate.pads[pad_index].mono = value as u8,
        "speed" => intermediate.pads[pad_index].speed = value as u8,
        "loop_mode" => intermediate.pads[pad_index].loop_mode = value as u8,
        "loop_range_end" => intermediate.pads[pad_index].loop_range_end = value,
        "loop_range_start" => intermediate.pads[pad_index].loop_range_start = value,
        "play_range_end" => intermediate.pads[pad_index].play_range_end = value,
        "play_range_start" => intermediate.pads[pad_index].play_range_start = value,
        _ => ()
    }
}

fn process_samples_container(samples: &Samples, intermediate: &mut IntermediatePreset) {
    for s in samples.items.iter() {
        let pad_index: usize = pad_id_to_index(&s.id);
        if intermediate.pads[pad_index].inactive { continue; }
        if s.references.is_none() {
            intermediate.pads[pad_index].inactive = true;
            continue;
        }
        let references = s.references.as_ref().unwrap();
        let reference = &references.items.first();
        if reference.is_none() {
            intermediate.pads[pad_index].inactive = true;
            continue;
        }
        let reference = reference.unwrap();
        // println!("{reference:?}");
        intermediate.pads[pad_index].sample_path = reference.value.clone();

    }
}

fn process_gui_container(gui: &Gui, intermediate: &mut IntermediatePreset) {
    for g in gui.params.iter() {
        if g.value.is_none() { continue }
        let value = g.value.unwrap();
        if (!g.id.contains('_')) {
            match g.id.as_str() {
                "bank" => intermediate.bank = value,
                "mode" => intermediate.mode = value,
                _ => (),
            }
            continue;
        }

        let (_, pad_id) = g.id.split_once('_').expect("foo...");
        let pad_index: usize = pad_id_to_index(pad_id);
        intermediate.pads[pad_index].color = (value * 7.0).round() as u8;
    }
}

fn rx_color_to_td_color(color: u8) -> i32 {
    match color {
        0 => return -13262337, // #35A1FF
        1 => return -8099340, // #8469F4
        2 => return -48223, // #FF43A1
        3 => return -38559, // #FF6961
        4 => return -19328, // #FFB480
        5 => return -461939, // #F8F38D
        6 => return -12396892, // #42D6A4
        7 => return -1710619, // #E5E5E5
        _ => -5631463, // #AA1219 (default TD pad color)
    }
}

fn rx_level_and_gain_to_td_volume(level: u8, gain: f64) -> f64 {
    let mut level: f64 = match level {
        00 => -48.125540454678699,
        01 => -38.583116391055221,
        02 => -34.146140574112714,
        03 => -31.223580096152403,
        04 => -29.040687631705044,
        05 => -25.846673804992303,
        06 => -23.516561117611591,
        07 => -20.890984765096807,
        08 => -18.298307076753225,
        09 => -15.869863144299639,
        10 => -13.161780356313614,
        11 => -10.395725911069878,
        12 => -7.7017553469803106,
        13 => -5.2652451244176559,
        14 => -2.5963049656793182,
        15 => 0.0052625759604806987,
        _  => 0.0,
    };
    level = f64::powf(10.0, level * 0.05);
    0.5 * f64::sqrt(level * 10.0 * gain)
}



#[derive(Debug)]
struct IntermediatePreset {
    polyphony: [u8; 8],
    volume: f64,
    velocity: f64,
    layout: bool,
    bank: f64,
    mode: f64,
    pads: [IntermediatePad; 32],
}

// defaults based on "All clear.rx1200"
impl IntermediatePreset {
    fn new() -> Self {
        let pads_array: [IntermediatePad; 32] = std::array::from_fn(|i| {
            IntermediatePad {
                inactive: false,
                pitch: 0,
                decay: 1.0,
                level: 15,
                pan: 0.5,
                pad: 0.0,
                output: (i % 8) as u8,
                filter: 0,
                finetune: 0.5,
                gain: 0.1000000014901161,
                mono: 0,
                speed: 0,
                sample_path: String::new(),
                factory_content: true,
                play_range_start: 0.0,
                play_range_end: 1.0,
                loop_range_start: 0.0,
                loop_range_end: 1.0,
                loop_mode: 0,
                midikey: 36 + i as u8,
                color: 0,
            }
        });

        IntermediatePreset {
            polyphony: [0; 8],
            volume: 0.699999988079071,
            velocity: 0.0,
            layout: false,
            bank: 0.0,
            mode: 0.0,
            pads: pads_array,
        }
    }
}

#[derive(Debug)]
struct IntermediatePad {
    inactive: bool,
    pitch: i8,
    decay: f64,
    level: u8,
    pan: f64,
    pad: f64,
    output: u8,
    filter: u8,
    finetune: f64,
    gain: f64,
    mono: u8,
    speed: u8,
    sample_path: String,
    factory_content: bool,
    play_range_start: f64,
    play_range_end: f64,
    loop_range_start: f64,
    loop_range_end: f64,
    loop_mode: u8,
    midikey: u8,
    color: u8,
}


fn assign_midi_keys(preset: &mut IntermediatePreset) {
    if preset.layout {
        let mut pad_index = 0;
        for bank in 0..4  {
            let mut midikey = 12 * bank + 36;
            for _ in 0..8  {
                preset.pads[pad_index].midikey = midikey;
                midikey += 1;
                pad_index += 1;
            }
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename = "taldrum")]
pub struct TdPreset {
    #[serde(rename = "@version")]
    pub version: u8,
    
    // #[serde(rename = "@path")]
    // pub path: String,
    
    #[serde(rename = "@name")]
    pub name: String,
    
    // #[serde(rename = "@volume")]
    // pub volume: String,
    
    // #[serde(rename = "@panelmode")]
    // pub panelmode: String,
    
    #[serde(rename = "pads")]
    pub pads: TdPads,
    
    // #[serde(rename = "midimap")]
    // pub global: MidiMap,
}

#[derive(Debug, Serialize)]
pub struct MidiMap {

}

#[derive(Debug, Serialize)]
pub struct TdPads {
    // A Vec named "Pad" will serialize into repeating <Pad> tags
    #[serde(rename = "pad")]
    pub items: Vec<TdPad>,
}

#[derive(Debug, Serialize)]
pub struct TdPad {
    // #[serde(rename = "@version")]
    // pub version: u8,
    
    // #[serde(rename = "@activemappings")]
    // pub activemappings: u8,
    
    // #[serde(rename = "@colour")]
    // pub colour: i32,
    
    #[serde(rename = "@volume")]
    pub volume: f64,
    
    #[serde(rename = "@midikey")]
    pub midikey: u8,
    
    #[serde(rename = "mappings")]
    pub mappings: TdMappings,
}

#[derive(Debug, Serialize)]
pub struct TdMappings {
    #[serde(rename = "mapping")]
    pub mapping: TdMapping,
}

#[derive(Debug, Serialize)]
pub struct  TdMapping {
    #[serde(rename = "@path")]
    pub path: String,
}


fn main() {
    {    
    // let path = "AliveandKickin.rx1200";
    // let xml = fs::read_to_string(path).expect("Didn't work");
    }
    
    let xml_data = include_str!("AliveandKickin.rx1200");
    // let xml_data = include_str!("AllClear.rx1200");
    // let xml_data = include_str!("YoungBlood808.rx1200");

    // Deserialize XML:
    let rx_preset: RxPreset = from_str(xml_data).expect("Failed to parse RX1200 preset");

    let mut intermediate_preset = IntermediatePreset::new();

    
    // process and sort tags into intermediate struct:
    for tag in rx_preset.tags {
        match tag {
            RxTag::Param(p) => process_param(&p, &mut intermediate_preset),
            RxTag::Samples(s) => process_samples_container(&s, &mut intermediate_preset),
            RxTag::Gui(g) => process_gui_container(&g, &mut intermediate_preset),
        }
    }
    // for pad in intermediate.pads  { if !pad.inactive {println!("{pad:?}\n");} }

    assign_midi_keys(&mut intermediate_preset);

    let mut td_pads = TdPads { items: Vec::new() };
    for pad in intermediate_preset.pads {
        if pad.inactive {continue;}
        let td_pad = TdPad {
            // sample_path: Some(pad.sample_path),
            volume: rx_level_and_gain_to_td_volume(pad.level, pad.gain),
            midikey: pad.midikey,
            // midikey: 36.000,
            mappings: TdMappings { mapping:TdMapping { path: pad.sample_path, } }
        };
        td_pads.items.push(td_pad);
    }

    let mut td_preset = TdPreset {
        version: 13,
        name: rx_preset.name,
        pads: td_pads,
    };
    // println!("{td_preset:?}");

    let raw_xml = to_string(&td_preset).expect("Failed to serialize preset");
    // println!("{raw_xml}");
    let final_xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", raw_xml);
    fs::write("Converted_Preset.xml", final_xml).expect("Failed to write file");


}
