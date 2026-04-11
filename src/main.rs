// use rxtd::*;
#![allow(warnings, unused)]
use rxtd::{collect_rx_files, config, convert_preset};
use colored::Colorize;

fn main() -> anyhow::Result<()> {
    
    println!();
    println!("{}", TITLE.red().bold());
    println!("{}", SUBTITLE.red().bold());
    println!();

    let (input_directory, output_directory) = config::run_configuration()?;
    
    let rx_files = collect_rx_files(&input_directory)?;

    let number_of_presets = rx_files.len();

    if number_of_presets > 0 {
        println!("Found {number_of_presets} RX1200 presets 😎");
        println!("{}", "Let's go!".bold().italic());
        println!();

        for rx_file in rx_files {
            convert_preset(&rx_file, &output_directory)?;
        }

        println!();
        println!("{}", "Done!".bold().italic());
    } else {
        println!("No RX1200 presets found in input directory 😮");
    }

    println!("Enjoy the rest of your day 🥰");
    println!();
    Ok(())
}



const TITLE: &str = "\
██████╗ ██╗  ██╗████████╗██████╗      ██████╗ ██████╗ ███╗   ██╗██╗   ██╗███████╗██████╗ ████████╗
██╔══██╗╚██╗██╔╝╚══██╔══╝██╔══██╗    ██╔════╝██╔═══██╗████╗  ██║██║   ██║██╔════╝██╔══██╗╚══██╔══╝
██████╔╝ ╚███╔╝    ██║   ██║  ██║    ██║     ██║   ██║██╔██╗ ██║██║   ██║█████╗  ██████╔╝   ██║   
██╔══██╗ ██╔██╗    ██║   ██║  ██║    ██║     ██║   ██║██║╚██╗██║╚██╗ ██╔╝██╔══╝  ██╔══██╗   ██║   
██║  ██║██╔╝ ██╗   ██║   ██████╔╝    ╚██████╗╚██████╔╝██║ ╚████║ ╚████╔╝ ███████╗██║  ██║   ██║   
╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   ╚═════╝      ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝   ╚═╝   \
";

const SUBTITLE: &str = "\
╔════════════════════════════════════════════════════════════════════════════════════════════════╗
║                     Convert RX1200 presets to TAL-Drum for FUN and PROFIT                      ║
╚════════════════════════════════════════════════════════════════════════════════════════════════╝\
";