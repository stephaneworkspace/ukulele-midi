use ghakuf::messages::*;
use ghakuf::reader::*;

pub struct HogeHandler<'a> {
    pub messages: &'a mut Vec<Message>,
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
            delta_time,
            event: event.clone(),
            data: data.clone(),
        });
    }
    fn midi_event(&mut self, delta_time: u32, event: &MidiEvent) {
        println!("delta time: {:>4}, MIDI event: {}", delta_time, event,);
        self.messages.push(Message::MidiEvent {
            delta_time,
            event: event.clone(),
        });
    }
    fn sys_ex_event(&mut self, delta_time: u32, event: &SysExEvent, data: &Vec<u8>) {
        println!(
            "delta time: {:>4}, System Exclusive Event: {}, data: {:?}",
            delta_time, event, data
        );
        self.messages.push(Message::SysExEvent {
            delta_time,
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
