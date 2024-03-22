mod file_import;
mod analyser;
use fundsp::hacker32::*;
use pitch_detection::*;
use std::time::{Duration, Instant};



fn main() {
    let wav:Wave32 =  file_import::import_wav32("./assets/sound examples/amelioratorOfficialSoundtrack.wav");

    let start = Instant::now();
    let (notes, times) = analyser::tonal_analysis::get_fundamental_notes(&wav, 0.2);
    let duration = start.elapsed();
    for i in 0..(notes).len(){

        println!("{}, {:.1}s", notes[i], times[i]);

    }
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
