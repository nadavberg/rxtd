use crate::rx;
use anyhow::{anyhow, Result};
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Preset {
    pub polyphony: [u8; 8],
    pub volume: f64,
    pub velocity: f64,
    pub layout: bool,
    pub pads: [Pad; 32],
}

impl Preset {
    pub fn new() -> Self {
        // defaults based on "All clear.rx1200"
        Preset {
            polyphony: [0; 8],
            volume: 0.699999988079071,
            velocity: 0.0,
            layout: false,
            pads: std::array::from_fn(Pad::new),
        }
    }

    pub fn assign_midi_keys(&mut self) {
        if self.layout {
            let mut index = 0;
            for bank in 0..4 {
                let mut midikey = 36 + 12 * bank;
                for _ in 0..8 {
                    self.pads[index].midikey = midikey;
                    midikey += 1;
                    index += 1;
                }
            }
        } else {
            let mut midikey = 36;
            for i in 0..32 {
                self.pads[i].midikey = midikey;
                midikey += 1;
            }
        }
    }

    pub fn finalize_preset(&mut self) {
        self.assign_midi_keys();
        for pad in &mut self.pads {
            if pad.inactive {continue}
            if pad.sample_reversed {pad.fix_reversed_pad()}
            pad.fix_fades();
            pad.set_truncate_range();
            pad.fix_loop_range();
            // dbg!(&pad);
        }
    }
}

#[derive(Debug)]
pub struct Pad {
    pub inactive: bool,

    pub pitch: u8,
    pub decay: u8,
    pub level: u8,
    pub pan: f64,

    pub output: u8,
    pub filter: u8,
    pub finetune: f64,
    pub gain: f64,
    pub mono: u8,
    pub speed: u8,

    // pub sample_path: String,
    pub sample_path: PathBuf,
    pub sample_length: u32,
    pub sample_reversed: bool,
    pub sample_gain: f64,
    pub sample_start: u32,
    pub sample_end: u32,

    pub play_range_start: f64,
    pub play_range_end: f64,
    pub loop_range_start: f64,
    pub loop_range_end: f64,
    pub loop_mode: u8,

    pub fade_in: f64,
    pub fade_out: f64,

    pub midikey: u8,
    pub color: u8,

    pub truncate_start: f64,
    pub truncate_end: f64,
}

impl Pad {
    fn new(i: usize) -> Self {
        // defaults based on "All clear.rx1200"
        let i = i as u8;
        Pad {
            inactive: false,

            pitch: 8,
            decay: 0,
            level: 15,
            pan: 0.5,

            output: (i % 8),
            filter: 0,
            finetune: 0.5,
            gain: 0.1000000014901161,
            mono: 0,
            speed: 0,

            // sample_path: String::new(),
            sample_path: PathBuf::new(),
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

            truncate_start: 0.0,
            truncate_end: 1.0,
        }
    }

    fn fix_reversed_pad(&mut self) {
        let temp = self.play_range_start;
        self.play_range_start = 1.0 - self.play_range_end;
        self.play_range_end = 1.0 - temp;
        let temp = self.fade_in;
        self.fade_in = 1.0 - self.fade_out;
        self.fade_out = 1.0 - temp;
    }

    fn fix_fades(&mut self) {
        let factor = 1.0 / (self.play_range_end - self.play_range_start);
        self.fade_in = (self.fade_in - self.play_range_start) * factor;
        self.fade_out = 1.0 - (self.play_range_end - self.fade_out) * factor;
    }

    fn set_truncate_range(&mut self) {
        let sample_length = match get_sample_length(&self.sample_path) {
            Ok(n) => n as f64,
            Err(e) => {
                eprintln!("Problem with {}: {}", self.sample_path.display(), e);
                return
            }
        };

        if sample_length < 1.0 {
            eprintln!("Problem with {}: Zero length sample!", self.sample_path.display());
            return
        }

        self.truncate_start = self.sample_start as f64 / sample_length;
        self.truncate_end = self.sample_end as f64 / sample_length;

        // Fix start/end points:
        let factor = self.truncate_end - self.truncate_start;
        if factor < 1.0 {
            self.play_range_start = self.truncate_start + factor * self.play_range_start;
            self.play_range_end = self.truncate_start + factor * self.play_range_end;
        }
    }

    fn fix_loop_range(&mut self) {
        let factor = self.truncate_end - self.truncate_start;
        if self.sample_reversed {
            let temp = self.loop_range_start;
            self.loop_range_start = self.truncate_end - factor * self.loop_range_end;
            self.loop_range_end = self.truncate_end - factor * temp;
        } else {
            self.loop_range_start = self.truncate_start + factor * self.loop_range_start;
            self.loop_range_end = self.truncate_start + factor * self.loop_range_end;
        }
    }
}

impl From<rx::Preset> for Preset {
    fn from(rx_preset: rx::Preset) -> Self {
        let mut preset = Preset::new();
        for tag in rx_preset.tags {
            match tag {
                rx::Tag::Param(param) => process_param_tag(&param, &mut preset),
                rx::Tag::Samples(samples) => process_samples_container(&samples, &mut preset),
                rx::Tag::Gui(gui) => process_gui_container(&gui, &mut preset),
            }
        }
        preset.finalize_preset();
        preset
    }
}

