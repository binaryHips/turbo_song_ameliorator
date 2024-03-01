
use fundsp::hacker32::*;
use std::fs;
use midly::Smf;

/// Imports a wave file in 32 bits fixed-point format.
/// Takes an absolute path string as an input.
pub fn import_wav32(pathstring:&str) -> Wave32 {
    let path = std::path::Path::new(&pathstring);

    let wave = Wave32::load(path).expect("Could not load file");
    return wave;
}


/// saves to a wave file
/// Takes an absolute path string as an input.
pub fn save_wav32(file:Wave32, pathstring:&str) {

    let path = std::path::Path::new(pathstring);

    file.save_wav32(path).expect("Could not save file");
}


/// loads an analysis file.
/// Takes an absolute path string as an input.
/// returns a tuple containing the bytes


pub fn import_analysis_file<'a>(pathstring:&str, bytes: &'a mut Vec<u8>, smf: &mut Smf<'a>) {



    // Load bytes into a buffer
    *bytes = fs::read(pathstring).unwrap();

    // Parse bytes in a separate step
    *smf = Smf::parse(bytes).unwrap();
}