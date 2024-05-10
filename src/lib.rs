
mod utils; //utility functions. Contains notes definitions.

mod analyser;
mod generator;

use godot::prelude::*;
use num_traits::FromPrimitive;
use utils::analysis_file::AnalysisData;
use utils::notes::NoteNames;
use utils::*;
use std::boxed::Box;
use generator::algorithms::markov::MarkovGenerator;
struct MyExtension;
use notes::Generator;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}




/// Wrapper for godot of analysdata
#[derive(Debug)]
#[derive(GodotClass)]
#[class(no_init, base = RefCounted)]
pub struct MusicAnalysisData{
    
    data: utils::analysis_file::AnalysisData,

    base: Base<RefCounted>

}


#[godot_api]
impl IRefCounted for MusicAnalysisData {

    fn to_string(&self) -> GString {

        let mut s:String = "AnalysisData[\n".to_string();
        s.push_str(&format!("\tbpm: {}\n", &self.data.bpm));
        s.push_str(&format!("\troot note: {:?}\n", &self.data.scale.root));
        s.push_str(&format!("\tnotes: {:?}\n", &self.data.scale.notes));
        s.into()
    }

}
#[godot_api]
impl MusicAnalysisData{
    #[func]
    fn create(bpm: i64, notes:Array<i64>, root:i64, start_time:f64) -> Gd<Self>{

        //sanity check
        if !notes.iter_shared().all(|x| x >= 0 && x < 12) || root < 0 || root >= 12{
            panic!("Notes must be in the range [[0,11]]")
        }

        let mut notes_vec:Vec<notes::NoteNames> = vec![];

        for n in notes.iter_shared(){
            notes_vec.push(FromPrimitive::from_i64(n).unwrap())
        }

        let scale = notes::Scale::new(
            notes_vec,
            FromPrimitive::from_i64(root).unwrap()
        );

        Gd::from_init_fn(|base| {

            Self {
                data: utils::analysis_file::AnalysisData {bpm:bpm as usize, scale, start_time},
                base
            }
   
        })
    }
}


#[derive(Debug)]
#[derive(GodotClass)]
#[class(no_init, base = RefCounted)]
struct MusicGenerator{

    generator: Box<dyn utils::notes::Generator>,

    ana_data: MusicAnalysisData,

    base: Base<RefCounted>
}

#[godot_api]
impl MusicGenerator {

    #[func]
    fn create(data:MusicAnalysisData, algo:GString) -> Gd<Self>{

        let generator = match algo.to_string().to_lowercase().as_str(){

            "markov" => MarkovGenerator::new(data.data),
            _ => panic!("Not a valid algorithm name. Valid names are: 'markov'"),
        };

        Gd::from_init_fn(|base| {

            Self {generator, ana_data:data, base}
        })
    }

    #[func]
    fn generate(&mut self, start_time:f64, end_time:f64){

        self.generator.generate(start_time, end_time);
    }

    fn get_notes(&self) -> Vec<MusicNote>{
        MusicNote::from_vec(self.generator.get_notes_vec())
    }

}

#[godot_api]
impl IRefCounted for MusicGenerator {}

#[derive(Debug)]
#[derive(GodotClass)]
#[class(no_init, base = RefCounted)]
struct MusicNote{

    #[var]
    start_time:f64,

    #[var]
    end_time:f64,

    #[var]
    note:i64,

    #[var]
    octave:i64,

    #[var]
    velocity:i64,


    base: Base<RefCounted>
}



#[godot_api]
impl MusicNote {

    #[func]
    pub fn create(start_time:f64, end_time:f64, note:i64, octave:i64, velocity:i64) -> Gd<Self>{

        Gd::from_init_fn(|base:Base<RefCounted>| {

            Self {start_time, end_time, note, octave, velocity, base}
        })
    }

    pub fn from_vec(v: &Vec<(notes::Note, f64, f64)>) -> Array<Gd<Self>>{
        let mut res:Array<Gd<Self>>;

        for i in v{

            res.push(
                Self::create(i.1, i.2, (i.0.note as u8).into(), i.0.octave.into(), (i.0.velocity as u8).into())
            );
        }
        res
    }
}

#[godot_api]
impl IRefCounted for MusicNote {}
