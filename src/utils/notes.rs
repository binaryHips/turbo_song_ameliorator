use std::fmt;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use midly::num;
use std::cmp::min;
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum NoteNames {A, Ad, B, C, Cd, D, Dd, E, F, Fd, G, Gd, SILENCE}

impl From<i32> for NoteNames {
    fn from(n: i32) -> Self {
        FromPrimitive::from_i32((n) % 12).unwrap()
    }
}

const NOTES_STR:[&str; 13] = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "silence"];

///A simple musical note.
/// 
#[derive(Copy, Clone)]
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
        let mut note:NoteNames;
        for n in &scale.notes{
            let d = (*n as usize).abs_diff(self.note as usize);
            let d = min(d, 12-d); //modulo dist
            if d <= closest{

                closest = d;
                self.note = n.clone();
            }
        }

    }

}


#[derive(Clone)]
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