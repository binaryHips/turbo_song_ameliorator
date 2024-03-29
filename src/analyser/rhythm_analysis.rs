use fundsp::hacker32::*;
use std::time::{Duration, Instant};

/// Trouve le BPM d'une musique (ou un BPM cohérent) à partir d'un fichier wave
/// Retourne le BPM trouvé en un entier sur 32 bits
pub fn find_wave_bpm(wave : &Wave32) -> i32
{
    let now = Instant::now();
    println!("start");
    const MIN : i32 = 20;           // BPM testé
    const MAX : i32 = 240;          // BPM testé
    const SEC_ANALYSED : f64 = 20.0;
    const STEP : i32 = 50;        // nombre de sample sautés (optimisation)
    let mut bpm = 0;
    let mut correlation = 0.0;
    let sr = wave.sample_rate();
    let len = min(wave.len(), (SEC_ANALYSED*sr) as usize) as i32;      // nombre de sample à analyser
    let mut tab = [0 ; (MAX-MIN+1) as usize];                        // liste des BPM à tester
    for i in MIN..=MAX      {tab[(i-MIN) as usize] += i;}                   //initialisation de tab
    for test in MIN..=MAX             // le bpm qu'on test
    {
        if tab[(test-MIN) as usize] != 0            // les 0 sont les BPM retirés (optimisation)
        {
            let st = 60*(sr as i32)/test;    // nombre de sample par temps
            let mut sum = 0.0;
            for sample in (st..len).step_by(STEP as usize)     
            {sum += wave.at(0 as usize, sample as usize)*wave.at(0 as usize, (sample-st) as usize);}
            sum /= (((len-st)+(STEP-1))/STEP) as f32;       // on normalise
            // nombre d'itération = |{step*k < len-st|k dans N}| = partie_entiere(((len-st)+(step-1))/step)
            if sum > correlation
            {
                if correlation < sum/2.0        {remove_multiples(MAX, MIN, bpm, test, &mut tab);}
                println!("{} remplace {} car {} > {}", test, bpm, sum, correlation);
                bpm = test;
                correlation = sum;                
            }
            else if sum < correlation/2.0       {remove_multiples(MAX, MIN, test, bpm, &mut tab);}
        }
    }
    let now2 = Instant::now();
    println!("{:?}", now2.duration_since(now));
    println!("{:?}", tab);
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
        {sum += wave.at(0 as usize, (temp*st+sample) as usize)*wave.at(0 as usize, ((temp-1)*st+sample) as usize);}
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

fn remove_multiples(max_bpm : i32, min_bpm : i32, bad_bpm : i32, good_bpm : i32, values : &mut[i32])
{
    if min(bad_bpm, good_bpm) <= 1 {return;} //safeguard

    let mut i = 2;
    while bad_bpm*i <= max_bpm
    {
        if (bad_bpm*i)%good_bpm != 0        // BPM multiple de bad mais pas de good
        {
            (*values)[((bad_bpm*i)-min_bpm) as usize] = 0;
            println!("Enlevé: {} car multiple de {}, (comparé à {})", bad_bpm*i, bad_bpm, good_bpm);
        }
        i += 1;
    }
}


/// Fonction de test
pub fn test(wave : &Wave32)
{
    let bpm = find_wave_bpm(wave);
    let first = find_wave_first_sample(wave, bpm);
    println!("{bpm}, {first}");
}