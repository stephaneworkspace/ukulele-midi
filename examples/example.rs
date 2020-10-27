use ukulele_midi::{SoundBytes, Variant};

fn main() {
    let mut sb: SoundBytes = SoundBytes {
        semitones_midi: &[0x60, 0x61, 0x62, 0x63],
        midi: &mut Vec::new(),
        wav: &mut Vec::new(),
    };
    match sb.generate_from_files(Variant::Arp8) {
        Ok(()) => println!(
            "<html><body><audio controls src=\"{}\" /></body></html>",
            sb.encode_base64_wav()
        ),
        Err(err) => panic!("Error: {}", err),
    };
}
