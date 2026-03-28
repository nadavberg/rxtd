use hound;
use crate::rx;

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
            pads: std::array::from_fn(|i| {Pad::new(i)}),
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

    pub fn set_truncate_range_and_fix_other_stuff(&mut self) {
        for pad in &mut self.pads {
            if pad.inactive {continue}
            if pad.sample_reversed {
                let temp = pad.play_range_start;
                pad.play_range_start = 1.0 - pad.play_range_end;
                pad.play_range_end = 1.0 - temp;
                let temp = pad.fade_in;
                pad.fade_in = 1.0 - pad.fade_out;
                pad.fade_out = 1.0 - temp;
            }

            // Fix fades:
            if pad.play_range_start > 0.0 || pad.play_range_end < 1.0 {
                let factor = 1.0 / (pad.play_range_end - pad.play_range_start);
                pad.fade_in = (pad.fade_in - pad.play_range_start) * factor;
                pad.fade_out = 1.0 - (pad.play_range_end - pad.fade_out) * factor;
            }

            // Check file and calculate trancate start/end:
            let wav = match hound::WavReader::open(&pad.sample_path) {
                Ok(w) => w,
                Err(e) => {
                    println!("\tProblem with {}: {}", pad.sample_path, e);
                    continue
                },
            };
            let sample_length = wav.duration() as f64;
            if sample_length < 1.0 {
                println!("\tZero length sample! ({})", pad.sample_path);
                continue
            }

            let truncate_start = pad.sample_start as f64 / sample_length;
            let truncate_end = pad.sample_end as f64 / sample_length;
            let factor = truncate_end - truncate_start;

            // Fix start/end points:
            if factor < 1.0 {
                pad.play_range_start = truncate_start + factor * pad.play_range_start;
                pad.play_range_end = truncate_start + factor * pad.play_range_end;
            }
            
            // Fix loop points:
            if pad.sample_reversed {
                let temp = pad.loop_range_start;
                pad.loop_range_start = truncate_end - factor * pad.loop_range_end;
                pad.loop_range_end = truncate_end - factor * temp;
            } else {
                pad.loop_range_start = truncate_start + factor * pad.loop_range_start;
                pad.loop_range_end = truncate_start + factor * pad.loop_range_end;   
            }  
            
            pad.truncate_start = truncate_start;
            pad.truncate_end = truncate_end;
        }
    }
}

#[derive(Debug)]
pub struct Pad {
    pub inactive: bool,

    pub pitch: u8,
    pub decay: f64,
    pub level: u8,
    pub pan: f64,

    pub output: u8,
    pub filter: u8,
    pub finetune: f64,
    pub gain: f64,
    pub mono: u8,
    pub speed: u8,

    pub sample_path: String,
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

fn pad_id_to_index(pad_id: &str) -> usize {
    let bytes = pad_id.as_bytes();
    let bank = (bytes[0] - b'a') as usize;
    let pad = (bytes[1] - b'1') as usize;
    8 * bank + pad
}

fn process_param_tag(param: &rx::Param, intermediate_preset: &mut Preset) {
    let value = match param.value {
        Some(v) => v,
        None => return
    };
    
    if !param.id.contains('_') {
        match param.id.as_str() {
            "volume" => intermediate_preset.volume = value,
            "velocity" => intermediate_preset.velocity = value,
            "layout" => intermediate_preset.layout = (value != 0.0) as bool,
            _ => (),
        }
        return
    }
    
    let (a, b) = param.id.split_once('_').unwrap();
    
    // Polyphony:
    if b.len() == 1 {
        let index = (b.as_bytes()[0] - b'1') as usize;
        intermediate_preset.polyphony[index] = value as u8;
        return
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

fn process_samples_container(samples: &rx::Samples, intermediate_preset: &mut Preset) {
    for sample in &samples.items {
        let pad_index: usize = pad_id_to_index(&sample.id);
        let ref mut pad = intermediate_preset.pads[pad_index];
        
        let Some(references) = &sample.references else {
            pad.inactive = true;
            continue
        };
        
        let Some(reference) = &references.reference else {
            pad.inactive = true;
            continue
        };
        
        pad.sample_path = match reference.ref_type.as_str() {
            "productCommonData" => format!(r"C:/ProgramData/Inphonik/RX1200{}", reference.value),
            _ => reference.value.clone(),
        };     
        
        pad.sample_reversed = sample.reversed;
        pad.sample_gain = sample.gain;
        pad.sample_start = sample.start;
        pad.sample_end = sample.end;
    }
}

fn process_gui_container(gui: &rx::Gui, intermediate_preset: &mut Preset) {
    for g in gui.params.iter() {
        if let Some((_, pad_id)) = g.id.split_once('_') {
            if let Some(value) = g.value {
                let pad_index: usize = pad_id_to_index(pad_id);
                intermediate_preset.pads[pad_index].color = (value * 7.0).round() as u8;
            }
        }
    }
}

impl Pad {
    fn new(i: usize) -> Self {
        // defaults based on "All clear.rx1200"
        let i = i as u8;
        Pad {
            inactive: false,

            pitch: 8,
            decay: 1.0,
            level: 15,
            pan: 0.5,

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

            truncate_start: 0.0,
            truncate_end: 1.0,
        }
    }
}

impl From<rx::Preset> for Preset {
    fn from(rx_preset: rx::Preset) -> Self {
        let mut intermediate = Preset::new();
        for tag in rx_preset.tags {
            match tag {
                rx::Tag::Param(param) => process_param_tag(&param, &mut intermediate),
                rx::Tag::Samples(samples) => process_samples_container(&samples, &mut intermediate),
                rx::Tag::Gui(gui) => process_gui_container(&gui, &mut intermediate),
            }
        }
        intermediate.assign_midi_keys();
        intermediate.set_truncate_range_and_fix_other_stuff();
        intermediate
    }
}
