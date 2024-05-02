use midly::{self, Smf};
use crate::utils::*;
use midly::*;


pub fn add_note(file: &Smf, note: notes::Note){

    let note:midly::MidiMessage;
    
}



pub fn get_bpm(file:&Smf) -> i32{

    return 0;
}


pub fn get_first_instant(file:&Smf) -> f64{

    return 5.0;
}

pub fn get_scale(file:&Smf) -> Vec<()>{
    let mut NotesDebut = Vec::new();
    let liste = file.tracks[0];
    let mut time = 0;
    let mut debut=0;
    for event in liste {
        let time = event.delta;
        if time == 0  || debut == 0 {
            let message = match event.kind {
                TrackEventKind::Midi { channel, message } => message,
                _ => continue
            };
            match message {
                MidiMessage::NoteOn {key,vel} =>  NotesDebut.push(key),
                MidiMessage::NoteOff {key,vel} => continue ,
                _ => continue
            };
            debut = 2;
        }
        else {
            break;
        }
    }
    return NotesDebut;
}
