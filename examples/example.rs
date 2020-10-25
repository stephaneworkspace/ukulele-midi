use chartgeneratorsvg::interface::InterfaceWasm;
use chartgeneratorsvg::interface::TraitChord;
use ghakuf::messages::*;
//use ghakuf::reader::*;
//use ghakuf::writer::*; // In version 0.5.6 only wirte in harddrive, i need to write in file
//use std::path;
use ukulele_midi::ghakuf_customize::writer::*;
//use ukulele_midi::hodge::HogeHandler;
use ukulele_midi::ukulele::{ArpPatern, Arpegiator, Chord, Ukulele};

fn ext() -> Vec<u8> {
    InterfaceWasm::chord_list_experimental("F", "m", 0 as u8)
    //       .iter()
    //       .map(|x| x - 24)
    //       .collect()
}

fn main() {
    let mut midi: Midi = Midi {
        data: &mut Vec::new(),
    };
    match midi.generate_midi() {
        Ok(()) => println!("Ok {:?}", midi.data),
        Err(err) => panic!("Error: {}", err),
    };
}

pub struct Midi<'a> {
    pub data: &'a mut Vec<u8>,
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
}
