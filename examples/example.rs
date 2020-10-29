use ukulele_midi::{SoundBytes, Variant};

fn main() {
    let mut sb: SoundBytes = SoundBytes {
        semitones_midi: &[0x60, 0x61, 0x62, 0x63], // Midi hexa value for play note
        midi: &mut Vec::new(),
        wav: &mut Vec::new(),
    };
    match sb.generate_from_local_asset(Variant::Arp8) {
        Ok(()) => {
            println!(
                "Buffer len: {}, Buffer {:?}",
                sb.midi.len(),
                sb.midi.to_vec()
            );
        }
        Err(err) => panic!("Error: {}", err),
    };
}
