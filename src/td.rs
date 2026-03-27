use crate::intermediate::IntermediatePreset;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename = "taldrum")]
pub struct TdPreset {
    #[serde(rename = "@version")]
    pub version: u8,
    
    #[serde(rename = "@volume")]
    pub volume: f64,
    
    #[serde(rename = "@numberofpadsmode")]
    pub numberofpadsmode: u8,
    
    #[serde(rename = "@voicesvoicegroupparam00")]
    pub voicesvoicegroupparam00: u8,
    
    #[serde(rename = "@voicesvoicegroupparam01")]
    pub voicesvoicegroupparam01: u8,
    
    #[serde(rename = "@voicesvoicegroupparam02")]
    pub voicesvoicegroupparam02: u8,
    
    #[serde(rename = "@voicesvoicegroupparam03")]
    pub voicesvoicegroupparam03: u8,
    
    #[serde(rename = "@voicesvoicegroupparam04")]
    pub voicesvoicegroupparam04: u8,
    
    #[serde(rename = "@voicesvoicegroupparam05")]
    pub voicesvoicegroupparam05: u8,
    
    #[serde(rename = "@voicesvoicegroupparam06")]
    pub voicesvoicegroupparam06: u8,
    
    #[serde(rename = "@voicesvoicegroupparam07")]
    pub voicesvoicegroupparam07: u8,
    
    #[serde(rename = "pads")]
    pub pads: TdPads,
}

#[derive(Debug, Serialize)]
pub struct TdPads {
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
    
    #[serde(rename = "@voicegroup")]
    pub voicegroup: u8,
    
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

// Transformation Functions:

pub fn rx_color_to_td_color(color: u8) -> i32 {
    match color {
        0 => -13262337, // #35A1FF
        1 => -8099340, // #8469F4
        2 => -48223, // #FF43A1
        3 => -38559, // #FF6961
        4 => -19328, // #FFB480
        5 => -461939, // #F8F38D
        6 => -12396892, // #42D6A4
        7 => -1710619, // #E5E5E5
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
    // Convert parameter to ratio: 
    let mut rx_pitch: f64 = match rx_pitch {
        0  =>  81.0 / 128.0, // -8 semitones  +7.82
        1  =>  85.0 / 128.0, // -7 semitones  -8.73 cents
        2  =>  91.0 / 128.0, // -6 semitones  +9.35 cents
        3  =>  96.0 / 128.0, // -5 semitones  +1.95 cents     3/4
        4  => 102.0 / 128.0, // -4 semitones  +6.91 cents     51/64
        5  => 108.0 / 128.0, // -3 semitones  +5.86 cents     27/32
        6  => 114.0 / 128.0, // -2 semitones  -0.53 cents     57/64
        7  => 121.0 / 128.0, // -1 semitones  +2.64 cents
        8  => 128.0 / 128.0, //                               Unison
        9  => 136.0 / 128.0, // +1 semitones  +4.96 cents     17/16   Minor diatonic semitone
        10 => 144.0 / 128.0, // +2 semitones  +3.91 cents     9/8     Pythagorean major second
        11 => 152.0 / 128.0, // +3 semitones  -2.49 cents     19/16
        12 => 161.0 / 128.0, // +4 semitones  -2.9  cents
        13 => 171.0 / 128.0, // +5 semitones  +1.42 cents
        14 => 181.0 / 128.0, // +6 semitones  -0.18 cents ?
        15 => 192.0 / 128.0, // +7 semitones  +1.95 cents     3/2     Perfect Fifth
        _  => 1.0
    };
    // Convert ratio to semitones:
    rx_pitch = 12.0 * f64::log2(rx_pitch);
    
    // Convert parameter to ratio:
    let mut rx_speed: f64 = match rx_speed {
        0 => 0.5,
        1 => 1.0,
        2 => 45.0 / 33.0,
        3 => 1.5,
        4 => 2.0,
        5 => 78.0 / 33.0,
        _ => 1.0
    };
    // Convert ratio to semitones:
    rx_speed = 12.0 * f64::log2(rx_speed);
    
    // Convert parameter to semitones:
    let rx_finetune = 2.0 * rx_finetune - 1.0;

    let total = rx_pitch + rx_speed + rx_finetune;
    let mut td_tune = (total.floor() + 48.0) / 96.0;
    let mut td_finetune = total.fract() + 0.5;
    
    // RX finetune range is [-100,+100], TD is only [-50,50]:
    if td_finetune > 1.0 {
        td_tune += 1.0 / 96.0;
        td_finetune -= 1.0;
    }
    else if td_finetune < 0.0 {
        td_finetune += 1.0;
    }
    
    // TODO: calculate sample rate? maybe not...

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

pub fn rx_mono_to_td_mono_outputmode(rx_mono: u8) -> (u8, u8) {
    if rx_mono == 1 { return (0, 0) }
    (1, match rx_mono { 2 => 1, 3 => 2, _ => 0 })
}


impl From<IntermediatePreset> for TdPreset {
    fn from(intermediate: IntermediatePreset) -> Self {
        let mut td_pads = TdPads { items: Vec::new() };
        let mut pad_count: u8 = 0;
        let td_velocity = rx_velocity_to_td_velocity(intermediate.velocity);
        let (td_master_volume, td_volume_adjustment) = rx_master_volume_to_td_master_volume(intermediate.volume);
        
        for pad in intermediate.pads {
            if pad.inactive {continue}
            pad_count += 1;

            let td_color = rx_color_to_td_color(pad.color);
            let td_volume = rx_level_and_gain_to_td_volume(pad.level, pad.gain);
            let (td_tune, td_finetune) = rx_pitch_speed_finetune_to_td_tune_finetune(pad.pitch, pad.speed, pad.finetune);
            let td_fadein = f64::min(2.0 * pad.fade_in, 1.0);
            let td_fadeout = f64::min(2.0 * (1.0 - pad.fade_out), 1.0);
            let td_loopenable = (pad.loop_mode > 0) as u8;
            let td_pingpong = (pad.loop_mode > 1) as u8;
            let (td_mono, td_outputmode) = rx_mono_to_td_mono_outputmode(pad.mono);

            td_pads.items.push(TdPad {
                version: 13,
                color: td_color,
                volume: td_volume,
                pan: pad.pan,
                midikey: pad.midikey,
                voicegroup: pad.output,
                mappings: TdMappings { mapping:TdMapping {
                    path: pad.sample_path,
                    start: pad.play_range_start,
                    end: pad.play_range_end,
                    loopstart: pad.loop_range_start,
                    loopend: pad.loop_range_end,
                    fadein: td_fadein,
                    fadeout: td_fadeout,
                    truncatestart: pad.truncate_start,
                    truncateend: pad.truncate_end,
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
                }}
            });
        }

        TdPreset {
            version: 13,
            volume: td_master_volume,
            numberofpadsmode: (pad_count > 16) as u8,
            voicesvoicegroupparam00: intermediate.polyphony[0],
            voicesvoicegroupparam01: intermediate.polyphony[1],
            voicesvoicegroupparam02: intermediate.polyphony[2],
            voicesvoicegroupparam03: intermediate.polyphony[3],
            voicesvoicegroupparam04: intermediate.polyphony[4],
            voicesvoicegroupparam05: intermediate.polyphony[5],
            voicesvoicegroupparam06: intermediate.polyphony[6],
            voicesvoicegroupparam07: intermediate.polyphony[7],
            pads: td_pads,
        }
    }
}
