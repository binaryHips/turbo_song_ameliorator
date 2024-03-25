
use fundsp::hacker32::*;
use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::yin::YINDetector;
use pitch_detection::detector::PitchDetector;
use pitch_detection::Pitch;

use std::time::{Duration, Instant};
///Converts a frequency into a musical note.
/// Returns a tuple containing the note name, the octave and the midi note number.
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

///Applies pitch detection to one slice of a given size. 
///Currently uses the yin algorithm.
pub fn get_fundamental_note(signal_slice: &Vec<f64>, samplerate:f64, window_size:usize)-> Option<Pitch<f64>>{
    let PADDING: usize = window_size / 2;
    const POWER_THRESHOLD: f64 = 0.1;
    const CLARITY_THRESHOLD: f64 = 0.01;
    let mut detector = YINDetector::new(window_size, PADDING);
    let pitch = detector
        .get_pitch(signal_slice, samplerate as usize, POWER_THRESHOLD, CLARITY_THRESHOLD);
    return pitch;
}

///Returns all the pitches and times of the fundamental note in the song.
/// Currently unstable.
pub fn get_fundamental_notes(wav: &Wave32, interval_seconds:f64) -> (Vec<String>, Vec<f64>) {

    const WINDOW_SIZE:usize = 1024;

    let samplerate = wav.sample_rate();

    //let mut pitches: Vec<Pitch<f64>> = Vec::new();
    let mut notes:Vec<String> = Vec::new();
    let mut times: Vec<f64> = Vec::new();

    let signal32: Vec<f32> = &wav.channel(0);

    let signal = convert(signal32);

    let mut last_note:String = String::new();

    for i in (0..wav.len()-WINDOW_SIZE).step_by((interval_seconds*samplerate) as usize) {

        let signal_slice = &signal[i..(i+WINDOW_SIZE)].to_vec();

        if let Some(pitch) = get_fundamental_note(signal_slice, samplerate, WINDOW_SIZE) {

            let freq = pitch.frequency;

            let note = freq_to_note(freq).0;

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




