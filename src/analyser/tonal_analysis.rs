
use fundsp::hacker32::*;
use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;
use pitch_detection::Pitch;

pub fn freq_to_note(freq: f64) -> (String, usize, usize){
    let notes = vec!["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];

    let mut note_number = round(12.0 * log2(freq / 440.0)) as usize;
    
    let midi_note = note_number+69; //A4 is the 69th midi note
    
    note_number = note_number + 49;
        
    let note = (note_number - 1 ) % notes.len();
    let note_str = notes[note].to_string();
    
    let octave = (note_number + 8 ) / notes.len();
    
    return (note_str, octave, midi_note);
}


fn convert(b: Vec<f32>) -> Vec<f64> {
    b.into_iter().map(|x| x as f64).collect()
}


pub fn get_fundamental_note(signal_slice: &Vec<f64>, samplerate:f64)-> Option<Pitch<f64>>{
    const SIZE: usize = 4000;
    const PADDING: usize = SIZE / 2;
    const POWER_THRESHOLD: f64 = 3.0;
    const CLARITY_THRESHOLD: f64 = 0.3;

    let mut detector = McLeodDetector::new(SIZE, PADDING);
    let pitch = detector
        .get_pitch(signal_slice, samplerate as usize, POWER_THRESHOLD, CLARITY_THRESHOLD);
    return pitch;
}


pub fn get_fundamental_notes(wav: &Wave32, interval_seconds:f64, ) -> (Vec<String>, Vec<f64>) {

    const WINDOW_SIZE:usize = 4000;

    let samplerate = wav.sample_rate();

    //let mut pitches: Vec<Pitch<f64>> = Vec::new();
    let mut notes:Vec<String> = Vec::new();
    let mut times: Vec<f64> = Vec::new();

    let signal32: Vec<f32> = (wav.channel(0)).clone();

    let signal = convert(signal32);

    let mut last_note:String = String::new();

    for i in (WINDOW_SIZE/2..wav.len()-WINDOW_SIZE/2).step_by((interval_seconds*samplerate) as usize) {

        let signal_slice = &signal[(i-WINDOW_SIZE/2)..(i+WINDOW_SIZE/2)].to_vec();

        if let Some(pitch) = get_fundamental_note(signal_slice, samplerate) {

            let freq = pitch.frequency as usize;

            let note = freq_to_note(pitch.frequency).0;

            if note != last_note{
            

                last_note = note.clone();
                notes.push(
                    note
                );
                
                
                times.push( (i as f64 )/ samplerate);


            }
        }
    }

    return (notes, times);


}

pub fn get_midi_note_events(wav: &Wave32) {

    let (notes, times) = get_fundamental_notes(wav, 0.01);



}