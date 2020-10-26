#[macro_use]
extern crate log;

pub mod ghakuf_customize;
pub mod hodge;
pub mod synthrs_customize;
pub mod ukulele;

use base64::encode;
use ghakuf::messages::*;
use ghakuf_customize::writer::*;
use std::io::Cursor;
use std::str::FromStr;
use synthrs::midi;
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
    /// Generate midi + wav in reference
    pub fn generate(&mut self, variant: Variant) -> Result<(), std::io::Error> {
        match self.generate_midi(variant) {
            Ok(()) => self.generate_wav(),
            Err(err) => Err(err),
        }
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

    /// Generate wav in reference
    fn generate_wav(&mut self) -> Result<(), std::io::Error> {
        let midi_u8: &[u8] = &self.midi;
        let mut cursor = Cursor::new(midi_u8);

        let song = midi::read_midi(&mut cursor).unwrap();

        write_wav_buffer(
            &mut self.wav,
            44_100,
            &quantize_samples::<i16>(
                &make_samples_from_midi(wave::square_wave, 44_100, false, song)
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
