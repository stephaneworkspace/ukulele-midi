//! Writes audio samples to files. (Reads too, this module needs a rename)
//!
//! ### Audacity PCM import settings
//!
//! * File > Import > Raw Data...
//! * Signed 16-bit PCM
//! * Little-endian
//! * 1 Channel (Mono)
//! * Sample rate: 44_100Hz (or whatever your samples generated have)

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::io::{Cursor, Result, Write};
//use synthrs::writer::write_wav;

/// Creates a buffer reference `data` and writes a bunch of `&[i16]` samples to it as a WAVE file
/// ```
/// //use synthrs::wave::sine_wave;
/// //use synthrs::writer::write_wav_file;
/// //use synthrs::synthesizer::{quantize_samples, make_samples};
///
/// //write_wav_file(
/// //    "out/sine.wav",
/// //    44_100,
/// //    &quantize_samples::<i16>(&make_samples(0.1, 44_100, sine_wave(440.0))),
/// //).expect("failed to write wav");
/// ```
pub fn write_wav_buffer(
    data: &mut Vec<u8>,
    sample_rate: usize,
    samples: &[i16],
) -> Result<()> {
    let mut c = Cursor::new(&mut *data);
    write_wav(&mut c, sample_rate, samples)
}

/// Writes a bunch of `&[i16]` samples to a `Write`. Also see `synthrs::writer::write_wav_file`.
/// ```
/// //use std::io::Cursor;
/// //use synthrs::wave::sine_wave;
/// //use synthrs::writer::write_wav;
/// //use synthrs::synthesizer::{quantize_samples, make_samples};
///
/// //let mut output_buffer: Vec<u8> = Vec::new();
/// //let mut output_writer = Cursor::new(output_buffer);
///
/// // You can use whatever implements `Write`, such as a `File`.
/// //write_wav(
/// //    &mut output_writer,
/// //    44_100,
/// //    &quantize_samples::<i16>(&make_samples(0.1, 44_100, sine_wave(440.0))),
/// //).expect("failed to write wav");
/// ```
pub fn write_wav<W>(
    writer: &mut W,
    sample_rate: usize,
    samples: &[i16],
) -> Result<()>
where
    W: Write,
{
    // See: http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html
    // Some WAV header fields
    let channels = 1;
    let bit_depth = 16;
    let subchunk_2_size = samples.len() * channels * bit_depth / 8;
    let chunk_size = 36 + subchunk_2_size as i32;
    let byte_rate = (sample_rate * channels * bit_depth / 8) as i32;
    let block_align = (channels * bit_depth / 8) as i16;

    writer.write_i32::<BigEndian>(0x5249_4646)?; // ChunkID, RIFF
    writer.write_i32::<LittleEndian>(chunk_size)?; // ChunkSize
    writer.write_i32::<BigEndian>(0x5741_5645)?; // Format, WAVE

    writer.write_i32::<BigEndian>(0x666d_7420)?; // Subchunk1ID, fmt
    writer.write_i32::<LittleEndian>(16)?; // Subchunk1Size, 16 for PCM
    writer.write_i16::<LittleEndian>(1)?; // AudioFormat, PCM = 1 (linear quantization)
    writer.write_i16::<LittleEndian>(channels as i16)?; // NumChannels
    writer.write_i32::<LittleEndian>(sample_rate as i32)?; // SampleRate
    writer.write_i32::<LittleEndian>(byte_rate)?; // ByteRate
    writer.write_i16::<LittleEndian>(block_align)?; // BlockAlign
    writer.write_i16::<LittleEndian>(bit_depth as i16)?; // BitsPerSample

    writer.write_i32::<BigEndian>(0x6461_7461)?; // Subchunk2ID, data
    writer.write_i32::<LittleEndian>(subchunk_2_size as i32)?; // Subchunk2Size, number of bytes in the data

    for sample in samples {
        writer.write_i16::<LittleEndian>(*sample)?
    }

    Ok(writer.flush()?)
}
