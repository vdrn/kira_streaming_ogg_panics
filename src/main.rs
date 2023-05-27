use std::{thread::sleep, time::Duration};

use kira::sound::streaming::{StreamingSoundData, StreamingSoundSettings};
use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

fn main() {
    const PANICS: &str = "in_the_end.ogg";
    const DOES_NOT_PANIC: &str = "Tribesmen.mp3";

    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
    let sound_data =
        StreamingSoundData::from_file(PANICS, StreamingSoundSettings::default()).unwrap();
    manager.play(sound_data).unwrap();

    for i in 0..10 {
        sleep(Duration::from_secs(2));
    }
}
