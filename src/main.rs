// #![allow(warnings, unused)]

use rxtd::*;

fn main() {
//     let title = "
// ██████╗ ████████╗██╗  ██╗██████╗      ██████╗ ██████╗ ███╗   ██╗██╗   ██╗███████╗██████╗ ████████╗
// ██╔══██╗╚══██╔══╝╚██╗██╔╝██╔══██╗    ██╔════╝██╔═══██╗████╗  ██║██║   ██║██╔════╝██╔══██╗╚══██╔══╝
// ██████╔╝   ██║    ╚███╔╝ ██║  ██║    ██║     ██║   ██║██╔██╗ ██║██║   ██║█████╗  ██████╔╝   ██║   
// ██╔══██╗   ██║    ██╔██╗ ██║  ██║    ██║     ██║   ██║██║╚██╗██║╚██╗ ██╔╝██╔══╝  ██╔══██╗   ██║   
// ██║  ██║   ██║   ██╔╝ ██╗██████╔╝    ╚██████╗╚██████╔╝██║ ╚████║ ╚████╔╝ ███████╗██║  ██║   ██║   
// ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═════╝      ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝   ╚═╝   
// ";
//     println!("{title}");

    let (input_directory, output_directory) = config::run_configuration();
    
    let rx_files = match collect_rx_files(&input_directory) {
        Ok(files) => files,
        Err(error) => {
            eprintln!("Whoops! {error}");
            return
        }
    };

    let number_of_presets = rx_files.len();

    if number_of_presets > 0 {
        println!("Found {number_of_presets} RX1200 presets 😎");
        println!("Let's go!");
        for rx_file in rx_files {
            if let Err(error) = convert_preset(&rx_file, &output_directory) {
                eprintln!("Failed to convert {}: {error}", rx_file.display());
            }
        }
        println!("Done!");
    } else {
        println!("No RX1200 presets found in input directory 😮");
    }

    println!("Enjoy the rest of your day 🥰");
    println!();
}

