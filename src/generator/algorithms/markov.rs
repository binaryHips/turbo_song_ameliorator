use crate::utils::{notes, analysis_file};
use crate::generator;
use midly;
use rand::Rng;

#[derive(Default, Debug)]
pub struct MarkovGenerator
{
    notes_vec : Vec<(notes::Note, f64, f64)>,
    analysis_data : analysis_file::AnalysisData,
}

impl MarkovGenerator{

    pub fn new(analysis_data : analysis_file::AnalysisData) -> Self
    {
        Self {notes_vec : vec![], analysis_data : analysis_data}
    }
}

impl notes::Generator for MarkovGenerator
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
    let rhythm_prob : Vec<Vec<f64>> = vec![
        vec![0.23266745005875442, 0.4336075205640423, 0.0564042303172738, 0.2773207990599295], 
        vec![0.8601694915254238, 0.08050847457627118, 0.05084745762711865, 0.00847457627118644], 
        vec![0.3562585969738652, 0.4621733149931224, 0.016506189821182942, 0.16506189821182943], 
        vec![0.7694610778443114, 0.0718562874251497, 0.12275449101796407, 0.03592814371257485]
        ];        // [1/4, 1/2, 3/4, 1]
    let note_prob : Vec<Vec<f64>> = vec![
        vec![0.06666666666666667, 0.4, 0.03333333333333333, 0.06666666666666667, 0.1, 0.06666666666666667, 0.0, 0.06666666666666667, 0.1, 0.1, 0.0, 0.0, 0.0], 
        vec![0.12962962962962962, 0.12962962962962962, 0.16666666666666666, 0.1111111111111111, 0.037037037037037035, 0.030864197530864196, 0.006172839506172839, 0.024691358024691357, 0.09876543209876543, 0.037037037037037035, 0.012345679012345678, 0.15432098765432098, 0.06172839506172839], 
        vec![0.14814814814814814, 0.15432098765432098, 0.018518518518518517, 0.30246913580246915, 0.08024691358024691, 0.018518518518518517, 0.018518518518518517, 0.006172839506172839, 0.06172839506172839, 0.04938271604938271, 0.018518518518518517, 0.030864197530864196, 0.09259259259259259], 
        vec![0.208, 0.08, 0.152, 0.248, 0.048, 0.104, 0.008, 0.008, 0.024, 0.008, 0.008, 0.088, 0.016], 
        vec![0.11224489795918367, 0.22959183673469388, 0.15816326530612246, 0.030612244897959183, 0.1836734693877551, 0.14795918367346939, 0.02040816326530612, 0.03571428571428571, 0.02040816326530612, 0.02040816326530612, 0.00510204081632653, 0.015306122448979591, 0.02040816326530612], 
        vec![0.03355704697986577, 0.06040268456375839, 0.09395973154362416, 0.2483221476510067, 0.087248322147651, 0.040268456375838924, 0.3624161073825503, 0.0, 0.053691275167785234, 0.006711409395973154, 0.0, 0.013422818791946308, 0.0], 
        vec![0.017857142857142856, 0.08035714285714286, 0.03571428571428571, 0.25892857142857145, 0.026785714285714284, 0.08035714285714286, 0.07142857142857142, 0.23214285714285715, 0.14285714285714285, 0.008928571428571428, 0.03571428571428571, 0.008928571428571428, 0.0], 
        vec![0.012269938650306749, 0.018404907975460124, 0.012269938650306749, 0.018404907975460124, 0.3374233128834356, 0.06134969325153374, 0.1411042944785276, 0.04294478527607362, 0.22699386503067484, 0.024539877300613498, 0.03067484662576687, 0.05521472392638037, 0.018404907975460124], 
        vec![0.02702702702702703, 0.0, 0.013513513513513514, 0.08108108108108109, 0.02702702702702703, 0.28378378378378377, 0.0945945945945946, 0.08108108108108109, 0.14864864864864866, 0.17567567567567569, 0.04054054054054054, 0.02702702702702703, 0.0], 
        vec![0.060240963855421686, 0.03614457831325301, 0.012048192771084338, 0.04819277108433735, 0.03614457831325301, 0.09036144578313253, 0.27710843373493976, 0.04819277108433735, 0.09036144578313253, 0.10843373493975904, 0.06626506024096386, 0.10843373493975904, 0.018072289156626505], 
        vec![0.047619047619047616, 0.08333333333333333, 0.0, 0.023809523809523808, 0.0, 0.0, 0.03571428571428571, 0.10714285714285714, 0.2261904761904762, 0.08333333333333333, 0.023809523809523808, 0.21428571428571427, 0.15476190476190477], 
        vec![0.1111111111111111, 0.06666666666666667, 0.0, 0.0, 0.0, 0.044444444444444446, 0.06666666666666667, 0.08888888888888889, 0.26666666666666666, 0.022222222222222223, 0.15555555555555556, 0.022222222222222223, 0.15555555555555556], 
        vec![0.3137254901960784, 0.08823529411764706, 0.09803921568627451, 0.0392156862745098, 0.0, 0.0196078431372549, 0.00980392156862745, 0.00980392156862745, 0.22549019607843138, 0.11764705882352941, 0.0196078431372549, 0.058823529411764705, 0.0], 
        vec![0.10443037974683544, 0.16772151898734178, 0.0759493670886076, 0.12658227848101267, 0.10759493670886076, 0.0379746835443038, 0.10126582278481013, 0.06329113924050633, 0.0981012658227848, 0.04113924050632911, 0.00949367088607595, 0.015822784810126583, 0.05063291139240506]
        ];
    let note_prob_simple : Vec<Vec<f64>> = vec![
        vec![0.0, 0.6, 0.0, 0.0, 0.2, 0.0, 0.0, 0.0, 0.0, 0.2, 0.0, 0.0, 0.0],
        vec![0.0, 0.3684210526315789, 0.0, 0.0, 0.10526315789473684, 0.10526315789473684, 0.05263157894736842, 0.0, 0.0, 0.10526315789473684, 0.0, 0.10526315789473684, 0.15789473684210525],
        vec![0.14285714285714285, 0.03571428571428571, 0.0, 0.2857142857142857, 0.0, 0.03571428571428571, 0.03571428571428571, 0.0, 0.07142857142857142, 0.10714285714285714, 0.0, 0.0, 0.2857142857142857],
        vec![0.4, 0.0, 0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.2, 0.0, 0.0],
        vec![0.2, 0.55, 0.0, 0.0, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.05, 0.0],
        vec![0.0, 0.0, 0.06666666666666667, 0.4666666666666667, 0.0, 0.0, 0.4666666666666667, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.1, 0.0, 0.3, 0.2, 0.0, 0.2, 0.15, 0.05, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0],
        vec![0.125, 0.0, 0.0, 0.0, 0.0, 0.0, 0.75, 0.0, 0.0, 0.125, 0.0, 0.0, 0.0],
        vec![0.17647058823529413, 0.11764705882352941, 0.0, 0.0, 0.0, 0.0, 0.0, 0.23529411764705882, 0.23529411764705882, 0.0, 0.0, 0.0, 0.23529411764705882],
        vec![0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5],
        vec![0.6666666666666666, 0.3333333333333333, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.08695652173913043, 0.08695652173913043, 0.0, 0.17391304347826086, 0.13043478260869565, 0.0, 0.043478260869565216, 0.13043478260869565, 0.043478260869565216, 0.08695652173913043, 0.0, 0.0, 0.21739130434782608]
        ];
    let rhythm_prob_simple : Vec<Vec<f64>> = vec![
    vec![0.03529411764705882, 0.5764705882352941, 0.0, 0.38823529411764707],
    vec![0.6923076923076923, 0.3076923076923077, 0.0, 0.0],
    vec![0.2727272727272727, 0.6060606060606061, 0.0, 0.12121212121212122],
    vec![0.5, 0.45454545454545453, 0.0, 0.045454545454545456]
    ];
    let dureet : f64 = 60.0/(analysis_data.bpm as f64);        // durée d'un temp en seconde
    let mut start_temp  : f64 = analysis_data.start_time;
    while start_temp < start_time {start_temp += dureet;}
    let mut end_temp  : f64 = start_temp;
    while end_temp < end_time {end_temp += dureet;}
    let nbt : i32 = ((end_temp - start_temp)/dureet) as i32;        // nombre de temps à jouer
    let list_rhythm : Vec<f64> = markov_rhythm(nbt, 4, dureet, &rhythm_prob_simple);
    let nbn : i32 = list_rhythm.len() as i32;      // nombre de notes
    let list_notes : Vec<notes::Note> = markov_notes(nbn, &note_prob_simple);
    let mut notes_vec : Vec<(notes::Note, f64, f64)> = construct_notes_vec(list_rhythm, list_notes);
    scaling(&mut notes_vec, &ana_file);
    return notes_vec;
}


