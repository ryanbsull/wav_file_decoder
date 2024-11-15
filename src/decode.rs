use std::fs::{self, File};
use std::io::prelude::*;

// WAVE PCM soundfile format reference: http://soundfile.sapp.org/doc/WaveFormat/

struct WaveForm {
    riff: u32, // == 0x52494646 big-endian form 'RIFF'
    /*
        chunk_size == 36 + SubChunk2Size, or more precisely:
        chunk_size == 4 + (8 + SubChunk1Size) + (8 + SubChunk2Size)
        This is the size of the rest of the chunk
        following this number.  This is the size of the
        entire file in bytes minus 8 bytes for the
        two fields not included in this count:
        ChunkID and ChunkSize.
    */
    chunk_size: u32,
    fmt: u32, // == 0x57415645 big-endian form 'WAVE'
    fmt_ck: WaveFmtChunk,
    wave_data: WaveData,
}

struct WaveFmtChunk {
    id: u32,           // Contains the letters "fmt " == 0x666d7420 big-endian form
    size: u32,         // 16 for PCM
    audio_fmt: u16,    // PCM = 1 (if != 1 then indicates compression)
    num_channels: u16, // Mono = 1, Stereo = 2
    sample_rate: u32,  // number of samples per second
    byte_rate: u32,    // == sample_rate * num_channels * bits_per_sample / 8
    blk_align: u16,    // == num_channels * bits_per_sample / 8
    bits_per_sample: u16,
}

struct WaveData {
    id: u32,       // contains letters "data" == 0x64617461 big-endian form
    size: u32,     // == num_samples * num_channels * bits_per_sample / 8
    data: Vec<u8>, // the actual data: see size for number of bytes to read
}

pub fn check_wav_file_format(path: &str) -> Result<(), std::io::Error> {
    Ok(())
}

pub fn decode_wav_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    Ok(Vec::new())
}
