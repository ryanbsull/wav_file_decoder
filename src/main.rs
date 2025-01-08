use num::complex::Complex;
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
        let mut complex_vec: Vec<Complex<f64>> = vec![];
        for sample in &file_contents.wave_data.data {
            complex_vec.push(Complex::new((*sample).into(), 0.0));
        }
        encode::fft(&mut complex_vec);
        println!("{:?}", &complex_vec);
        display::show_wav(&file_contents);
    } else {
        println!("Could not read file");
        exit(1);
    }
}