// fn get_sample_length(path: &str) -> Result<u64> {
fn get_sample_length(path: &Path) -> Result<u64> {
    use symphonia::core::codecs::CODEC_TYPE_NULL;
    use symphonia::core::formats::FormatOptions;
    use symphonia::core::io::MediaSourceStream;
    use symphonia::core::meta::MetadataOptions;
    use symphonia::core::probe::Hint;
    use std::path::Path;
    
    let source = std::fs::File::open(&path)?;
    let mss = MediaSourceStream::new(Box::new(source), Default::default());
    
    // WAV, AIFF, FLAC, MP3, OGG, M4A, AU, SND, W64, WV, PCM
    let extension = path.extension().and_then(|x| x.to_str()).unwrap_or("");
    let mut hint = Hint::new();
    hint.with_extension(extension);

    let metadata_opts: MetadataOptions = Default::default();
    let format_opts: FormatOptions = Default::default();

    let probed = symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;

    let mut format = probed.format;
    let Some(track) = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
    else {
        return Err(anyhow!("no supported audio tracks"));
    };
   
    Ok(track.codec_params.n_frames.unwrap_or_default())
}

fn pad_id_to_index(pad_id: &str) -> Option<usize> {
    let bytes = pad_id.as_bytes();
    if bytes.len() == 2
        || matches!(bytes[0], b'a'..=b'd')
        || matches!(bytes[1], b'1'..=b'8')
    {
        let bank = (bytes[0] - b'a') as usize;
        let pad = (bytes[1] - b'1') as usize;
        Some(8 * bank + pad)
    } else {
        None
    }
}

fn process_param_tag(param: &rx::Param, intermediate_preset: &mut Preset) {
    
    let value = match param.value {
        Some(v) => v,
        None => return,
    };

    if !param.id.contains('_') {
        match param.id.as_str() {
            "volume" => intermediate_preset.volume = value,
            "velocity" => intermediate_preset.velocity = value,
            "layout" => intermediate_preset.layout = value != 0.0,
            _ => (),
        }
        return
    }
    let (a, b) = param.id.split_once('_').unwrap();
    
    // Polyphony:
    if b.len() == 1 {
        let index = (b.as_bytes()[0] - b'1') as usize;
        if matches!(index, 0..8) {
            intermediate_preset.polyphony[index] = value as u8;
        }
        return
    }
    
    let (param_name, pad_id) = if a.len() == 2 { (b, a) } else { (a, b) };
    let Some(pad_index) = pad_id_to_index(pad_id) else {return};
    let pad = &mut intermediate_preset.pads[pad_index];
    // println!("{param_name} {pad_id} ({pad_index}) {value}");
    match param_name {
        "pitch" => pad.pitch = (15.0 * value).round() as u8,
        "decay" => pad.decay = (15.0 * value).round() as u8,
        "level" => pad.level = (15.0 * value).round() as u8,
        "pan" => pad.pan = value,
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
        _ => (),
    }
}

fn process_samples_container(samples: &rx::Samples, intermediate_preset: &mut Preset) {
    for sample in &samples.items {
        let Some(pad_index) = pad_id_to_index(&sample.id) else {continue};
        let pad = &mut intermediate_preset.pads[pad_index];

        let Some(references) = &sample.references else {
            pad.inactive = true;
            continue;
        };

        let Some(reference) = &references.reference else {
            pad.inactive = true;
            continue;
        };

        let path = match reference.ref_type.as_str() {
            "productCommonData" => {
                let programdata_folder = env::var("PROGRAMDATA").expect("Failed to get ProgramData folder");
                let path = format!(r"{}\Inphonik\RX1200{}", programdata_folder, reference.value);
                PathBuf::from(path)
            },
            "productUserData" => {
                let appdata_folder = env::var("APPDATA").expect("Failed to get AppData folder");
                let path = format!(r"{}\Inphonik\RX1200{}", appdata_folder, reference.value);
                PathBuf::from(path)
            }
            _ => PathBuf::from(&reference.value),
        };
        
        // dbg!(&path);
        pad.sample_path = match dunce::canonicalize(&path) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Problem with {}: {}", path.display(), e);
                pad.inactive = true;
                path
            }
        };
        // dbg!(&pad.sample_path);

        pad.sample_reversed = sample.reversed;
        pad.sample_gain = sample.gain;
        pad.sample_start = sample.start;
        pad.sample_end = sample.end;
    }
}

fn process_gui_container(rx_gui: &rx::Gui, preset: &mut Preset) {
    for g in rx_gui.params.iter() {
        if let Some((_, pad_id)) = g.id.split_once('_')
            && let Some(value) = g.value
            && let Some(pad_index) = pad_id_to_index(pad_id)
        {
            preset.pads[pad_index].color = (value * 7.0).round() as u8;
        }
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
