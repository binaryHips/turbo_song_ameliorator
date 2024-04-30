use crate::utils;
use midly;
use rand::Rng;

fn markov(ana_file : &Smf, start_time : f64, end_time : f64) -> Vec<(Note, f64, f64)>
{
    let rhythm_prob : Vec<Vec<f64>> = vec![vec![1/4,1/4,1/4,1/4],
                                           vec![1/4,1/4,1/4,1/4],
                                           vec![1/4,1/4,1/4,1/4],
                                           vec![1/4,1/4,1/4,1/4]];        // [1/4, 1/2, 3/4, 1]
    let note_prob : Vec<Vec<f64>> = vec![vec![1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13],        // s'il est au début
                                         vec![1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13],        // s'il y avait un silence avant
                                         vec![1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13],        // note précedente à partir de là
                                         vec![1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13],
                                         vec![1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13],
                                         vec![1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13],
                                         vec![1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13],
                                         vec![1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13],
                                         vec![1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13],
                                         vec![1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13],
                                         vec![1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13],
                                         vec![1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13],
                                         vec![1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13],
                                         vec![1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13,1/13]];       // notes de A à G# + silence
    let dureet : f64 = 1.0/((get_bpm(&ana_file)*60) as f64);        // durée d'un temp en seconde
    let mut start_temp  : f64 = get_first_instant(&anafile);
    while start_temp < start_time {start_temp += dureet;}
    let mut end_temp  : f64 = start_temp;
    while end_temp < end_time {end_temp += dureet;}
    let nbt : i32 = ((end_temp - start_temp)/dureet) as i32;        // nombre de temps à jouer
    let list_rhythm : Vec<f64> = markov_rhythm(nbt, 4, &rhythm_prob);
    let nbn : i32 = list_rhythm.len();      // nombre de notes
    let list_notes : Vec<Note> = markov_notes(nbn, &note_prob);
    let melody : Vec<(Note, f64, f64)> = construct_melody(list_rhythm, list_notes, start_temp);
    scaling(&melody, &anafile);
    return melody;
}


fn markov_rhythm(nbt : i32, precision : i32, tab : Vec<Vec<f64>>) -> Vec<f64>
{
    let mut rng = rand::thread_rng();
    let mut somme : i32 = 0;
    let mut list_rhythm : Vec<f64>;
    while somme < nbt*precision
    {
        let proba : f64 = rng.gen::<f64>();
        let somme_prob : f64 = 1;
        let mut i : i32 = 0;
        while somme_prob > proba
        {
            somme_prob -= tab[somme%precision][i];
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


fn markov_notes(nbn : i32, tab : Vec<Vec<f64>>) -> Vec<Note>
{
    let notes : Vec<Notes_name> = vec![A, As, B, C, Cs, D, Ds, E, F, Fs, G, Gs];
    let mut rng = rand::thread_rng();
    let mut list_notes : Vec<Note>;
    let mut last_note : i32 = 0;
    for note in 0..nbn
    {
        let proba : f64 = rng.gen::<f64>();
        let somme_prob : f64 = 1;
        let mut i : i32 = 0;
        while somme_prob > proba
        {
            somme_prob -= tab[last_note][i];
            i+=1;
        }
        last_note = i;
        if i == 1 {list_notes.push(Note(A, 5, 0));}
        else {list_notes.push(Note(notes[i-2], 5, 100));}
    }
    return list_notes;
}


fn construct_melody(list_rhythm : Vec<f64>, list_notes : Vec<Note>, start_temp  : f64) -> Vec<(Note, f64, f64)>
{
    let mut melody : Vec<(Note, f64, f64)>;
    let mut instant  : f64 = start_temp;
    for note in 0..list_rhythm.len()
    {
        if list_notes[i].velocity != 0 {melody.push((list_notes[i], instant, instant+list_rhythm[i]));}
        instant += list_rhythm[i];
    }
    return melody;
}


fn scaling(melody : &mut Vec<(Note, f64, f64)>, anafile : &Smf)
{
    for song in melody {song[0].quantize_to_scale(get_scale(&anafile));}
}