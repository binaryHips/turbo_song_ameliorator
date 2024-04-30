use std::fmt;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(FromPrimitive, Copy, Clone)]
enum NoteNames {A, Ad, B, C, Cd, D, Dd, E, F, Fd, G, Gd}

const NOTES_STR:[&str; 12] = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];

///A simple musical note.
/// 
pub struct Note{

    note:NoteNames,
    octave:u8,
}

impl fmt::Display for Note{
    //for printing
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", NOTES_STR[self.note as usize], self.octave)
    }
}


impl Note {


    pub fn to_midi(&self) -> u8{
        return (21 + self.note as usize + (self.octave as usize * 12)) as u8;
    }

    pub fn quantize_to_scale(&mut self, scale:Scale){
        let mut closest:usize = 99999;
        for n in scale.notes{
            let d = (n as usize).abs_diff(self.note as usize);
            if d <= closest{

                closest = d;
                self.note = n;
            }
        }

    }

}


struct Scale{

    notes:Vec<NoteNames>,
    root:NoteNames,
}

impl Scale{
    pub fn get_relative(&self, n:i32) -> NoteNames{
        return FromPrimitive::from_i32((n + self.root as i32) % 12).unwrap();
    }
}