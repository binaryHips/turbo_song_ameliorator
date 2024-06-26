use std::fmt;
use std::fmt::Debug;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use midly::num;
use std::cmp::min;
use godot::prelude::*;



#[derive(FromPrimitive, Copy, Clone, Default)]
pub enum NoteNames {#[default] A, Ad, B, C, Cd, D, Dd, E, F, Fd, G, Gd, SILENCE}

impl From<i32> for NoteNames {
    fn from(n: i32) -> Self {
        FromPrimitive::from_i32((n) % 12).unwrap()
    }
}

impl From<usize> for NoteNames {
    fn from(n: usize) -> Self {
        FromPrimitive::from_i32((n as i32) % 12).unwrap()
    }
}

impl Debug for NoteNames{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>{

        write!(f, "{}", NOTES_STR[*self as usize])
    }
}

const NOTES_STR:[&str; 13] = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "silence"];
 
///A simple musical note.
/// 
#[derive(Copy, Clone, Debug, Default)]

pub struct Note{

    pub note:NoteNames,
    pub octave:u8,
    pub velocity:num::u7,
}

impl fmt::Display for Note{
    //for printing
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", NOTES_STR[self.note as usize], self.octave)
    }
}




impl Note {

    pub fn new(note:NoteNames) -> Self{

        Self { note, octave:5, velocity:num::u7::new(100)}
    }

    pub fn to_midi(&self) -> num::u7{
        return num::u7::new((21 + self.note as usize + (self.octave as usize * 12)) as u8);
    }

    pub fn quantize_to_scale(&mut self, scale:&Scale){
        let mut closest:usize = 99999;
        let mut note:NoteNames = NoteNames::A;
        for n in &scale.notes{
            let mut d = (*n as usize).abs_diff((self.note as usize + scale.root as usize)%12);
            d = min(d, 12-d); //modulo dist
            if d < closest{

                closest = d;
                note = n.clone();
            }
        }
        self.note = note.clone();
    }

}


#[derive(Clone, Debug, Default)]
pub struct Scale{

    pub notes:Vec<NoteNames>,
    pub root:NoteNames,
}

impl Scale{

    pub fn new(notes:Vec<NoteNames>, root:NoteNames)-> Self{
        Self {notes, root}
    }

    pub fn get_relative(&self, n:i32) -> NoteNames{
        return FromPrimitive::from_i32((n + self.root as i32) % 12).unwrap();
    }
}

pub trait Generator {

    fn get_notes_vec(&self) -> &Vec<(Note, f64, f64)>;


    fn generate(&mut self, start_time : f64, end_time : f64);


    fn midi_gen(&self, pathstring : &str);
}

impl Debug for dyn Generator{

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Generator(Current content: {:?})", &self.get_notes_vec())
    }
}