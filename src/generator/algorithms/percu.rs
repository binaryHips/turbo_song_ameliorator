use crate::utils::{notes, analysis_file};
use crate::generator;
use midly;
use rand::Rng;
use num_traits::FromPrimitive;
use std::cmp;

#[derive(Default, Debug)]
pub struct PercuGenerator
{
    notes_vec : Vec<(notes::Note, f64, f64)>,
    analysis_data : analysis_file::AnalysisData,
}

impl PercuGenerator{

    pub fn new(analysis_data : analysis_file::AnalysisData) -> Self
    {
        Self {notes_vec : vec![], analysis_data : analysis_data}
    }
}

impl notes::Generator for PercuGenerator
{

    fn get_notes_vec(&self) -> &Vec<(notes::Note, f64, f64)>
    {
        return & (self.notes_vec);
    }

    fn generate(&mut self, start_time : f64, end_time : f64)
    {
        self.notes_vec = create(&self.analysis_data, start_time, end_time);
    }

    fn midi_gen(&self, pathstring : &str)
    {
        generator::midi_gen::midi_generator(&self.notes_vec, &self.analysis_data, pathstring);
    }
}


/// Génère aléatoirement une mélodie basée sur des probabilités, sur une portion d'une musique, à partir de données analysées sur la musique.
/// Renvoie une mélodie sous la forme d'un vecteur songs représentés par une Note (à l'intensité nulle pour les silences),
/// un instant de début et un de fin en seconde codées sur deux flottants sur 64 bits. (Les notes commencent à l'instant 0)
fn create(analysis_data : &analysis_file::AnalysisData, start_time : f64, end_time : f64) -> Vec<(notes::Note, f64, f64)>
{
    let percu_prob : Vec<Vec<f64>> = vec![
        vec![1.0, 0.15, 0.75, 0.05, 0.25, 0.05],      // 0 (premier temps)
        vec![0.0, 0.0, 1.0, 0.05, 0.0, 0.05],    // 1/2
        vec![0.0, 0.75, 0.5, 0.05, 0.05, 0.05],    // 1
        vec![0.0, 0.0, 1.0, 0.1, 0.0, 0.05],     // 1,5
        vec![1.0, 0.25, 0.75, 0.05, 0.1, 0.05],      // 2...
        vec![0.0, 0.25, 1.0, 0.05, 0.0, 0.05], 
        vec![0.25, 0.75, 0.5, 0.05, 0.05], 
        vec![0.5, 0.25, 1.0, 0.25, 0.0, 0.05]
        ];      // [grosse caisse : A, caisse clair : A#, charlestone fermée : B, charlestone ouverte : C, crash : C#, percus : D et +]
    const NB_PERCUS : i32 = 3;          // nombre de percussions bonus
    const TMP_PER_MES : i32 = 4;        // nombre de temps par mesure
    const PRECISION : i32 = 2;      // plus petite fraction du temps
    let dureet : f64 = 60.0/(analysis_data.bpm as f64);        // durée d'un temp en seconde
    let mut start_temp  : f64 = analysis_data.start_time;
    while start_temp < start_time {start_temp += dureet;}
    let mut end_temp  : f64 = start_temp;
    while end_temp < end_time {end_temp += dureet;}
    let nbt : i32 = ((end_temp - start_temp)/dureet) as i32;        // nombre de temps à jouer
    let tmp_dans_mes : i32 = (((start_temp-analysis_data.start_time)/dureet) as i32)%TMP_PER_MES;
    let list_percu : Vec<Vec<bool>> = liste_percu_gen(nbt, PRECISION, TMP_PER_MES, tmp_dans_mes*PRECISION, &percu_prob, NB_PERCUS);
    let mut notes_vec : Vec<(notes::Note, f64, f64)> = construct_notes_vec(list_percu, dureet/(PRECISION as f64), start_temp-start_time);
    notes_vec.push((notes::Note{note : notes::NoteNames::A, octave : 5, velocity : midly::num::u7::new(100)}, 10.0, 10.0));
    return notes_vec;
}


/// Génère la suite de rythme nécessaire à la crétion de la mélodie sur le nombre de temps nbt.
/// Retourne la suite de rythme sous forme d'un vecteur de durées codées par un flottant sur 64 bits.
fn liste_percu_gen(nbt : i32, precision : i32, tmp_per_mes : i32, frac_tmp_dans_mes : i32, tab : &Vec<Vec<f64>>, nb_perucs : i32) -> Vec<Vec<bool>>
{
    let mut rng = rand::thread_rng();
    let mut list_percu : Vec<Vec<bool>> = vec![];
    for i in 0..(nbt*precision)
    {
        let mut percus : Vec<bool> = vec![];
        for j in 0..((tab[((i+frac_tmp_dans_mes)%(precision*tmp_per_mes)) as usize].len())-1+(nb_perucs as usize))
        {
            let proba : f64 = rng.gen::<f64>();
            let j2 : usize = cmp::min(j, (tab[((i+frac_tmp_dans_mes)%(precision*tmp_per_mes)) as usize].len()-1) as usize);
            percus.push(proba<tab[((i+frac_tmp_dans_mes)%(precision*tmp_per_mes)) as usize][j2]);
        }
        list_percu.push(percus);
    }
    return list_percu;
}


/// Assemble la suite de note et de rythme pour en faire une mélodie.
/// Retourne la mélodie sous la forme d'un vecteur de triplet (note, instant de début, instant de fin).
fn construct_notes_vec(list_percu : Vec<Vec<bool>>, dureet_frac : f64, decallage : f64) -> Vec<(notes::Note, f64, f64)>
{
    let mut notes_vec : Vec<(notes::Note, f64, f64)> = vec![];
    let mut instant  : f64 = decallage;
    for i in 0..list_percu.len()
    {
        for j in 0..(list_percu[i].len())
        {
            if list_percu[i][j]
            {
                notes_vec.push(
                    (
                    notes::Note{note : j.into(), octave : 5, velocity : midly::num::u7::new(127)},
                    instant,
                    instant+dureet_frac));
            }
        }
        instant += dureet_frac;
    }
    return notes_vec;
}