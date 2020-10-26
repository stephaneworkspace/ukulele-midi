extern crate sdl2;

use std::thread;

use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioSpecWAV};
//use sdl2::rwops::RWops;

use chartgeneratorsvg::interface::InterfaceWasm;
use chartgeneratorsvg::interface::TraitChord;
use ghakuf::messages::*;
//use ghakuf::reader::*;
//use ghakuf::writer::*; // In version 0.5.6 only wirte in harddrive, i need to write in file
//use std::path;
use ukulele_midi::ghakuf_customize::writer::*;
//use ukulele_midi::hodge::HogeHandler;
use ukulele_midi::ukulele::{ArpPatern, Arpegiator, Chord, Ukulele};

use std::io::Cursor;
use synthrs::midi;
use synthrs::synthesizer::{make_samples_from_midi, quantize_samples};
use synthrs::wave;
//use synthrs::writer::write_wav_file;
use ukulele_midi::synthrs_customize::write_wav_buffer;

fn ext() -> Vec<u8> {
    InterfaceWasm::chord_list_experimental("F", "m", 0 as u8)
    //       .iter()
    //       .map(|x| x - 24)
    //       .collect()
}

fn main() {
    let mut midi: Midi = Midi {
        data: &mut Vec::new(),
        wav: &mut Vec::new(),
    };
    match midi.generate_midi() {
        Ok(()) => println!("Ok"),
        Err(err) => panic!("Error: {}", err),
    };
    match midi.generate_wav() {
        Ok(()) => println!("Ok"),
        Err(err) => panic!("Error: {}", err),
    };

    /*write_wav_file(
        "examples/example.wav",
        44_100,
        &quantize_samples::<i16>(
            &make_samples_from_midi(wave::square_wave, 44_100, false, song)
                .unwrap(),
        ),
    )
    .expect("failed");*/

    let sdl_context = sdl2::init().unwrap();
    let audio_system = sdl_context.audio().unwrap();

    let audio_spec = AudioSpecDesired {
        freq: None,
        channels: None,
        samples: None,
    };
    //let rw_ops = RWops::from_bytes(&midi.wav).unwrap(); // TODO .?
    //let audio_wav = AudioSpecWAV::load_wav_rw(&_rw_ops).unwrap();

    let copied_data = CopiedData {
        bytes: sdl2::audio::AudioCVT::new(
            sdl2::audio::AudioFormat::S16LSB, //wav.format,
            1,                                //wav.channels,
            44100,                            //wav.freq,
            sdl2::audio::AudioFormat::S16LSB, //spec.format,
            1,                                //spec.channels,
            44100,                            //spec.freq,
        )
        .unwrap()
        .convert(midi.wav[..].to_vec()),
        //bytes: audio_wav.buffer().to_vec(),
        position: 0,
    };
    //let wrapped_data = WrappedData{ audio: audio_wav, position: 0 };

    let audio_device = audio_system
        .open_playback(None, &audio_spec, move |_spec| copied_data)
        .unwrap();

    audio_device.resume();
    thread::sleep(std::time::Duration::new(5000, 0));
}

pub struct Midi<'a> {
    pub data: &'a mut Vec<u8>,
    pub wav: &'a mut Vec<u8>,
}

impl<'a> Midi<'a> {
    fn generate_midi(&mut self) -> Result<(), std::io::Error> {
        // sample messages
        let tempo: u32 = 60 * 1000000 / 102; //bpm:102
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
        let ukulele = Ukulele::new(ext());
        write_messages.append(&mut ukulele.chord());
        write_messages.append(
            &mut ukulele.arp(ArpPatern::OneThreeTwoThreeFourThreeTwo, 4),
        );
        write_messages.append(&mut ukulele.arp(ArpPatern::OneTwoThreeFour, 4));
        write_messages.push(Message::MetaEvent {
            delta_time: 0,
            event: MetaEvent::EndOfTrack,
            data: Vec::new(),
        });
        //let mut read_messages: Vec<Message> = Vec::new();

        // build example
        //{
        //let path = path::Path::new("examples/example.mid");
        let mut writer = Writer::new();
        writer.running_status(true);
        for message in &write_messages {
            writer.push(&message);
        }
        //let _ = writer.write(&path);
        //let data: &mut Vec<u8> = &mut Vec::new();
        //writer.write_buffer(data)?;
        Ok(writer.write_buffer(&mut self.data)?)
        /*
        match writer.write_buffer(&mut self.data) {
            Ok(()) => {
                self.data = data;
                Ok(())
            }
            Err(err) => Err(err),
        }*/
        //}

        // parse example
        /*{
            let path = path::Path::new("examples/example.mid");
            let mut handler = HogeHandler {
                messages: &mut read_messages,
            };
            let mut reader = Reader::new(&mut handler, &path).unwrap();
            let _ = reader.read();
        }*/

        // result check
        /*if write_messages == read_messages {
            println!("Correct I/O has done!");
        }*/

        // Test Fm
        // assert_eq!(ext(), vec![0x2c, 0x24, 0x29, 0x30]);
    }
    fn generate_wav(&mut self) -> Result<(), std::io::Error> {
        let midi_u8: &[u8] = &self.data;
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

//----------------------------------------------------------------------------//

struct CopiedData {
    bytes: Vec<u8>,
    position: usize,
}

impl AudioCallback for CopiedData {
    type Channel = u8;

    fn callback(&mut self, data: &mut [u8]) {
        let (start, end) = (self.position, self.position + data.len());
        self.position += data.len();

        let audio_data = &self.bytes[start..end];
        for (src, dst) in audio_data.iter().zip(data.iter_mut()) {
            *dst = *src;
        }
    }
}

//----------------------------------------------------------------------------//

struct WrappedData {
    audio: AudioSpecWAV,
    position: usize,
}

impl AudioCallback for WrappedData {
    type Channel = u8;

    fn callback(&mut self, data: &mut [u8]) {
        let (start, end) = (self.position, self.position + data.len());
        self.position += data.len();

        let audio_data = &self.audio.buffer()[start..end];
        for (src, dst) in audio_data.iter().zip(data.iter_mut()) {
            *dst = *src;
        }
    }
}

unsafe impl Send for WrappedData {}