/// Génère la suite de rythme nécessaire à la crétion de la mélodie sur le nombre de temps nbt.
/// Retourne la suite de rythme sous forme d'un vecteur de durées codées par un flottant sur 64 bits.
fn markov_rhythm(nbt : i32, precision : i32, dureet : f64, tab : &Vec<Vec<f64>>) -> Vec<f64>
{
    let mut rng = rand::thread_rng();
    let mut somme : i32 = 0;
    let mut list_rhythm : Vec<f64> = vec![];
    while somme < nbt*precision
    {
        let proba : f64 = rng.gen::<f64>();
        let mut somme_prob : f64 = 1.0;
        let mut i : i32 = 0;
        while somme_prob > proba
        {
            somme_prob -= tab[(somme%precision) as usize][i as usize];
            i+=1;
        }
        // le i représente au début le position dans la liste de 0 à 3 mais fini à 1 de trop, or,
        // on l'utilise ensuite seulement pour les quarts allant de 1 à 4, donc tout va bien
        somme += i;
        list_rhythm.push(dureet*(i as f64)/(precision as f64));
    }
    let last = list_rhythm.len()-1;
    if somme > nbt*precision {list_rhythm[last] -= ((somme-nbt*precision) as f64)/(precision as f64);}
    return list_rhythm;
}


