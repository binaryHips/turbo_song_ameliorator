use crate::src::file_import;
use midly;
use crate::src::analyser;


enum Note{
    A,
    Ad,
    B,
    C,
    Cd,
    D,
    Dd,
    E,
    F,
    Fd,
    G,
    Gd
}

#[derive(Copy, Clone)]
struct Analysis_data{
    bpm: usize,
    notes: Vec< (f64, Vec<Note>) >, // notes detected for each timestamp.
    volumes: Vec< (f64, f32) > //volume
}




///Parses a MIDI file containing analysis informations. 

pub fn parse_analysis_file(midi:&Smf){


}