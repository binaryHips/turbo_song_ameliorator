use fundsp::hacker32::*;
use std::time::{Duration, Instant};

/// Trouve le BPM d'une musique (ou un BPM cohérent) à partir d'un fichier wave
/// Retourne le BPM trouvé en un entier sur 32 bits
pub fn find_wave_bpm(wave : &Wave32) -> i32
{
    let now = Instant::now();
    println!("start");
    const min : i32 = 30;
    const max : i32 = 240;
    let mut bpm = 0;
    let mut correlation = 0.0;
    let sr = wave.sample_rate();
    let len = wave.len() as i32;    // nombre de sample
    // Optimisation : remplacer le for par un parcours de liste auquel on enlève les multiples des bpm trop faible
    let mut tab = [0 ; (max-min+1) as usize];
    for i in min..max+1
    {
        tab[(i-min) as usize] += i;
    }
    for test in min..max+1             // le bpm qu'on test
    {
        if tab[(test-min) as usize] != 0
        {
            let st = 60*(sr as i32)/test;    // nombre de sample par temps
            let mut sum = 0.0;
            for sample in st..len        
            {
                // Les calculs des samples sont arrondies à l'inférieurs mais comme on les parcours un par un ensuite, le décallage ne se cumule pas
                sum += wave.at(0 as usize, sample as usize)*wave.at(0 as usize, (sample-st) as usize);
            }
            if sum > correlation
            {
                let mut i = 2;
                while bpm*i <= max
                {
                    if (((bpm*i) as f64)/(test as f64)) as i32 != bpm*i/test
                    {
                        tab[((bpm*i)-min) as usize] == 0;
                    }
                    i += 1;
                }
                bpm = test;
                correlation = sum;
            }
            else
            {
                let mut i = 2;
                while test*i <= max
                {
                    if (((test*i) as f64)/(bpm as f64)) as i32 != test*i/bpm
                    {
                        tab[((test*i)-min) as usize] == 0;
                    }
                    i += 1;
                }
            }
        }
    }
    let now2 = Instant::now();
    println!("{:?}", now2.duration_since(now));
    return bpm;
}


/// Trouve le premier sample du début du premier temps d'une musique
/// à partir d'un fichier wave et du bpm supposé de la musique
/// Retourne le numéro du sample en un entier sur 32 bits
pub fn find_wave_first_sample(wave : &Wave32, bpm : i32) -> i32
{
    let now = Instant::now();
    println!("start");
    let mut first = 0;
    let mut correlation = 0.0;
    let sr = wave.sample_rate();
    let st = 60*(sr as i32)/bpm;    // nombre de sample par temps
    let len = wave.len() as i32;    // nombre de sample
    for sample in 0..st   // le sample testé dans le premier interval de temps au BPM 
    {
        let mut sum = 0.0;
        for temp in 1..(len*bpm/60/(sr as i32))     // le temps dans la musique en partant du sample testé
        {
            sum += wave.at(0 as usize, (temp*st+sample) as usize)*wave.at(0 as usize, ((temp-1)*st+sample) as usize);
        }
        if sum > correlation
        {
            first = sample;
            correlation = sum;
        }
    }
    let now2 = Instant::now();
    println!("{:?}", now2.duration_since(now));
    return first;
}


/// Fonction de test
pub fn test(wave : &Wave32)
{
    let bpm = find_wave_bpm(wave);
    let first = find_wave_first_sample(wave, bpm);
    println!("{bpm}, {first}");
}