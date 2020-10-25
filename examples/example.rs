extern crate ghakuf;
use chartgeneratorsvg::interface::InterfaceWasm;
use chartgeneratorsvg::interface::TraitChord;
use ghakuf::messages::*;
use ghakuf::reader::*;
use ghakuf::writer::*;
use std::path;

fn ext() -> Vec<u8> {
    InterfaceWasm::chord_list_experimental("C", "m", 0 as u8)
}

fn main() {
    // sample messages
    let tempo: u32 = 60 * 1000000 / 102; //bpm:102
    let mut write_messages: Vec<Message> = Vec::new();
    write_messages.push(Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::SetTempo,
        data: [(tempo >> 16) as u8, (tempo >> 8) as u8, tempo as u8].to_vec(),
    });
    write_messages.push(Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::EndOfTrack,
        data: Vec::new(),
    });
    write_messages.push(Message::TrackChange);
    const INTERVAL: u32 = 192;
    for (i, semitones) in ext().iter().enumerate() {
        let delta_time = INTERVAL * i as u32;
        write_messages.push(Message::MidiEvent {
            delta_time,
            event: MidiEvent::NoteOn {
                ch: 0,
                note: semitones.clone(),
                velocity: 0x7f,
            },
        });
        let delta_time = INTERVAL * (i as u32 + 1);
        write_messages.push(Message::MidiEvent {
            delta_time,
            event: MidiEvent::NoteOn {
                ch: 0,
                note: semitones.clone(),
                velocity: 0,
            },
        });
    }
    write_messages.push(Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::EndOfTrack,
        data: Vec::new(),
    });
    let mut read_messages: Vec<Message> = Vec::new();

    // build example
    {
        let path = path::Path::new("examples/example.mid");
        let mut writer = Writer::new();
        writer.running_status(true);
        for message in &write_messages {
            writer.push(&message);
        }
        let _ = writer.write(&path);
    }

    // parse example
    {
        let path = path::Path::new("examples/example.mid");
        let mut handler = HogeHandler {
            messages: &mut read_messages,
        };
        let mut reader = Reader::new(&mut handler, &path).unwrap();
        let _ = reader.read();
    }

    // result check
    if write_messages == read_messages {
        println!("Correct I/O has done!");
    }
}

struct HogeHandler<'a> {
    messages: &'a mut Vec<Message>,
}
impl<'a> Handler for HogeHandler<'a> {
    fn header(&mut self, format: u16, track: u16, time_base: u16) {
        println!(
            "SMF format: {}, track: {}, time base: {}",
            format, track, time_base
        );
    }
    fn meta_event(&mut self, delta_time: u32, event: &MetaEvent, data: &Vec<u8>) {
        println!(
            "delta time: {:>4}, Meta event: {}, data: {:?}",
            delta_time, event, data
        );
        self.messages.push(Message::MetaEvent {
            delta_time: delta_time,
            event: event.clone(),
            data: data.clone(),
        });
    }
    fn midi_event(&mut self, delta_time: u32, event: &MidiEvent) {
        println!("delta time: {:>4}, MIDI event: {}", delta_time, event,);
        self.messages.push(Message::MidiEvent {
            delta_time: delta_time,
            event: event.clone(),
        });
    }
    fn sys_ex_event(&mut self, delta_time: u32, event: &SysExEvent, data: &Vec<u8>) {
        println!(
            "delta time: {:>4}, System Exclusive Event: {}, data: {:?}",
            delta_time, event, data
        );
        self.messages.push(Message::SysExEvent {
            delta_time: delta_time,
            event: event.clone(),
            data: data.clone(),
        });
    }
    fn track_change(&mut self) {
        // Excepts first track change (from format chunk to data chunk)
        if self.messages.len() > 0 {
            println!("Track change occcurs!");
            self.messages.push(Message::TrackChange)
        }
    }
}
