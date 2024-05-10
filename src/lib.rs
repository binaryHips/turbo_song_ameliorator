
mod utils; //utility functions. Contains notes definitions.

mod analyser;
mod generator;

use godot::prelude::*;
use num_traits::FromPrimitive;
use utils::notes::NoteNames;
use utils::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}




//Wrapper for godot of analysdata
#[derive(Debug)]
#[derive(GodotClass)]
#[class(no_init, base = RefCounted)]
pub struct AnalysisData{
    
    data: utils::analysis_file::AnalysisData,

    base: Base<RefCounted>

}


#[godot_api]
impl IRefCounted for AnalysisData {

    fn to_string(&self) -> GString {

        let mut s:String = "AnalysisData[\n".to_string();
        s.push_str(&format!("\tbpm: {}\n", &self.data.bpm));
        s.push_str(&format!("\troot note: {:?}\n", &self.data.scale.root));
        s.push_str(&format!("\tnotes: {:?}\n", &self.data.scale.notes));
        s.into()
    }

}
#[godot_api]
impl AnalysisData{
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

// struct Algorithm{



// }