use byteorder::{ByteOrder, LittleEndian};
use core::fmt;
use std::fs;
use std::str::from_utf8;

/*
    WAVE PCM soundfile format references:
        - http://soundfile.sapp.org/doc/WaveFormat/
        - https://www.mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html
*/

#[allow(dead_code)]
#[derive(Debug)]
pub struct WaveForm {
    riff: [u8; 4], // == 0x52494646 big-endian form 'RIFF'
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
    fmt: [u8; 4], // == 0x57415645 big-endian form 'WAVE'
    fmt_ck: WaveFmtChunk,
    pub wave_data: WaveData,
}

#[allow(dead_code)]
#[derive(Debug)]
struct WaveFmtChunk {
    id: [u8; 4],       // Contains the letters "fmt " == 0x666d7420 big-endian form
    size: u32,         // 16 for PCM
    audio_fmt: u16,    // PCM = 1 (if != 1 then indicates compression)
    num_channels: u16, // Mono = 1, Stereo = 2
    sample_rate: u32,  // number of samples per second
    byte_rate: u32,    // == sample_rate * num_channels * bits_per_sample / 8
    blk_align: u16,    // == num_channels * bits_per_sample / 8
    bits_per_sample: u16,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WaveData {
    id: [u8; 4],       // contains letters "data" == 0x64617461 big-endian form
    pub size: u32,     // == num_samples * num_channels * bits_per_sample / 8
    pub data: Vec<u8>, // the actual data: see size for number of bytes to read
}

impl WaveForm {
    fn new(file_contents: &Vec<u8>) -> Self {
        WaveForm {
            riff: file_contents[0..4].try_into().unwrap(),
            chunk_size: LittleEndian::read_u32(&file_contents[4..8]),
            fmt: file_contents[8..12].try_into().unwrap(),
            fmt_ck: WaveFmtChunk::new(&file_contents[12..36]),
            wave_data: WaveData::new(&file_contents[36..]),
        }
    }
}

impl WaveFmtChunk {
    fn new(fmt_chunk: &[u8]) -> Self {
        WaveFmtChunk {
            id: fmt_chunk[0..4].try_into().unwrap(),
            size: LittleEndian::read_u32(&fmt_chunk[4..8]),
            audio_fmt: LittleEndian::read_u16(&fmt_chunk[8..10]),
            num_channels: LittleEndian::read_u16(&fmt_chunk[10..12]),
            sample_rate: LittleEndian::read_u32(&fmt_chunk[12..16]),
            byte_rate: LittleEndian::read_u32(&fmt_chunk[16..20]),
            blk_align: LittleEndian::read_u16(&fmt_chunk[20..22]),
            bits_per_sample: LittleEndian::read_u16(&fmt_chunk[22..24]),
        }
    }
}

impl WaveData {
    fn new(data_chunk: &[u8]) -> Self {
        WaveData {
            id: data_chunk[0..4].try_into().unwrap(),
            size: LittleEndian::read_u32(&data_chunk[4..8]),
            data: Vec::from(&data_chunk[8..]),
        }
    }
}

impl fmt::Display for WaveForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WaveForm{{\n\t'{}',\n\tchunk_size: {} bytes,\n\tfmt: {},\n{},\n{}\n}}",
            from_utf8(&self.riff).unwrap(),
            &self.chunk_size,
            from_utf8(&self.fmt).unwrap(),
            &self.fmt_ck,
            &self.wave_data
        )
    }
}

impl fmt::Display for WaveFmtChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "WaveFmtChunk{{\n\t'{}',\n\tsize: {},\n\taudio_fmt: {},\n\tnum_channels: {},\n\tsample_rate: {},\n\tbyte_rate: {} byte/sec,\n\tblock_align: {},\n\tbits_per_sample: {}\n}}",
            from_utf8(&self.id).unwrap(),
            &self.size,
            &self.audio_fmt,
            &self.num_channels,
            &self.sample_rate,
            &self.byte_rate,
            &self.blk_align,
            &self.bits_per_sample
        )
    }
}

impl fmt::Display for WaveData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WaveData{{\n\t'{}',\n\t{},\n\t[wave_data]\n}}",
            from_utf8(&self.id).unwrap(),
            &self.size
        )
    }
}

fn check_wav_file_format(file_contents: &Vec<u8>) -> Result<(), std::io::Error> {
    let riff = match from_utf8(&file_contents[0..4]) {
        Ok(s) => s,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let wave = match from_utf8(&file_contents[8..12]) {
        Ok(s) => s,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let fmt = match from_utf8(&file_contents[12..16]) {
        Ok(s) => s,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    if riff != "RIFF" || wave != "WAVE" || fmt != "fmt " {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid File format (expects .wav)",
        ));
    }

    Ok(())
}

pub fn decode_wav_file(path: &str) -> Result<WaveForm, std::io::Error> {
    let file_contents = fs::read(path)?;
    check_wav_file_format(&file_contents)?;
    Ok(WaveForm::new(&file_contents))
}
