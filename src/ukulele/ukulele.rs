use ghakuf::messages::*;
const INTERVAL: u32 = 192;

pub struct Ukulele {
    semitones: Vec<u8>,
}

impl Ukulele {
    pub fn new(semitones: Vec<u8>) -> Self {
        Self { semitones }
    }
}

pub trait Chord {
    fn chord(&self) -> Vec<Message>;
}

impl Chord for Ukulele {
    fn chord(&self) -> Vec<Message> {
        let mut write_messages: Vec<Message> = Vec::new();
        for (i, s) in self.semitones.iter().enumerate() {
            write_messages.push(Message::MidiEvent {
                delta_time: 0,
                event: MidiEvent::NoteOn {
                    ch: i as u8,
                    note: s.clone(),
                    velocity: 0x7f,
                },
            });
        }
        for (i, s) in self.semitones.iter().enumerate() {
            write_messages.push(Message::MidiEvent {
                delta_time: INTERVAL,
                event: MidiEvent::NoteOn {
                    ch: i as u8,
                    note: s.clone(),
                    velocity: 0,
                },
            });
        }
        write_messages
    }
}

pub enum ArpPatern {
    OneTwoThreeFour,
    OneThreeTwoThreeFourThreeTwo,
}

type UkuleleString = usize;

impl ArpPatern {
    fn pattern(&self) -> Vec<UkuleleString> {
        match self {
            ArpPatern::OneTwoThreeFour => vec![1, 2, 3, 4],
            ArpPatern::OneThreeTwoThreeFourThreeTwo => vec![1, 3, 2, 3, 4, 3, 2, 3],
        }
        .iter()
        .map(|x| x - 1)
        .collect()
    }
}

pub trait Arpegiator {
    fn arp(&self, pattern: ArpPatern, repeat: u32) -> Vec<Message>;
}

impl Arpegiator for Ukulele {
    fn arp(&self, pattern: ArpPatern, repeat: u32) -> Vec<Message> {
        let mut write_messages: Vec<Message> = Vec::new();
        for _ in 0..repeat {
            for ptn in pattern.pattern().iter() {
                for (_, semitones) in self.semitones.iter().enumerate().filter(|(x, _)| x == ptn) {
                    write_messages.push(Message::MidiEvent {
                        delta_time: 0,
                        event: MidiEvent::NoteOn {
                            ch: 0,
                            note: semitones.clone(),
                            velocity: 0x7f,
                        },
                    });
                    write_messages.push(Message::MidiEvent {
                        delta_time: INTERVAL,
                        event: MidiEvent::NoteOn {
                            ch: 0,
                            note: semitones.clone(),
                            velocity: 0,
                        },
                    });
                }
            }
        }
        write_messages
    }
}
