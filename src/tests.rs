#[cfg(test)]
mod tests {
    use std::path;
    use crate::*;

    #[test]
    fn pad_id_to_index_test() {
        let pad_ids = [
            "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "b1", "b2", "b3", "b4", "b5", "b6",
            "b7", "b8", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "d1", "d2", "d3", "d4",
            "d5", "d6", "d7", "d8",
        ];
        for (index, &pad_id) in pad_ids.iter().enumerate() {
            assert_eq!(intermediate::pad_id_to_index(pad_id), Some(index));
        }
    }

    #[test]
    fn atest() {
        use std::{env, fs, path::PathBuf};
        // println!("{}", 0.1+0.2);
        // let path = Path::new(r"C:\Users\Nadav\Desktop\rtxd\spikes.wav");
        let mut programdata_folder = env::var("PROGRAMDATA")
            .expect("Failed to get ProgramData folder");
        let mut path = PathBuf::from(&programdata_folder);
        path.push(r"Inphonik\RX1200\");
        // programdata_folder.push_str(r"\Inphonik\RX1200");
        eprintln!("\n\tProgramData folder: {}\n", path.display());
    }
}
