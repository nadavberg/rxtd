use crate::intermediate;
use serde::Serialize;
use std::ffi::OsStr;
use std::path::Path;
use std::fs;

#[derive(Debug, Serialize)]
#[serde(rename = "taldrum")]
pub struct Preset {
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
    pub pads: Pads,
}

#[derive(Debug, Serialize)]
pub struct Pads {
    #[serde(rename = "pad")]
    pub items: Vec<Pad>,
}

#[derive(Debug, Serialize)]
pub struct Pad {
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
    pub mappings: Mappings,
}

#[derive(Debug, Serialize)]
pub struct Mappings {
    #[serde(rename = "mapping")]
    pub mapping: Mapping,
}

#[derive(Debug, Serialize)]
pub struct  Mapping {
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

    #[serde(rename = "@filtertype")]
    pub filtertype: u8,

    #[serde(rename = "@filtercutoff")]
    pub filtercutoff: f64,

    #[serde(rename = "@filterresonance")]
    pub filterresonance: f64,

    #[serde(rename = "@filterdrive")]
    pub filterdrive: f64,

    #[serde(rename = "@ampattack")]
    pub ampattack: f64,

    #[serde(rename = "@amphold")]
    pub amphold: f64,

    #[serde(rename = "@ampdecay")]
    pub ampdecay: f64,

    #[serde(rename = "@ampsustain")]
    pub ampsustain: f64,

    #[serde(rename = "@amprelease")]
    pub amprelease: f64,

    #[serde(rename = "@env1attack")]
    pub env1attack: f64,

    #[serde(rename = "@env1hold")]
    pub env1hold: f64,

    #[serde(rename = "@env1decay")]
    pub env1decay: f64,

    #[serde(rename = "@env1sustain")]
    pub env1sustain: f64,

    #[serde(rename = "@env1release")]
    pub env1release: f64,

    #[serde(rename = "@matrix0source")]
    pub matrix0source: u8,

    #[serde(rename = "@matrix0destination")]
    pub matrix0destination: u8,

    #[serde(rename = "@matrix0intensity")]
    pub matrix0intensity: f64,

}

// Transformation Functions:

