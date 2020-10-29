///! This library is a little ukulele synthetizer
#[macro_use]
extern crate log;

pub mod ghakuf_customize;
pub mod hodge;
pub mod synthrs_customize;
pub mod ukulele;

use base64::encode;
use ghakuf::messages::*;
use ghakuf_customize::writer::*;
use std::io::prelude::*; // dev dep
use std::io::Cursor;
use std::str::FromStr;
use synthrs::midi;
use synthrs::sample;
use synthrs::synthesizer::{make_samples_from_midi, quantize_samples};
use synthrs::wave;
use synthrs_customize::write_wav_buffer;
pub use ukulele::{ArpPatern, Arpegiator, Chord, Ukulele};

// Custom error for variant string
#[derive(Debug)]
pub struct ParseVariantError {
    pub name: String,
}

pub enum Variant {
    Chord,
    Arp8,
    Arp4,
}

impl FromStr for Variant {
    type Err = ParseVariantError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Variant::*;
        match s {
            "chord" => Ok(Chord),
            "arp8" => Ok(Arp8),
            "arp4" => Ok(Arp4),
            _ => Err(ParseVariantError {
                name: s.to_string(),
            }),
        }
    }
}

pub struct SoundBytes<'a> {
    pub semitones_midi: &'a [u8],
    pub midi: &'a mut Vec<u8>,
    pub wav: &'a mut Vec<u8>,
}

impl<'a> SoundBytes<'a> {
    /// Generate midi + wav in reference from extern sample
    pub fn generate_from_sample_base64(
        &mut self,
        variant: Variant,
        sample_ukulele: Box<[u8]>,
    ) -> Result<(), std::io::Error> {
        match self.generate_midi(variant) {
            Ok(()) => self.generate_wav_from_buffer(sample_ukulele.to_vec()),
            Err(err) => Err(err),
        }
    }

    /// Generate base64 for the waveform of the sampled ukulele C note
    /// Dev-depency, in wasm (for example) is not so easy for acces to assets
    pub fn generate_sample_base64(&self) -> std::io::Result<()> {
        let path = std::path::Path::new("assets/ukulele-a-440.wav");
        let file = std::fs::File::open(path)?;
        let mut reader = std::io::BufReader::new(file);
        //let mut buffer: Vec<u8> = Vec::new();
        //reader.read_to_end(&mut buffer)?;
        //println!("{:?}", encode(&reader.buffer()));

        let mut out = Vec::new();
        reader.read_to_end(&mut out).unwrap();

        println!("{}", encode(&out));

        //reader.flush()?;
        Ok(())
    }

    pub fn encode_base64_wav(&self) -> String {
        format!("data:audio/wav;base64,{}", encode(&self.wav))
    }

    /// Generate midi in reference
    fn generate_midi(
        &mut self,
        variant: Variant,
    ) -> Result<(), std::io::Error> {
        // sample messages
        let tempo: u32 = 60 * 1000000 / 102; // bpm: 102
        let mut write_messages: Vec<Message> = Vec::new();
        write_messages.push(Message::MetaEvent {
            delta_time: 0,
            event: MetaEvent::SetTempo,
            data: [(tempo >> 16) as u8, (tempo >> 8) as u8, tempo as u8]
                .to_vec(),
        });
        write_messages.push(Message::MetaEvent {
            delta_time: 0,
            event: MetaEvent::EndOfTrack,
            data: Vec::new(),
        });
        write_messages.push(Message::TrackChange);
        let ukulele = Ukulele::new(&self.semitones_midi[..]);
        match variant {
            Variant::Chord => {
                write_messages.append(&mut ukulele.chord());
            }
            Variant::Arp8 => {
                write_messages.append(
                    &mut ukulele
                        .arp(ArpPatern::OneThreeTwoThreeFourThreeTwo, 4),
                );
            }
            Variant::Arp4 => {
                write_messages
                    .append(&mut ukulele.arp(ArpPatern::OneTwoThreeFour, 4));
            }
        }
        write_messages.push(Message::MetaEvent {
            delta_time: 0,
            event: MetaEvent::EndOfTrack,
            data: Vec::new(),
        });
        let mut writer = Writer::new();
        writer.running_status(true);
        for message in &write_messages {
            writer.push(&message);
        }
        Ok(writer.write_buffer(&mut self.midi)?)
    }

    /*
    pub fn compress(&self) {
        let mut input = BufReader::new(
            std::fs::File::open("assets/ukulele-a-440.wav").unwrap(),
        );
        let output = std::fs::File::create("ukulele.gz").unwrap();
        let mut encoder = flate2::write::GzEncoder::new(
            output,
            flate2::Compression::default(),
        );
        let start = std::time::Instant::now();
        std::io::copy(&mut input, &mut encoder).unwrap();
        let output = encoder.finish().unwrap();
        println!(
            "Source len: {:?}",
            input.get_ref().metadata().unwrap().len()
        );
        println!("Target len: {:?}", output.metadata().unwrap().len());
        println!("Elapsed: {:?}", start.elapsed());
    }*/

    fn generate_wav_from_buffer(
        &mut self,
        sample: Vec<u8>,
    ) -> Result<(), std::io::Error> {
        let midi_u8: &[u8] = &self.midi;
        let mut cursor = Cursor::new(midi_u8);

        let song = midi::read_midi(&mut cursor).unwrap();

        let (ukulele_sample, ukulele_sample_len) =
            sample::samples_from_wave_bytes(sample).unwrap();
        let ukulele_sampler = |frequency: f64| {
            wave::sampler(
                frequency,
                &ukulele_sample,
                ukulele_sample_len,
                440.0,
                44_100,
            )
        };
        write_wav_buffer(
            &mut self.wav,
            44_100,
            &quantize_samples::<i16>(
                &make_samples_from_midi(ukulele_sampler, 44_100, false, song)
                    .unwrap(),
            ),
        )
        .expect("failed"); // TODO better
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
