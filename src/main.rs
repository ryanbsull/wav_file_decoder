use std::process::exit;

mod cli;
mod decode;

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
        println!("{:?}", file_contents);
    } else {
        println!("Could not read file");
        exit(1);
    }
}
