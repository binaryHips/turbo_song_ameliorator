use crate::utils::{notes, analysis_file};
use crate::generator;
use midly::*;
use rand::Rng;
use std::fs;

pub struct MusicalDiceGenerator
{
    melody : Vec<(notes::Note, f64, f64)>,
    analysis_data : analysis_file::AnalysisData,
}

impl MusicalDiceGenerator
{
    pub fn new(analysis_data : analysis_file::AnalysisData) -> Self
    {
        Self {melody : vec![], analysis_data : analysis_data}
    }

    pub fn get_melody(self) -> Vec<(notes::Note, f64, f64)>
    {
        return self.melody;
    }

    pub fn generate(&mut self, start_time : f64, end_time : f64)
    {
        self.melody = create(&self.analysis_data, start_time, end_time);
    }

    pub fn midi_gen(self, PATHSTRING : &str)
    {
        generator::midi_gen::midi_generator(&self.melody, &self.analysis_data, PATHSTRING);
    }
}


// nbFace correspond au nombre de faces du dé que l'on lance = nombres de morceaux musicaux disponibles.
// nbDice correspond au nombre de fois où on lance le dé.
fn roll_dice(nbFace : i32, nbDice : i32) -> Vec<i32>{
    let mut listeDes : Vec<i32> = vec![];
    let mut rng = rand::thread_rng();
    for i in 0..nbDice {
        let res : i32 = rng.gen_range(0..nbFace);
        listeDes.push(res);
    }
    return listeDes;
}

