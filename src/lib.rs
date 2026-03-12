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
         0 =>  81.0 / 128.0,
         1 =>  85.0 / 128.0,
         2 =>  91.0 / 128.0,
         3 =>  96.0 / 128.0,
         4 => 102.0 / 128.0,
         5 => 108.0 / 128.0,
         6 => 114.0 / 128.0,
         7 => 121.0 / 128.0,
         8 => 128.0 / 128.0,
         9 => 136.0 / 128.0,
        10 => 144.0 / 128.0,
        11 => 152.0 / 128.0,
        12 => 161.0 / 128.0,
        13 => 171.0 / 128.0,
        14 => 181.0 / 128.0,
        15 => 192.0 / 128.0,
        _  => 1.0
    };
    rx_pitch = 12.0 * f64::log2(rx_pitch);
    
    let mut rx_speed: f64 = match rx_speed {
        0 => 0.5,
        1 => 1.0,
        2 => 44.0 / 33.0,
        3 => 1.5,
        4 => 2.0,
        5 => 78.0 / 33.0,
        _ => 1.0
    };
    rx_speed = 12.0 * f64::log2(rx_speed);

    let rx_finetune = 2.0 * rx_finetune - 1.0;

    let total = rx_pitch + rx_speed + rx_finetune;
    let td_tune = (total.round() + 48.0) / 96.0;
    let td_finetune = total.fract() + 0.5;

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