
mod utils; //utility functions. Contains notes definitions.

mod analyser;
mod generator;

use fundsp::hacker32::*;
use pitch_detection::*;
use std::time::{Duration, Instant};
use utils::file_import;


fn main() {
    let (bytes,file) = utils::file_import::import_midi("/home/e20210002460/turbo_song_ameliorator/turbo_song_ameliorator/assets/melody/crypt_of_the_necrodancer_disco_descent.mid");
    let v = utils::analysis_file::get_bpm(&file);
    print!("{}", v);
    //test_pitch_detection();
    //test_rhythm();
}


fn test_pitch_detection() {

    let wav:Wave32 =  file_import::import_wav32("./assets/sound examples/EDAC 60.wav");

    let start = Instant::now();
    let (notes, times) = analyser::tonal_analysis::get_fundamental_notes(&wav, 1.0);
    let duration = start.elapsed();
    for i in 0..(notes).len(){

        println!("{}, {:.1}s", notes[i], times[i]);

    }
    println!("Time elapsed : {:?}", duration);
}

fn test_volume_detection() {

    let wav:Wave32 =  file_import::import_wav32("./assets/sound examples/EDAC 60.wav");

    let start = Instant::now();
    let volumes = analyser::volume_analysis::get_volumes_RMS_3bands(&wav, 2.0);
    let duration = start.elapsed();
    for i in 0..(volumes).len(){

        println!("{:?}, {:.1}s", volumes[i], i*2);

    }
    println!("Time elapsed : {:?}", duration);

}




fn test_lowpass(){

    let wav:Wave32 =  file_import::import_wav32("./assets/sound examples/EDAC 60.wav");

    let mut node = lowpass_hz(500.0, 1.0) | lowpass_hz(500.0, 1.0);
    let res = wav.filter(10.0, &mut node);

    file_import::save_wav32(&res, "./assets/sound examples/EDAC_60_LOWPASSED.wav");
}


fn test_rhythm()
{
    let wav:Wave32 = file_import::import_wav32("./assets/sound examples/amelioratorOfficialSoundtrack.wav");
    analyser::rhythm_analysis::test(&wav);
}