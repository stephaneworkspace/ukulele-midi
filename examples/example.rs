use chartgeneratorsvg::interface::InterfaceWasm;
use chartgeneratorsvg::interface::TraitChord;
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioSpecWAV};
use std::thread;
use ukulele_midi::SoundBytes;

fn ext() -> Vec<u8> {
    InterfaceWasm::chord_list_experimental("F", "m", 0 as u8)
    //       .iter()
    //       .map(|x| x - 24)
    //       .collect()
}

fn main() {
    let mut sb: SoundBytes = SoundBytes {
        semitones_midi: &ext()[..],
        midi: &mut Vec::new(),
        wav: &mut Vec::new(),
    };
    match sb.generate() {
        Ok(()) => println!("Ok"),
        Err(err) => panic!("Error: {}", err),
    };

    // println!("{}", sb.encode_base64_wav());

    let sdl_context = sdl2::init().unwrap();
    let audio_system = sdl_context.audio().unwrap();

    let audio_spec = AudioSpecDesired {
        freq: None,
        channels: None,
        samples: None,
    };

    // Add this code audio convert (AudioCVT)for remove distorion
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
        .convert(sb.wav[..].to_vec()),
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
