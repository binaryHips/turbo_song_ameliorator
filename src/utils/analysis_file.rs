use midly::{*, Smf};
use num_traits::FromPrimitive;
use crate::utils::*;
use anyhow::{anyhow, Result};
use std::fs;
use godot::prelude::*;
use num_derive::FromPrimitive;
/// A struct that represents the data from an analysis.
/// Can be constructed from a MIDI analysis file (experimental)
/// 


#[derive(Clone, Debug, Default)]
pub struct AnalysisData{
    /// beats per minute. Currently doesnt support non-int values.
    pub bpm: usize,

    /// Scale of the song. For now, assumes the whole song is in the same scale.
    pub scale: notes::Scale,

    /// Offset from the start of the file to the start of the song
    pub start_time:f64,
    //scales: Vec< (f64, notes::Scale) >, // notes detected for each timestamp.
    //volumes: Vec< (f64, f32) > //volume
}

impl AnalysisData{

    ///Creates a new AnalysisData
    pub fn new(bpm: usize, scale: notes::Scale, start_time:f64) -> Self{
        AnalysisData {bpm, scale, start_time}
    }

    /// DOES NOT WORK YET
    /// Imports a midi file containing analysis file information, and parses its data into an AnalysisData.
    pub fn from_analysis_file(filename: &str) -> Result<Self>{

        let bytes:Vec<u8> = fs::read(filename).unwrap();
        let smf = Smf::parse(&bytes).unwrap();
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

fn add_note(file: &Smf, note: notes::Note){

    let note:midly::MidiMessage;
    
}


///Recovers bpm from an smf
fn get_bpm(file:&Smf) -> usize{

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


fn get_first_instant(file:&Smf) -> f64{

    return 5.0;
}


///gets scale from an Smf
fn get_scale(file:&Smf) -> notes::Scale{
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
