use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(value_name = "FILE")]
    pub wav_file: Option<String>,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
