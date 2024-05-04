use midly::{*, Smf};
use crate::utils::*;
use inner::*;

#[derive(Clone)]
struct AnalysisData{
    bpm: usize,
    notes: Vec< (f64, notes::Scale) >, // notes detected for each timestamp.
    volumes: Vec< (f64, f32) > //volume
}

impl AnalysisData{

    pub fn from_analysis_file(filename: &str) -> Result<AnalysisData>{


    }

    pub fn to_analysis_file(filename: &str) -> Result<AnalysisData>{
        

    }
}


///Parses a MIDI file containing analysis informations. 

pub fn parse_analysis_file(midi:&Smf){


}

pub fn add_note(file: &Smf, note: notes::Note){

    let note:midly::MidiMessage;
    
}



pub fn get_bpm(file:&Smf) -> i32{

    return 0;
}


pub fn get_first_instant(file:&Smf) -> f64{

    return 5.0;
}


///gets scale from a 
pub fn get_scale(file:&Smf) -> notes::Scale{
    let mut NotesDebut = Vec::new();
    let liste = file.tracks[0];
    let mut time = 0;
    let mut debut = 0;
    for event in liste {
        let time = event.delta;
        if time == 0  || debut == 0 {
            
            //ancien

            // let message = match event.kind {
            //     TrackEventKind::Midi { channel, message } => message,
            //     _ => continue
            // };

            //Nouveau
            let message = inner!(event, if  TrackEventKind::Midi, else {continue}).message;

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
    return notes::Scale::new(NotesDebut, NotesDebut[0]);
}
