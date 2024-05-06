use crate::utils::{notes, analysis_file};
use midly;
use rand::Rng;

fn markov(ana_file : &midly::Smf, start_time : f64, end_time : f64) -> Vec<(notes::Note, f64, f64)>
{
    let rhythm_prob : Vec<Vec<f64>> = vec![vec![0.25,0.25,0.25,0.25],
                                           vec![0.25,0.25,0.25,0.25],
                                           vec![0.25,0.25,0.25,0.25],
                                           vec![0.25,0.25,0.25,0.25]];        // [1/4, 1/2, 3/4, 1]
    let note_prob : Vec<Vec<f64>> = vec![vec![1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0],        // s'il est au début
                                         vec![1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0],        // s'il y avait un silence avant
                                         vec![1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0],        // note précedente à partir de là
                                         vec![1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0],
                                         vec![1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0],
                                         vec![1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0],
                                         vec![1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0],
                                         vec![1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0],
                                         vec![1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0],
                                         vec![1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0],
                                         vec![1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0],
                                         vec![1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0],
                                         vec![1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0],
                                         vec![1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0,1.0/13.0]];       // notes de A à G# + silence
    let dureet : f64 = 1.0/((analysis_file::get_bpm(&ana_file)*60) as f64);        // durée d'un temp en seconde
    let mut start_temp  : f64 = analysis_file::get_first_instant(&ana_file);
    while start_temp < start_time {start_temp += dureet;}
    let mut end_temp  : f64 = start_temp;
    while end_temp < end_time {end_temp += dureet;}
    let nbt : i32 = ((end_temp - start_temp)/dureet) as i32;        // nombre de temps à jouer
    let list_rhythm : Vec<f64> = markov_rhythm(nbt, 4, &rhythm_prob);
    let nbn : i32 = list_rhythm.len() as i32;      // nombre de notes
    let list_notes : Vec<notes::Note> = markov_notes(nbn, &note_prob);
    let melody : Vec<(notes::Note, f64, f64)> = construct_melody(list_rhythm, list_notes, start_temp);
    scaling(&melody, &ana_file);
    return melody;
}


fn markov_rhythm(nbt : i32, precision : i32, tab : &Vec<Vec<f64>>) -> Vec<f64>
{
    let mut rng = rand::thread_rng();
    let mut somme : i32 = 0;
    let mut list_rhythm : Vec<f64>;
    while somme < nbt*precision
    {
        let proba : f64 = rng.gen::<f64>();
        let somme_prob : f64 = 1.0;
        let mut i : i32 = 0;
        while somme_prob > proba
        {
            somme_prob -= tab[(somme%precision) as usize][i as usize];
            i+=1;
        }
        // le i représente au début le position dans la liste de 0 à 3 mais fini à 1 de trop, or,
        // on l'utilise ensuite seulement pour les quarts allant de 1 à 4, donc tout va bien
        somme += i;
        list_rhythm.push((i as f64)/(precision as f64));
    }
    if somme > nbt*precision {list_rhythm[list_rhythm.len()-1] -= ((somme-nbt*precision) as f64)/(precision as f64);}
    return list_rhythm;
}


fn markov_notes(nbn : i32, tab : &Vec<Vec<f64>>) -> Vec<notes::Note>
{
    let mut rng = rand::thread_rng();
    let mut list_notes : Vec<notes::Note>;
    let mut last_note : i32 = 0;
    for note in 0..nbn
    {
        let proba : f64 = rng.gen::<f64>();
        let somme_prob : f64 = 1.0;
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
        }
    }
    return list_notes;
}


fn construct_melody(list_rhythm : Vec<f64>, list_notes : Vec<notes::Note>, start_temp  : f64) -> Vec<(notes::Note, f64, f64)>
{
    let mut melody : Vec<(notes::Note, f64, f64)>;
    let mut instant  : f64 = start_temp;
    for i in 0..list_rhythm.len()
    {
        if list_notes[i].velocity != 0 {melody.push((list_notes[i], instant, instant+list_rhythm[i]));}
        instant += list_rhythm[i];
    }
    return melody;
}


fn scaling(melody : &Vec<(notes::Note, f64, f64)>, anafile : &midly::Smf)
{
    for song in melody {song.0.quantize_to_scale(analysis_file::get_scale(&anafile));}
}