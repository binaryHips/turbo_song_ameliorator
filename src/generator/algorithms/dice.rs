use crate::utils::{notes, analysis_file};
use crate::generator;
use midly::*;
use rand::Rng;
use std::fs;

pub struct MusicalDiceGenerator
{
    melody : Vec<(notes::Note, f64, f64)>,
    analysis_data : analysis_file::AnalysisData,
    pathstring : &str,
}

impl 
MusicalDiceGenerator
{
    pub fn new(analysis_data : analysis_file::AnalysisData, pathstring : &str) -> Self
    {
        Self {melody : vec![], analysis_data : analysis_data, pathstring : pathstring}
    }

    pub fn get_melody(self) -> Vec<(notes::Note, f64, f64)>
    {
        return self.melody;
    }

    pub fn set_pathstring(pathstring : &str)
    {
        self.pathstring = pathstring;
    }

    pub fn generate(&mut self, start_time : f64, end_time : f64)
    {
        self.melody = create(&self.analysis_data, start_time, end_time, self.pathstring);
    }

    pub fn midi_gen(self)
    {
        generator::midi_gen::midi_generator(&self.melody, &self.analysis_data, self.pathstring);
    }
}


// nbFace correspond au nombre de faces du dé que l'on lance = nombres de morceaux musicaux disponibles.
// nbDice correspond au nombre de fois où on lance le dé.
fn roll_dice(nbFace : i32, nbDice : i32) -> Vec<i32>{
    let mut listeDes : Vec<i32> = vec![];
    let rng = rand::thread_rng();
    for i in 0..nbDice {
        let res : i32 = rng.gen_range(0..nbFace);
        listeDes.push(res);
    }
    return listeDes;
}

// & ou pas ?
fn create(analysis_data : &analysis_file::AnalysisData, start_time : f64, end_time : f64, pathstring : &str) -> Vec<(notes::Note, f64, f64)>
{  
    let mut cpt = 0;
    for entry in fs::read_dir(pathstring)? {
        cpt = cpt +1
    }
    let bytesM:Vec<u8> = fs::read(musique).unwrap();
    let smfM = Smf::parse(&bytesM).unwrap();
    let trackM = smfM.tracks[0];
    let mut deltaM = 0;
    for event in track{
        deltaM = deltaM + event.delta;
    }
    let mut nbDice = 0;
    let quotient = deltaM % 4;
    let deltaM = deltaM - quotient;
    if quotient != 0 {
        nbDice = deltaM/4 +1;
    }
    else {
        nbDice = deltaM/4;
    }
    let mut listeDes = roll_dice(cpt,nbDice);
    let mut debut = 0;
    let mut fin = 4;
    let mut trackSortie: Vec<TrackEvent>; 
    for i in listeDes {
        cpt = 0;
        for entry in fs::read_dir(pathstring)?{
            if cpt == i {
                let bytes:Vec<u8> = fs::read(entry).unwrap();
                let smf = Smf::parse(&bytes).unwrap();
                let track = smf.tracks[0];
                let mut delta: u28 = 0;
                for event in track{
                    if delta < debut{
                        delta = delta + event.delta;
                    }
                    else {
                        if delta< fin {
                            trackSortie.push(event);
                            let mut notes_in_scale = [];
                            for i in 0..127{
                                note_in_scale.push(0);
                            }
                            let mut notes_in_scale0 = notes_in_scale;
                            let message = match event.kind {
                                TrackEventKind::Midi { channel, message } => message,
                                _ => continue 
                            };
                            match message {
                                MidiMessage::NoteOn {key, vel: _} =>  notes_in_scale[key]= notes_in_scale[key]+1,
                                MidiMessage::NoteOff {key, vel: _} => if notes_in_scale != notes_in_scale0 {notes_in_scale[key]= notes_in_scale[key]-1},
                                _ => continue
                            };
                            delta = delta+event.delta;
                        }
                        else {
                            if notes_in_scale != notes_in_scale0 {
                                for i in 0..127 {
                                    while notes_in_scale[i] != 0 {
                                        trackSortie.push(TrackEvent{delta : num::u28::new(0 as u32), kind : TrackEventKind::Midi{channel: num::u4::new(0 as u8), message : message::MidiMessage::NoteOff{key: i, vel :0}}});
                                        notes_in_scale[key]= notes_in_scale[key]-1;
                                    }
                                }
                        
                            }  
                
                        }
                    }
                    
                }
            }
            cpt = cpt +1;
        }
        debut = debut +4;
        fin = fin +4 
    }
}


/// Récupère la mélodie générée par construct_melody() et ajuste chaque note à la gamme supposé du fichier d'analyse.
/// Pas de retour.
fn scaling(melody : &mut Vec<(notes::Note, f64, f64)>, anafile : &analysis_file::AnalysisData)
{
    for song in melody {
        song.0.quantize_to_scale(&anafile.scale);
    }
}