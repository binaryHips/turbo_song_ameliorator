mod file_import;
mod analyser;
use fundsp::hacker32::*;
use pitch_detection::*;
use std::time::{Duration, Instant};



fn main() {
    let mut node = lowpass_hz(1000.0, 0.5);

    print!("Filter: lowpass_hz(1000.0, 0.5)\n\n");

    // The display method prints an ASCII oscilloscope and other information about the node.
    print!("{}", node.display());
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
    let volumes = analyser::volume_analysis::get_volumes_RMS(&wav, 2.0);
    let duration = start.elapsed();
    for i in 0..(volumes).len(){

        println!("{}, {:.1}s", volumes[i], i*2);

    }
    println!("Time elapsed : {:?}", duration);

}

