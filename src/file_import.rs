
use fundsp::hacker32::*;


/// Imports a wave file in 32 bits fixed-point format.
/// Takes an absolute path string as an input.
pub fn import_wav_file_32(pathstring:&str) -> Wave32 {
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