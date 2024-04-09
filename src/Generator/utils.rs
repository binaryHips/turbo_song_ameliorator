mod utils;
use std::fmt;

enum Notes_name {A, As, B, C, Cs, D, Ds, E, F, Fs, G, Gs} // s for sharp

const notes_str:Vec = vec!["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];

///A simple musical note.
/// 
struct Note{

    note:Notes_name,
    octave:u8,
    velocity:u8,

}

impl fmt::Display for Note{
    //for printing
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", notes_str[self.note], self.octave)
    }

}


impl Note {


    fn to_midi(&self) -> u8{
        return 21 + note + (octave * 12);
    }

}