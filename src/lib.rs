#![allow(warnings, unused)]

use serde::Deserialize;
use serde::Serialize;

use hound;


// Root Struct
#[derive(Debug, Deserialize)]
pub struct RxPreset {
    #[serde(rename = "@name")]
    pub name: String,
    
    // #[serde(rename = "@author")]
    // author: Option<String>,
    
    // #[serde(rename = "@comment")]
    // comment: Option<String>,
    
    #[serde(rename = "$value")]
    pub tags: Vec<RxTag>,
}

// Enum to Catch Interleaved Tags:
#[derive(Debug, Deserialize)]
pub enum RxTag {
    #[serde(rename = "PARAM")]
    Param(RxParam),
    
    #[serde(rename = "SAMPLES")]
    Samples(Samples),
    
    #[serde(rename = "GUI")]
    Gui(RxGui),
}

// Generic Param Tag:
#[derive(Debug, Deserialize)]
pub struct RxParam {
    #[serde(rename = "@id")]
    pub id: String,
    
    #[serde(rename = "@value")]
    pub value: Option<f64>, 
}

// Samples Container:
#[derive(Debug, Deserialize)]
pub struct Samples {
    #[serde(rename = "SAMPLE", default)]
    pub items: Vec<Sample>,
}

// Individual Sample:
#[derive(Debug, Deserialize)]
pub struct Sample {
    #[serde(rename = "@id")]
    pub id: String,
    
    #[serde(rename = "@reversed")]
    pub reversed: bool,
    
    #[serde(rename = "@gain")]
    pub gain: f64,
    
    #[serde(rename = "@start")]
    pub start: u32,
    
    #[serde(rename = "@end")]
    pub end: u32,
    
    #[serde(rename = "REFERENCES")]
    pub references: Option<RxReferences>,
}

// References Container:
#[derive(Debug, Deserialize)]
pub struct RxReferences {
    #[serde(rename = "REFERENCE")]
    pub reference: Option<RxReference>,
}

// Individual Reference:
#[derive(Debug, Deserialize)]
pub struct RxReference {
    #[serde(rename = "@type")]
    pub ref_type: String,
    
    #[serde(rename = "@value")]
    pub value: String,
}

// GUI Container:
#[derive(Debug, Deserialize)]
pub struct RxGui {
    #[serde(rename = "PARAM", default)]
    pub params: Vec<RxParam>, 
}


#[derive(Debug)]
pub struct IntermediatePreset {
    pub name: String,
    pub polyphony: [u8; 8],
    pub volume: f64,
    pub velocity: f64,
    pub layout: bool,
    pub bank: f64,
    pub mode: f64,
    pub pads: [IntermediatePad; 32],
}
impl IntermediatePreset {
    pub fn new() -> Self {
        // defaults based on "All clear.rx1200"
        IntermediatePreset {
            name: String::new(),
            polyphony: [0; 8],
            volume: 0.699999988079071,
            velocity: 0.0,
            layout: false,
            bank: 0.0, // delete?
            mode: 0.0, // delete?
            pads: std::array::from_fn(|i| {IntermediatePad::new(i)}),
        }
    }
    
