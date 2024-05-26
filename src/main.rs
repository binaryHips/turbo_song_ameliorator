
mod utils; //utility functions. Contains notes definitions.

mod analyser;
mod generator;

use fundsp::hacker32::*;
use pitch_detection::*;
use std::time::{Duration, Instant};
use utils::file_import;
use utils::notes::Generator;

fn main() {
    test_midi_gen_percu();
}


fn test_midi_gen_percu()
{
    let scale : utils::notes::Scale = utils::notes::Scale{notes : vec![utils::notes::NoteNames::A, utils::notes::NoteNames::B, utils::notes::NoteNames::C, utils::notes::NoteNames::D, utils::notes::NoteNames::E, utils::notes::NoteNames::F, utils::notes::NoteNames::G], root : utils::notes::NoteNames::A};
    let analysis_data : utils::analysis_file::AnalysisData = utils::analysis_file::AnalysisData{bpm : 120 as usize, scale : scale, start_time : 0.0};
    let pathstring : &str = "./assets/generation/test_make_melody.mid";
    let mut percu_gen : generator::algorithms::percu::PercuGenerator = generator::algorithms::percu::PercuGenerator::new(analysis_data);
    percu_gen.generate(0.0, 20.0);
    println!("{:?}", percu_gen.get_notes_vec().len());
    percu_gen.midi_gen(pathstring);
}


//fn test_pitch_detection() {
//
//    let wav:Wave32 =  file_import::import_wav32("./assets/sound examples/EDAC 60.wav");
//
//    let start = Instant::now();
//    let (notes, times) = analyser::tonal_analysis::get_fundamental_notes(&wav, 1.0);
//    let duration = start.elapsed();
//    for i in 0..(notes).len(){
//
//        println!("{}, {:.1}s", notes[i], times[i]);
//
//    }
//    println!("Time elapsed : {:?}", duration);
//}


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