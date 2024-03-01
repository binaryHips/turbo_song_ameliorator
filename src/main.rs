mod file_import;

fn main() {
    let pathstring = "./assets/sound examples/amelioratorOfficialSoundtrack.wav";
    let file = file_import::import_wav32(pathstring);
    file_import::save_wav32(file, "./assets/sound examples/amelioratorOfficialSoundtrack2.wav");
}