/// Génère la suite de note nécessaire à la crétion de la mélodie en fonction du nombre de rythme généré par markov_rhythm.
/// Retourne la suite de note sous forme d'un vecteur de note (structure note).
fn markov_notes(nbn : i32, tab : &Vec<Vec<f64>>) -> Vec<notes::Note>
{
    let mut rng = rand::thread_rng();
    let mut list_notes : Vec<notes::Note> = vec![];
    let mut last_note : i32 = 0;
    for _note in 0..nbn
    {
        let proba : f64 = rng.gen::<f64>();
        let mut somme_prob : f64 = 1.0;
        let mut i : i32 = 0;
        while somme_prob > proba
        {
            somme_prob -= tab[last_note as usize][i as usize];
            i+=1;
        }
        last_note = i;
        //liste_note_names[i as usize] = notes::NoteNames::Ad;
        match i     // TODO : repasser les noms des notes en liste
        {
            1 => list_notes.push(notes::Note{note : notes::NoteNames::A, octave : 5, velocity : midly::num::u7::new(0)}),
            2 => list_notes.push(notes::Note{note : notes::NoteNames::A, octave : 5, velocity : midly::num::u7::new(100)}),
            3 => list_notes.push(notes::Note{note : notes::NoteNames::Ad, octave : 5, velocity : midly::num::u7::new(100)}),
            4 => list_notes.push(notes::Note{note : notes::NoteNames::B, octave : 5, velocity : midly::num::u7::new(100)}),
            5 => list_notes.push(notes::Note{note : notes::NoteNames::C, octave : 5, velocity : midly::num::u7::new(100)}),
            6 => list_notes.push(notes::Note{note : notes::NoteNames::Cd, octave : 5, velocity : midly::num::u7::new(100)}),
            7 => list_notes.push(notes::Note{note : notes::NoteNames::D, octave : 5, velocity : midly::num::u7::new(100)}),
            8 => list_notes.push(notes::Note{note : notes::NoteNames::Dd, octave : 5, velocity : midly::num::u7::new(100)}),
            9 => list_notes.push(notes::Note{note : notes::NoteNames::E, octave : 5, velocity : midly::num::u7::new(100)}),
            10 => list_notes.push(notes::Note{note : notes::NoteNames::F, octave : 5, velocity : midly::num::u7::new(100)}),
            11 => list_notes.push(notes::Note{note : notes::NoteNames::Fd, octave : 5, velocity : midly::num::u7::new(100)}),
            12 => list_notes.push(notes::Note{note : notes::NoteNames::G, octave : 5, velocity : midly::num::u7::new(100)}),
            13 => list_notes.push(notes::Note{note : notes::NoteNames::Gd, octave : 5, velocity : midly::num::u7::new(100)}),
            _ => list_notes.push(notes::Note{note : notes::NoteNames::A, octave : 5, velocity : midly::num::u7::new(100)}),     // normalement impossible mais en cas d'erreur, on renvoie la fondamentale
        }
    }
    return list_notes;
}


/// Assemble la suite de note et de rythme pour en faire une mélodie.
/// Retourne la mélodie sous la forme d'un vecteur de triplet (note, instant de début, instant de fin).
fn construct_notes_vec(list_rhythm : Vec<f64>, list_notes : Vec<notes::Note>) -> Vec<(notes::Note, f64, f64)>
{
    let mut notes_vec : Vec<(notes::Note, f64, f64)> = vec![];
    let mut instant  : f64 = 0.0;
    for i in 0..list_rhythm.len()
    {
        if list_notes[i].velocity != 0 {notes_vec.push((list_notes[i], instant, instant+list_rhythm[i]));}
        instant += list_rhythm[i];
    }
    return notes_vec;
}


/// Récupère la mélodie générée par construct_notes_vec() et ajuste chaque note à la gamme supposé du fichier d'analyse.
/// Pas de retour.
fn scaling(notes_vec : &mut Vec<(notes::Note, f64, f64)>, anafile : &analysis_file::AnalysisData)
{
    for song in notes_vec {
        song.0.quantize_to_scale(&anafile.scale);
    }
}