fn rx_to_td_color(color: u8) -> i32 {
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

#[allow(clippy::excessive_precision)]
fn rx_to_td_volume(level: u8, gain: f64) -> f64 {
    let level_db: f64 = match level {
         0 => -48.125540454678699,
         1 => -38.583116391055221,
         2 => -34.146140574112714,
         3 => -31.223580096152403,
         4 => -29.040687631705044,
         5 => -25.846673804992303,
         6 => -23.516561117611591,
         7 => -20.890984765096807,
         8 => -18.298307076753225,
         9 => -15.869863144299639,
        10 => -13.161780356313614,
        11 => -10.395725911069878,
        12 => -7.7017553469803106,
        13 => -5.2652451244176559,
        14 => -2.5963049656793182,
        15 =>  0.0052625759604806987,
        _  =>  0.0,
    };
    let level_ratio = f64::powf(10.0, level_db * 0.05);
    0.5 * f64::sqrt(level_ratio * 10.0 * gain)
}

fn rx_velocity_to_td(rx_velocity: f64) -> f64 {
    (1.0 - f64::powf(0.01, rx_velocity)) / 0.99
    // (1.0 - f64::powf(0.01, rx_velocity)) * (127.0 / 126.0)
}

#[allow(clippy::eq_op)]
fn rx_to_td_pitch(rx_pitch: u8, rx_speed: u8, rx_finetune: f64) -> (f64, f64) {
    let pitch_ratio: f64 = match rx_pitch {
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
    let pitch_semitones = 12.0 * f64::log2(pitch_ratio);
    
    let speed_ratio: f64 = match rx_speed {
        0 => 0.5,
        1 => 1.0,
        2 => 45.0 / 33.0,
        3 => 1.5,
        4 => 2.0,
        5 => 78.0 / 33.0,
        _ => 1.0,
    };
    let speed_semitones = 12.0 * f64::log2(speed_ratio);
    
    let finetune_semitones = 2.0 * rx_finetune - 1.0;

    let total = pitch_semitones + speed_semitones + finetune_semitones;
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

fn rx_to_td_fades(rx_fade_in: f64, rx_fade_out: f64) -> (f64, f64) {
    let fadein = f64::min(2.0 * rx_fade_in, 1.0);
    let fadeout = f64::min(2.0 * (1.0 - rx_fade_out), 1.0);
    (fadein, fadeout)
}

fn rx_master_volume_to_td(rx_master_volume: f64) -> (f64, f64) {
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

fn rx_to_td_stereo(rx_mono: u8) -> (u8, u8) {
    if rx_mono == 1 { return (0, 0) }
    (1, match rx_mono { 2 => 1, 3 => 2, _ => 0 })
}

fn rx_to_td_filter(rx_filter: u8) -> (u8, f64, f64, f64, u8, u8, f64) {
    let mut filtertype = 1u8;
    let mut cutoff = 1.0; // td default
    let mut resonance = 0.0; // td default
    let mut drive = 0.0;
    let mut matrix_source = 0u8; // td default
    let mut matrix_destination = 0u8; // td default
    let mut matrix_intensity = 0.5; // td default

    match rx_filter {
        1 => {
            cutoff = 0.889539361000061; // 13 kHz
            resonance = 0.16;
        },
        2 => {
            cutoff = 0.8546499609947205; // 10 kHz
            resonance = 0.16;
        },
        3 => {
            cutoff = 0.4597677886486053; // 500 Hz
            resonance = 0.2;
            matrix_source = 7;
            matrix_destination = 4;
            matrix_intensity = 0.75;
        },
        _ => {
            filtertype = 0; // td default
            drive = 0.5; // td default
        },
    }
    (filtertype, cutoff, resonance, drive, matrix_source, matrix_destination, matrix_intensity)
}

impl From<intermediate::Preset> for Preset {
    fn from(intermediate: intermediate::Preset) -> Self {
        let mut pads = Pads { items: Vec::new() };
        let mut pad_count: u8 = 0;
        let version = 13;
        let (td_master_volume, td_volume_adjustment) = rx_master_volume_to_td(intermediate.volume);
        let velocityintensity = rx_velocity_to_td(intermediate.velocity);
        
        for pad in intermediate.pads {
            // dbg!(&pad);

            if pad.inactive {continue}
            pad_count += 1;
            let color = rx_to_td_color(pad.color);
            let td_volume = rx_to_td_volume(pad.level, pad.gain);
            let (tune, finetune) = rx_to_td_pitch(pad.pitch, pad.speed, pad.finetune);
            let (fadein, fadeout) = rx_to_td_fades(pad.fade_in, pad.fade_out);
            let (mono, outputmode) = rx_to_td_stereo(pad.mono);
            let loopenabled = (pad.loop_mode > 0) as u8;
            let loopmode = (pad.loop_mode > 1) as u8;
            let reversenabled = pad.sample_reversed as u8;
            let (
                filtertype,
                filtercutoff,
                filterresonance,
                filterdrive,
                matrix0source,
                matrix0destination,
                matrix0intensity
            ) = rx_to_td_filter(pad.filter);

            pads.items.push(Pad {
                version,
                color,
                volume: td_volume,
                pan: pad.pan,
                midikey: pad.midikey,
                voicegroup: pad.output,
                mappings: Mappings { mapping:Mapping {
                    path: pad.sample_path,
                    start: pad.play_range_start,
                    end: pad.play_range_end,
                    loopstart: pad.loop_range_start,
                    loopend: pad.loop_range_end,
                    fadein,
                    fadeout,
                    truncatestart: pad.truncate_start,
                    truncateend: pad.truncate_end,
                    volume: td_volume_adjustment,
                    tune,
                    finetune,
                    mono,
                    outputmode,
                    velocityintensity,
                    reversenabled,
                    normalizesample: 0, // <= TODO
                    loopenabled,
                    loopmode,

                    filtertype,
                    filtercutoff,
                    filterresonance,
                    filterdrive,

                    ampattack: 0.0,
                    amphold: 0.0, //
                    ampdecay: 0.0, //
                    ampsustain: 1.0, //
                    amprelease: 0.0,

                    env1attack: 0.0,
                    env1hold: 0.0,
                    env1decay: 0.3,
                    env1sustain: 0.0,
                    env1release: 0.0,

                    matrix0source,
                    matrix0destination,
                    matrix0intensity,
                }}
            });
        }

        Preset {
            version,
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
            pads,
        }
    }
}

impl Preset {
    pub fn save_to_file(&self, path: &Path, name: &OsStr) -> anyhow::Result<()> {
        let xml = quick_xml::se::to_string(self)?;
        let mut path = path.to_path_buf();
        path.push(name);
        path.set_extension("taldrum");
        fs::write(path, xml)?;
        Ok(())
    }
}
