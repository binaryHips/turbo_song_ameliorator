mod file_import;
mod analyser;
use fundsp::hacker32::*;


fn convert(b: Vec<f32>) -> Vec<f64> {
    b.into_iter().map(f64).collect()
}

fn main() {
    let wav:Wave32 =  file_import::import_wav32("../assets/sound examples/amelioratorOfficialSoundtrack.wav");

    let samplerate = wav.sample_rate();
    let signal32 = *(wav.channel(0));

    let signal = convert(signal32);

    analyser::tonal_analysis::get_fundamental_note(signal, samplerate);

    
}