// & ou pas ?
fn create(analysis_data : &analysis_file::AnalysisData, start_time : f64, end_time : f64) -> Vec<(notes::Note, f64, f64)>
{
    const PATHSTRING : &str = "./MIDIS";
    const BPM_MUSIC : i32 = 120;
    const NBT_DE : i32 = 4;         // nombre de temps par dé lancé
    let mut nbMusic = 0;
    for entry in fs::read_dir(PATHSTRING) {
        nbMusic = nbMusic +1
    }
    let dureet : f64 = 60.0/(analysis_data.bpm as f64);        // durée d'un temp en seconde
    let mut start_temp  : f64 = analysis_data.start_time;       // instant du premier temps à jouer en seconde
    while start_temp < start_time {start_temp += dureet;}
    let mut end_temp  : f64 = start_temp;                       // instant du dernier temps à jouer en seconde
    while end_temp < end_time {end_temp += dureet;}
    let nbt : i32 = ((end_temp - start_temp)/dureet) as i32;        // nombre de temps à jouer
    let reste = nbt % NBT_DE;
    let nbDice : i32 = (nbt+NBT_DE-1)/NBT_DE;         // division entière avec arrondie sup
    let listeDes  : Vec<i32> = roll_dice(nbMusic, nbDice);
    let mut debut = 0.0;
    let mut fin = (NBT_DE as f64)*dureet;
    let mut melody : Vec<(notes::Note, f64, f64)> = vec![];
    let mut taille_melody : i32 = 0;
    let notes_pressed  = &mut vec![-1; 128];
    for i in listeDes
    {
        nbMusic = 0;
        for entry in fs::read_dir(PATHSTRING).unwrap()
        {
            if nbMusic == i
            {
                let bytes:Vec<u8> = fs::read((entry.unwrap()).path()).unwrap();
                let smf = Smf::parse(&bytes).unwrap();
                let track = &smf.tracks[0];
                let mut delta = 0.0;
                for track_event in track{
                    if delta>=debut && delta<fin
                    {
                        let message = match track_event.kind
                        {
                            TrackEventKind::Midi { channel, message } => message,
                            _ => continue 
                        };
                        match message
                        {
                            MidiMessage::NoteOn {key, vel: _} => if notes_pressed[key.as_int() as usize] == -1
                            {
                                let num_note = ((key.as_int() as i32)-21)%12;
                                let octave = ((key.as_int() as i32)-21)/12;
                                match num_note
                                {
                                    0 => melody.push((notes::Note{note : notes::NoteNames::A, octave : octave as u8, velocity : midly::num::u7::new(100)}, ((i*NBT_DE) as f64)*dureet+delta, 0.0)),
                                    1 => melody.push((notes::Note{note : notes::NoteNames::Ad, octave : octave as u8, velocity : midly::num::u7::new(100)}, ((i*NBT_DE) as f64)*dureet+delta, 0.0)),
                                    2 => melody.push((notes::Note{note : notes::NoteNames::B, octave : octave as u8, velocity : midly::num::u7::new(100)}, ((i*NBT_DE) as f64)*dureet+delta, 0.0)),
                                    3 => melody.push((notes::Note{note : notes::NoteNames::C, octave : octave as u8, velocity : midly::num::u7::new(100)}, ((i*NBT_DE) as f64)*dureet+delta, 0.0)),
                                    4 => melody.push((notes::Note{note : notes::NoteNames::Cd, octave : octave as u8, velocity : midly::num::u7::new(100)}, ((i*NBT_DE) as f64)*dureet+delta, 0.0)),
                                    5 => melody.push((notes::Note{note : notes::NoteNames::D, octave : octave as u8, velocity : midly::num::u7::new(100)}, ((i*NBT_DE) as f64)*dureet+delta, 0.0)),
                                    6 => melody.push((notes::Note{note : notes::NoteNames::Dd, octave : octave as u8, velocity : midly::num::u7::new(100)}, ((i*NBT_DE) as f64)*dureet+delta, 0.0)),
                                    7 => melody.push((notes::Note{note : notes::NoteNames::E, octave : octave as u8, velocity : midly::num::u7::new(100)}, ((i*NBT_DE) as f64)*dureet+delta, 0.0)),
                                    8 => melody.push((notes::Note{note : notes::NoteNames::F, octave : octave as u8, velocity : midly::num::u7::new(100)}, ((i*NBT_DE) as f64)*dureet+delta, 0.0)),
                                    9 => melody.push((notes::Note{note : notes::NoteNames::Fd, octave : octave as u8, velocity : midly::num::u7::new(100)}, ((i*NBT_DE) as f64)*dureet+delta, 0.0)),
                                    10 => melody.push((notes::Note{note : notes::NoteNames::G, octave : octave as u8, velocity : midly::num::u7::new(100)}, ((i*NBT_DE) as f64)*dureet+delta, 0.0)),
                                    11 => melody.push((notes::Note{note : notes::NoteNames::Gd, octave : octave as u8, velocity : midly::num::u7::new(100)}, ((i*NBT_DE) as f64)*dureet+delta, 0.0)),
                                    _ => continue
                                }
                                notes_pressed[key.as_int() as usize] = taille_melody;
                                taille_melody += 1;   
                            },
                            MidiMessage::NoteOff {key, vel: _} => if notes_pressed[key.as_int() as usize] != -1
                            {
                                melody[notes_pressed[key.as_int() as usize] as usize].2 = ((i*NBT_DE) as f64)*dureet+delta;
                                notes_pressed[key.as_int() as usize] = -1;
                            },
                            _ => continue
                        }
                    }
                    else if delta >= fin && delta < end_temp-start_temp
                    {
                        let message = match track_event.kind
                        {
                            TrackEventKind::Midi { channel, message } => message,
                            _ => continue 
                        };
                        match message
                        {
                            MidiMessage::NoteOff {key, vel: _} => if notes_pressed[key.as_int() as usize] != -1
                            {
                                melody[notes_pressed[key.as_int() as usize] as usize].2 = ((i*NBT_DE) as f64)*dureet+delta;
                                notes_pressed[key.as_int() as usize] = -1;
                            },
                            _ => continue,                
                        }
                    }
                    delta += (track_event.delta.as_int() as f64)*(BPM_MUSIC as f64)/(analysis_data.bpm as f64);
                }
                for pos in (&mut *notes_pressed).into_iter()
                {
                    if *pos != -1
                    {
                        melody[(*pos) as usize].2 = end_temp-start_temp;
                        *pos = -1;
                    }
                }
                break;
            }
            nbMusic = nbMusic +1;
        }
        debut = debut + (NBT_DE as f64)*dureet;
        fin = fin + (NBT_DE as f64)*dureet 
    }
    scaling(&mut melody, &analysis_data);
    return melody;
}


/// Récupère la mélodie générée par construct_melody() et ajuste chaque note à la gamme supposé du fichier d'analyse.
/// Pas de retour.
fn scaling(melody : &mut Vec<(notes::Note, f64, f64)>, analysis_data : &analysis_file::AnalysisData)
{
    for song in melody {
        song.0.quantize_to_scale(&analysis_data.scale);
    }
}