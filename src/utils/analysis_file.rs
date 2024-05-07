use midly::{*, Smf};
use crate::utils::*;
use inner::*;
use anyhow::{anyhow, Result};


#[derive(Clone)]
pub struct AnalysisData{
    bpm: usize,
    scale: notes::Scale,
    start_time:f64,
    //scales: Vec< (f64, notes::Scale) >, // notes detected for each timestamp.
    //volumes: Vec< (f64, f32) > //volume
}

impl AnalysisData{

    pub fn new(bpm: usize, scale: notes::Scale, start_time:f64) -> Self{
        AnalysisData {bpm, scale, start_time}
    }

    pub fn from_analysis_file(filename: &str) -> Result<Self>{
        let res = file_import::import_midi(filename);
        let smf = res.s;

        let bpm:usize = get_bpm(&smf);
        
        let scale:notes::Scale = get_scale(&smf);

        let first_instant:f64 = get_first_instant(&smf);

        Ok(AnalysisData {
            bpm,
            scale,
            start_time: first_instant,
        })
    }

    pub fn to_analysis_file(filename: &str){
        

    }
}


///Parses a MIDI file containing analysis informations. 

pub fn parse_analysis_file(midi:&Smf){


}

pub fn add_note(file: &Smf, note: notes::Note){

    let note:midly::MidiMessage;
    
}


///Recovers bpm from an smf
pub fn get_bpm(file:&Smf) -> usize{

    //find the metaEvent containing the bpm

    let meta_message = match file.tracks[0][0].kind{

        TrackEventKind::Meta(msg) => msg,
        _ => panic!("Tempo definition is not the first message of the file")
    };

    let ms_per_beat = match meta_message{

        MetaMessage::Tempo(v) => v.as_int() as f64,
        _ => panic!("Tempo definition is not the first message of the file")
    };

    return (60.0 / (ms_per_beat / 1000.0)) as usize
}


pub fn get_first_instant(file:&Smf) -> f64{

    return 5.0;
}


///gets scale from an Smf
pub fn get_scale(file:&Smf) -> notes::Scale{
    let mut notes_in_scale:Vec<notes::NoteNames> = Vec::new();
    let liste = &file.tracks[0];
    let mut time = 0;
    let mut debut = 0;
    for event in liste {
        let time = event.delta;
        if time == 0  || debut == 0 {

            let message = match event.kind {
                TrackEventKind::Midi { channel, message } => message,
                _ => continue
            };


            match message {
                MidiMessage::NoteOn {key,vel: _} =>  notes_in_scale.push(notes::NoteNames::from(key.as_int() as i32)),
                MidiMessage::NoteOff {key, vel: _} => continue ,
                _ => continue
            };
            debut = 2;
        }
        else {
            break;
        }
    }
    return notes::Scale::new(notes_in_scale.clone(), notes_in_scale[0]);
}
