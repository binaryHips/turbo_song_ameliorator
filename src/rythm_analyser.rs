use fundsp::hacker32::*;

/// Trouve le BPM d'une musique (ou un BPM cohérent) à partir d'un fichier wave
/// Retourne le BPM trouvé en un entier sur 32 bits
pub fn find_wave_bpm(wave : Wave32) -> i32
{
    let mut bpm = 0;
    let mut correlation = 0.0;
    let sr = wave.sample_rate();
    let len = wave.len() as i32;    // nombre de sample
    // Optimisation : remplacer le for par un parcours de liste auquel on enlève les multiples des bpm trop faible
    for test in 30..241             // le bpm qu'on test
    {
        let mut sum = 0.0;
        for sample in (60*(sr as i32)/test)..len        
        {
            // Les calculs des samples sont arrondies à l'inférieurs mais comme on les parcours un par un ensuite, le décallage ne se cumule pas
            sum += wave.at(0 as usize, sample as usize)*wave.at(0 as usize, (sample-(60*(sr as i32)/test)) as usize);
        }
        if sum > correlation
        {
            bpm = test;
            correlation = sum;
        }
    }
    return bpm;
}


/// Trouve le premier sample du début du premier temps d'une musique
/// à partir d'un fichier wave et du bpm supposé de la musique
/// Retourne le numéro du sample en un entier sur 32 bits
pub fn find_wave_first_sample(wave : Wave32, bpm : i32) -> i32
{
    let mut first = 0;
    let mut correlation = 0.0;
    let sr = wave.sample_rate();
    let len = wave.len() as i32;    // nombre de sample
    for sample in 0..(60*(sr as i32)/bpm)
    {
        for temp in 0..(len*bpm/60/(sr as i32))
        {
            // à faire
        }
    }
    return 0;
}