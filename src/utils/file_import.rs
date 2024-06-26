
use fundsp::hacker32::*;
use std::fs;
use midly::Smf;


/// Imports an audio file of any format as a wave in 32 bits fixed-point format.
/// Takes an absolute path string as an input.
pub fn import_wav32(pathstring:&str) -> Wave32 {
    let path = std::path::Path::new(&pathstring);

    let wave = Wave32::load(path).expect("Could not load file");
    return wave;
}


/// saves to a wave file
/// Takes an absolute path string as an input.
pub fn save_wav32(file:&Wave32, pathstring:&str) {

    let path = std::path::Path::new(pathstring);

    file.save_wav32(path).expect("Could not save file");
}



// pub fn import_midi(pathstring:&str) -> Smf {

//     // Load bytes into a buffer
//     let bytes = fs::read(pathstring).unwrap();

//     // Parse bytes in a separate step
//     let smf = Smf::parse(&(bytes.clone())).unwrap();

//     return smf; //not ideal? idk.
// }

/// saves to a midi file
/// Takes an absolute path string as an input.

pub fn save_midi(file:&Smf, pathstring:&str) {

    file.save(pathstring).unwrap();
}