    pub fn assign_midi_keys(&mut self) {
        if self.layout {
            let mut pad_index = 0;
            for bank in 0..4  {
                let mut midikey = 12 * bank + 36;
                for _ in 0..8  {
                    self.pads[pad_index].midikey = midikey;
                    midikey += 1;
                    pad_index += 1;
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct IntermediatePad {
    pub inactive: bool,

    pub pitch: u8,
    pub decay: f64,
    pub level: u8,
    pub pan: f64,

    pub pad: f64,
    pub output: u8,
    pub filter: u8,
    pub finetune: f64,
    pub gain: f64,
    pub mono: u8,
    pub speed: u8,

    pub sample_path: String,
    pub sample_length: u32,
    pub sample_reversed: bool,//false,
    pub sample_gain: f64,//1.0,
    pub sample_start: u32,//0,
    pub sample_end: u32,//0

    pub play_range_start: f64,
    pub play_range_end: f64,
    pub loop_range_start: f64,
    pub loop_range_end: f64,
    pub loop_mode: u8,

    pub fade_in: f64,
    pub fade_out: f64,

    pub midikey: u8,
    pub color: u8,
}
impl IntermediatePad {
    pub fn new(i: usize) -> Self {
        // defaults based on "All clear.rx1200"
        let i = i as u8;
        IntermediatePad {
            inactive: false,

            pitch: 8,
            decay: 1.0,
            level: 15,
            pan: 0.5,

            pad: 0.0,
            output: (i % 8),
            filter: 0,
            finetune: 0.5,
            gain: 0.1000000014901161,
            mono: 0,
            speed: 0,

            sample_path: String::new(),
            sample_length: 0,
            sample_reversed: false,
            sample_gain: 1.0,
            sample_start: 0,
            sample_end: 0,
            
            play_range_start: 0.0,
            play_range_end: 1.0,
            loop_range_start: 0.0,
            loop_range_end: 1.0,
            loop_mode: 0,

            fade_in: 0.0,
            fade_out: 1.0,

            midikey: 36 + i,
            color: 0,
        }
    }
}


#[derive(Debug, Serialize)]
#[serde(rename = "taldrum")]
pub struct TdPreset {
    #[serde(rename = "@version")]
    pub version: u8,
    
    #[serde(rename = "@volume")]
    pub volume: f64,
    
    #[serde(rename = "@numberofpadsmode")]
    pub numberofpadsmode: u8,
    
    #[serde(rename = "pads")]
    pub pads: TdPads,
}

#[derive(Debug, Serialize)]
pub struct TdPads {
    // A Vec named "Pad" will serialize into repeating <Pad> tags
    #[serde(rename = "pad")]
    pub items: Vec<TdPad>,
}

#[derive(Debug, Serialize)]
pub struct TdPad {
    #[serde(rename = "@version")]
    pub version: u8,
    
    #[serde(rename = "@colour")] // sic
    pub color: i32,
    
    #[serde(rename = "@volume")]
    pub volume: f64,
    
    #[serde(rename = "@pan")]
    pub pan: f64,
    
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

    #[serde(rename = "@start")]
    pub start: f64,

    #[serde(rename = "@end")]
    pub end: f64,

    #[serde(rename = "@loopstart")]
    pub loopstart: f64,

    #[serde(rename = "@loopend")]
    pub loopend: f64,

    #[serde(rename = "@fadein")]
    pub fadein: f64,

    #[serde(rename = "@fadeout")]
    pub fadeout: f64,

    #[serde(rename = "@truncatestart")]
    pub truncatestart: f64,

    #[serde(rename = "@truncateend")]
    pub truncateend: f64,

    #[serde(rename = "@volume")]
    pub volume: f64,

    #[serde(rename = "@tune")]
    pub tune: f64,

    #[serde(rename = "@finetune")]
    pub finetune: f64,

    #[serde(rename = "@mono")]
    pub mono: u8,

    #[serde(rename = "@outputmode")]
    pub outputmode: u8,

    #[serde(rename = "@velocityintensity")]
    pub velocityintensity: f64,

    #[serde(rename = "@reversenabled")]
    pub reversenabled: u8,

    #[serde(rename = "@normalizesample")]
    pub normalizesample: u8,

    #[serde(rename = "@loopenabled")]
    pub loopenabled: u8,

    #[serde(rename = "@loopmode")]
    pub loopmode: u8,
}

pub fn build_intermediate_preset(rx_preset: RxPreset) -> IntermediatePreset {
    let mut intermediate_preset = IntermediatePreset::new();
    for tag in rx_preset.tags {
        match tag {
            RxTag::Param(param) => process_param_tag(&param, &mut intermediate_preset),
            RxTag::Samples(samples) => process_samples_container(&samples, &mut intermediate_preset),
            RxTag::Gui(gui) => process_gui_container(&gui, &mut intermediate_preset),
        }
    }
    intermediate_preset.assign_midi_keys();
    intermediate_preset.name = rx_preset.name;
    intermediate_preset
}

pub fn pad_id_to_index(pad_id: &str) -> usize {
    let bytes = pad_id.as_bytes();
    let bank = (bytes[0] - b'a') as usize;
    let pad = (bytes[1] - b'1') as usize;
    8 * bank + pad
}

pub fn process_param_tag(param: &RxParam, intermediate_preset: &mut IntermediatePreset) {
    if param.value.is_none() { return; }
    let value = param.value.unwrap();

    if !param.id.contains('_') {
        match param.id.as_str() {
            "volume" => intermediate_preset.volume = value,
            "velocity" => intermediate_preset.velocity = value,
            "layout" => intermediate_preset.layout = (value != 0.0) as bool,
            _ => (),
        }
        return;
    }
    
    let (a, b) = param.id.split_once('_').unwrap();
    
    // Polyphony:
    if b.len() == 1 {
        let index = (b.as_bytes()[0] - b'1') as usize;
        intermediate_preset.polyphony[index] = value as u8;
        return;
    }

    let (param_name, pad_id): (&str, &str);

    if a.len() == 2 {
        pad_id = a;
        param_name = b;
    } else {
        pad_id = b;
        param_name = a;
    }

    let pad_index: usize = pad_id_to_index(pad_id);
    let ref mut pad = intermediate_preset.pads[pad_index];

    match param_name {
        "pitch" => pad.pitch = (15.0 * value).round() as u8,
        "decay" => pad.decay = value,
        "level" => pad.level = (15.0 * value).round() as u8,
        "pan" => pad.pan = value,
        "pad" => pad.pad = value,
        "output" => pad.output = value as u8,
        "filter" => pad.filter = value as u8,
        "finetune" => pad.finetune = value,
        "gain" => pad.gain = value,
        "mono" => pad.mono = value as u8,
        "speed" => pad.speed = value as u8,
        "loop_mode" => pad.loop_mode = value as u8,
        "loop_range_end" => pad.loop_range_end = value,
        "loop_range_start" => pad.loop_range_start = value,
        "play_range_end" => pad.play_range_end = value,
        "play_range_start" => pad.play_range_start = value,
        "fade_in" => pad.fade_in = value,
        "fade_out" => pad.fade_out = value,
        _ => ()
    }
}

pub fn process_samples_container(samples: &Samples, intermediate_preset: &mut IntermediatePreset) {
    for sample in &samples.items {
        let pad_index: usize = pad_id_to_index(&sample.id);
        let ref mut pad = intermediate_preset.pads[pad_index];
        
        if sample.references.is_none() {pad.inactive = true; continue}
        let references = sample.references.as_ref().unwrap();
        if references.reference.is_none() {pad.inactive = true; continue}
        let reference = references.reference.as_ref().unwrap();
 
        pad.sample_reversed = sample.reversed;
        pad.sample_gain = sample.gain;
        pad.sample_start = sample.start;
        pad.sample_end = sample.end;
        
        let sample_path =
            if reference.ref_type == "productCommonData" {
                r"C:/ProgramData/Inphonik/RX1200".to_string() + reference.value.as_str()
            } else {
                reference.value.clone()
            };
        pad.sample_path = sample_path.clone();

        let wav = hound::WavReader::open(&sample_path);
        if wav.is_err() {
            println!("{} {}", wav.err().unwrap(), sample_path.rsplit_once('\\').expect("feugh...").1);
            continue
        }
        pad.sample_length = wav.unwrap().duration();
    }
}

pub fn process_gui_container(gui: &RxGui, intermediate_preset: &mut IntermediatePreset) {
    for g in gui.params.iter() {
        if g.value.is_none() { continue }
        let value = g.value.unwrap();
        if !g.id.contains('_') {
            // delete? (just continue)
            match g.id.as_str() {
                "bank" => intermediate_preset.bank = value,
                "mode" => intermediate_preset.mode = value,
                _ => (),
            }
            continue;
        }

        let (_, pad_id) = g.id.split_once('_').expect("foo...");
        let pad_index: usize = pad_id_to_index(pad_id);
        intermediate_preset.pads[pad_index].color = (value * 7.0).round() as u8;
    }
}



// Transformation Functions:

pub fn rx_color_to_td_color(color: u8) -> i32 {
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

pub fn rx_level_and_gain_to_td_volume(level: u8, gain: f64) -> f64 {
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

pub fn rx_velocity_to_td_velocity(rx_velocity: f64) -> f64 {
    (127.0 / 126.0) * (1.0 - f64::powf(0.01, rx_velocity))
}

pub fn rx_pitch_speed_finetune_to_td_tune_finetune(rx_pitch: u8, rx_speed: u8, rx_finetune: f64) -> (f64, f64) {
    let mut rx_pitch: f64 = match rx_pitch {
         0 =>  81.0 / 128.0, // -8  +7.82
         1 =>  85.0 / 128.0, // -7  -8.73 cents
         2 =>  91.0 / 128.0, // -6  +9.35 cents
         3 =>  96.0 / 128.0, // -5  +1.95 cents     3/4
         4 => 102.0 / 128.0, // -4  +6.91 cents     51/64
         5 => 108.0 / 128.0, // -3  +5.86 cents     27/32
         6 => 114.0 / 128.0, // -2  -0.53 cents     57/64
         7 => 121.0 / 128.0, // -1  +2.64 cents
         8 => 128.0 / 128.0, // +0                          Unison
         9 => 136.0 / 128.0, // +1  +4.96 cents     17/16   Minor diatonic semitone
        10 => 144.0 / 128.0, // +2  +3.91 cents     9/8     Pythagorean major second
        11 => 152.0 / 128.0, // +3  -2.49 cents     19/16
        12 => 161.0 / 128.0, // +4  -2.9  cents
        13 => 171.0 / 128.0, // +5  +1.42 cents
        14 => 181.0 / 128.0, // +6  -0.18 cents ?
        15 => 192.0 / 128.0, // +7  +1.95 cents     3/2     Perfect Fifth
        _  => 1.0
    };
    rx_pitch = 12.0 * f64::log2(rx_pitch);
    
    let mut rx_speed: f64 = match rx_speed {
        0 => 0.5,
        1 => 1.0,
        2 => 45.0 / 33.0,
        3 => 1.5,
        4 => 2.0,
        5 => 78.0 / 33.0,
        _ => 1.0
    };
    rx_speed = 12.0 * f64::log2(rx_speed);

    let rx_finetune = 2.0 * rx_finetune - 1.0;

    let total = rx_pitch + rx_speed + rx_finetune;
    // let td_tune = (total.round() + 48.0) / 96.0;
    let mut td_tune = (total.floor() + 48.0) / 96.0;
    let mut td_finetune = total.fract() + 0.5;
    
    // println!("total = {total}, td_tune = {td_tune}, td_finetune = {td_finetune}");


    if td_finetune > 1.0 {
        td_tune += 1.0 / 96.0;
        td_finetune -= 1.0;
    }
    else if td_finetune < 0.0 {
        td_finetune += 1.0;
    }
    
    // println!("total = {total}, td_tune = {td_tune}, td_finetune = {td_finetune}");

    (td_tune, td_finetune)
}

pub fn rx_master_volume_to_td_master_volume(rx_master_volume: f64) -> (f64, f64) {
    let rx_master_volume = rx_master_volume / 0.6999998092651;
    let td_master_volume: f64;
    let td_pad_volume_adjustment: f64;

    if rx_master_volume > f64::powf(4.0 / 3.0, 1.0 / 3.0) {
        td_master_volume = 1.0;
        td_pad_volume_adjustment = f64::sqrt(f64::powi(rx_master_volume, 3) * 3.0 / 16.0);
    } else {
        td_master_volume = 0.75 * f64::powi(rx_master_volume, 3);
        td_pad_volume_adjustment = 0.5;
    }

    (td_master_volume, td_pad_volume_adjustment)
}

pub fn fix_play_range_loop_fade_and_get_trunc(pad: &mut IntermediatePad) -> (f64, f64) {
    // println!("start:{}\tend:{}", pad.play_range_start, pad.play_range_end);
    if pad.sample_reversed {
        let temp = pad.play_range_start;
        pad.play_range_start = 1.0 - pad.play_range_end;
        pad.play_range_end = 1.0 - temp;
        let temp = pad.fade_in;
        pad.fade_in = 1.0 - pad.fade_out;
        pad.fade_out = 1.0 - temp;
    }

    // println!("in:{}\tout:{}", pad.fade_in, pad.fade_out);
    if pad.play_range_start > 0.0 || pad.play_range_end < 1.0 {
        let factor = 1.0 / (pad.play_range_end - pad.play_range_start);
        pad.fade_in = (pad.fade_in - pad.play_range_start) * factor;
        pad.fade_out = 1.0 - (pad.play_range_end - pad.fade_out) * factor;
    }
        
    let sample_length = pad.sample_length as f64;
    let td_truncate_start = pad.sample_start as f64 / sample_length;
    let td_truncate_end = pad.sample_end as f64 / sample_length;

    // Fix start/end points:
    let length_as_ratio = td_truncate_end - td_truncate_start;
    if length_as_ratio < 1.0 {
        pad.play_range_start = td_truncate_start + length_as_ratio * pad.play_range_start;
        pad.play_range_end = td_truncate_start + length_as_ratio * pad.play_range_end;
        
    }
    
    // Fix loop points:
    if pad.sample_reversed {
        let temp = pad.loop_range_start;
        pad.loop_range_start = td_truncate_end - length_as_ratio * pad.loop_range_end;
        pad.loop_range_end = td_truncate_end - length_as_ratio * temp;
    } else {
        pad.loop_range_start = td_truncate_start + length_as_ratio * pad.loop_range_start;
        pad.loop_range_end = td_truncate_start + length_as_ratio * pad.loop_range_end;   
    }  
        
        (td_truncate_start, td_truncate_end)
    }
    


pub fn build_td_preset(preset: IntermediatePreset) -> TdPreset {
    let mut td_pads = TdPads { items: Vec::new() };
    let mut pad_count: u8 = 0;
    let td_velocity = rx_velocity_to_td_velocity(preset.velocity);
    let (td_master_volume, td_volume_adjustment) = rx_master_volume_to_td_master_volume(preset.volume);
    
    for mut pad in preset.pads {
        if pad.inactive {continue}
        // println!("{pad:?}\n");

        let td_color = rx_color_to_td_color(pad.color);
        let td_volume = rx_level_and_gain_to_td_volume(pad.level, pad.gain);
        let (td_truncate_start, td_truncate_end) = fix_play_range_loop_fade_and_get_trunc(&mut pad);
        let (td_tune, td_finetune) = rx_pitch_speed_finetune_to_td_tune_finetune(pad.pitch, pad.speed, pad.finetune);

        let td_fadein = f64::min(2.0 * pad.fade_in, 1.0);
        let td_fadeout = f64::min(2.0 * (1.0 - pad.fade_out), 1.0);

        let td_loopenable = (pad.loop_mode > 0) as u8;
        let td_pingpong = (pad.loop_mode > 1) as u8;
        
        let td_mono: u8;
        let td_outputmode: u8;
        if pad.mono == 1 {
            td_mono = 0;
            td_outputmode = 0;
        } else {
            td_mono = 1;
            td_outputmode = match pad.mono {
                0 => 0,
                2 => 1,
                3 => 2,
                _ => 0,
            }
        };

        let td_pad = TdPad {
            version: 13,
            color: td_color,
            volume: td_volume,
            pan: pad.pan,
            midikey: pad.midikey,
            mappings: TdMappings {
                mapping:TdMapping {
                    path: pad.sample_path,
                    start: pad.play_range_start,
                    end: pad.play_range_end,
                    loopstart: pad.loop_range_start,
                    loopend: pad.loop_range_end,
                    fadein: td_fadein,
                    fadeout: td_fadeout,
                    truncatestart: td_truncate_start,
                    truncateend: td_truncate_end,
                    volume: td_volume_adjustment,
                    tune: td_tune,
                    finetune: td_finetune,
                    mono: td_mono,
                    outputmode: td_outputmode,
                    velocityintensity: td_velocity,
                    reversenabled: pad.sample_reversed as u8,
                    normalizesample: 0,
                    loopenabled: td_loopenable,
                    loopmode: td_pingpong,
                }
            }
        };
        td_pads.items.push(td_pad);
        pad_count += 1;
    }

    let td_preset = TdPreset {
        version: 13,
        // name: preset.name,
        volume: td_master_volume,
        numberofpadsmode: (pad_count > 16) as u8,
        pads: td_pads,
    };

    td_preset
}