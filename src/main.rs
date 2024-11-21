use std::process::exit;

mod cli;
mod decode;
mod display;
mod encode;

fn main() {
    let cli = cli::parse_args();
    let wav_file: String;

    if let Some(f) = cli.wav_file.as_deref() {
        wav_file = String::from(f);
    } else {
        println!("No file provided");
        exit(1);
    }

    if let Ok(file_contents) = decode::decode_wav_file(&wav_file) {
        let float_arr: Vec<f64> = file_contents
            .wave_data
            .data
            .iter()
            .map(|&e| e as f64)
            .collect();
        println!("{:?}", encode::dct(&float_arr));
        display::show_wav(&file_contents);
    } else {
        println!("Could not read file");
        exit(1);
    }
}
