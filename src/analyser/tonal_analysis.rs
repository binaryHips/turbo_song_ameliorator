
use fundsp::hacker32::*;
use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::yin::YINDetector;
use pitch_detection::detector::PitchDetector;
use pitch_detection::Pitch;
use num_traits::FromPrimitive;
use crate::utils::*;
use std::time::{Duration, Instant};
///Converts a frequency into a musical note.
/// Returns a tuple containing the note name, the octave and the midi note number.
pub fn freq_to_note(freq: f32) -> notes::NoteNames{

    let mut note_number = round(12.0 * log2(freq / 440.0)) as usize;
    
    note_number = note_number + 49;
        
    let note = FromPrimitive::from_usize((note_number - 1 ) % 12).unwrap();
    
    return note;
}


// fn convert(b: Vec<f32>) -> Vec<f64> {
//     b.into_iter().map(|x| x as f64).collect()
// }

// ///Applies pitch detection to one slice of a given size. 
// ///Currently uses the yin algorithm.
// pub fn get_fundamental_note(signal_slice: &Vec<f64>, samplerate:f64, window_size:usize)-> Option<Pitch<f64>>{
//     let PADDING: usize = window_size / 2;
//     const POWER_THRESHOLD: f64 = 0.1;
//     const CLARITY_THRESHOLD: f64 = 0.01;
//     let mut detector = YINDetector::new(window_size, PADDING);
//     let pitch = detector
//         .get_pitch(signal_slice, samplerate as usize, POWER_THRESHOLD, CLARITY_THRESHOLD);
//     return pitch;
// }

// ///Returns all the pitches and times of the fundamental note in the song.
// /// Currently unstable.

// pub fn get_fundamental_notes(wav: &Wave32, interval_seconds:f64) -> (Vec<String>, Vec<f64>) {

//     const WINDOW_SIZE:usize = 1024;

//     let samplerate = wav.sample_rate();

//     //let mut pitches: Vec<Pitch<f64>> = Vec::new();
//     let mut notes:Vec<String> = Vec::new();
//     let mut times: Vec<f64> = Vec::new();

//     let signal32: Vec<f32> = wav.channel(0).clone();

//     let signal = convert(signal32);

//     let mut last_note:String = String::new();

//     for i in (0..wav.len()-WINDOW_SIZE).step_by((interval_seconds*samplerate) as usize) {

//         let signal_slice = &signal[i..(i+WINDOW_SIZE)].to_vec();

//         if let Some(pitch) = get_fundamental_note(signal_slice, samplerate, WINDOW_SIZE) {

//             let freq = pitch.frequency;

//             let note = freq_to_note(freq).0;

//             if note != last_note{
            

//                 last_note = note.clone();
//                 notes.push(
//                     note
//                 );
                
                
//                 times.push( (i as f64 )/ samplerate);


//             }
//         }
//     }

//     return (notes, times);


// }

// pub fn get_midi_note_events(wav: &Wave32) {

//     let (notes, times) = get_fundamental_notes(wav, 0.01);



// }

/// infers minor scale a wav. Experimental
pub fn get_minor_scale(wav:&Wave32)-> notes::Scale{
    

    let samplerate = wav.sample_rate();

    let signal32: &Vec<f32> = &wav.channel(0)[0..500_000].to_vec();

    let WINDOW_SIZE:usize = signal32.len();
    
    let padding: usize = WINDOW_SIZE / 2;
    const POWER_THRESHOLD: f32 = 0.1;
    const CLARITY_THRESHOLD: f32 = 0.01;
    let mut detector = YINDetector::new(WINDOW_SIZE, padding);
    let pitch = detector
        .get_pitch(signal32, samplerate as usize, POWER_THRESHOLD, CLARITY_THRESHOLD);


    infer_minor_scale(
        freq_to_note(pitch.unwrap().frequency)
    )
}
/// infers minor scale from a note
pub fn infer_minor_scale(name : notes::NoteNames) -> notes::Scale
{
    let tons : Vec<i32> = vec![0, 2, 3, 5, 7, 8, 10];
    let mut notes : Vec<notes::NoteNames> = vec![];
    for i in tons {
        notes.push(FromPrimitive::from_i32(((name as i32)+i)%12).unwrap());
    }
    return notes::Scale{notes : notes, root : name};
}