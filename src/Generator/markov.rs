use utils;
use midly;
use rand::Rng;

fn markov(ana_file : Smf, start_time:f64, end_time:f64) -> Vec<(Note, f64, f64)>
{
    let rhythm_prob : Vec<Vec<f64>> = vec![vec![1/4,1/4,1/4,1/4],
                                           vec![1/4,1/4,1/4,1/4],
                                           vec![1/4,1/4,1/4,1/4],
                                           vec![1/4,1/4,1/4,1/4]];        // [1/4, 1/2, 3/4, 1]
    let dureet : f64 = 1/anafile.bpm*60;        // durée d'un temp en seconde
    let mut start_temp  : f64 = anafile.first_instant;
    while start_temp < start_time {start_temp += dureet;}
    let mut end_temp  : f64 = start_temp;
    while end_temp < end_time {end_temp += dureet;}
    let nbt : i32 = ((end_temp - start_temp)/dureet) as i32;        // nombre de temps à jouer
    let list_rhythm : Vec<f64> = markov_rhythm(nbt, 4, &rhythm_prob);
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
        somme += i;
        list_rhythm.push((i as f64)/(precision as f64));
    }
    if somme > nbt*precisionb {list_rhythm[list_rhythm.len()-1] -= ((somme-nbt*precision) as f64)/(precision as f64);}
    return list_rhythm;
}