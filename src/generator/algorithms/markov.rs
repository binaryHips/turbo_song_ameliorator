use crate::utils::{notes, analysis_file};
use midly;
use rand::Rng;

///
fn markov(ana_file : &analysis_file::AnalysisData, start_time : f64, end_time : f64) -> Vec<(notes::Note, f64, f64)>
{
    let rhythm_prob : Vec<Vec<f64>> = vec![vec![0.25,0.25,0.25,0.25],
                                           vec![0.25,0.25,0.25,0.25],
                                           vec![0.25,0.25,0.25,0.25],
                                           vec![0.25,0.25,0.25,0.25]];        // [1/4, 1/2, 3/4, 1]
    let note_prob : Vec<Vec<f64>> = vec![
    vec![0.01563262939453125, 0.006930461153388023, 7.450580596923828e-09, 0.031494140625, 8.344650268554688e-07, 0.0078277587890625, 0.0, 0.000125885009765625, 0.875, 0.06298828311264515, 0.0, 0.0, 0.0],
    vec![0.002386302915381555, 8.113423541768971e-07, 0.11527177541240174, 1.1936780908962997e-07, 0.11111111111113324, 5.688081210902803e-05, 1.5906100562185203e-13, 0.08888888888888886, 0.666682700164444, 8.118076074445204e-07, 6.694486768596467e-17, 0.015488361630451994, 0.0001122365472598027],
    vec![1.0530654697190513e-05, 0.1319535240213249, 0.005208333333333333, 0.5637691294381509, 0.11111111236755103, 0.006970750619622148, 0.006944448828563412, 1.7042250602341306e-14, 0.11810821762242332, 0.00021043771645364522, 2.984098305896557e-11, 0.00015782828286240794, 0.05555568708515952],
    vec![0.7571046930944606, 4.0149797292654476e-05, 0.00131016534559952, 0.12353382633943286, 0.0003074869004840996, 0.05882357239191976, 1.3368983957219262e-05, 3.819709702062648e-07, 1.336905336565808e-05, 1.3368983957219262e-05, 2.3578454951003994e-09, 0.05882586382578745, 1.3750954927425525e-05],
    vec![1.952852936201804e-05, 0.0011553122236326561, 0.9547511570784141, 0.005429864253689643, 0.03257948799033228, 0.0006121525537252163, 0.0009049773755662775, 8.618833481470847e-09, 2.262444090062084e-05, 5.130279454139844e-11, 6.41282153887196e-12, 4.672792636286591e-19, 0.004524886877828088],
    vec![0.00189393939398842, 0.043564879361068225, 0.5017171717174537, 0.08713782854134501, 0.0038888901134898476, 2.711148896270713e-11, 0.35232756830181905, 0.0, 0.007575773407298292, 4.871243297169223e-09, 0.0, 0.0018939442651826912, 0.0],
    vec![1.5539841667661225e-08, 0.006853070185529404, 9.791109331254344e-07, 0.2505113232832294, 2.2704314124829706e-09, 0.01644736963194939, 0.005239802622699038, 0.687504430277679, 0.027960526517605604, 1.1099886905472304e-09, 0.005208356643095835, 0.0002741228070175439, 0.0],
    vec![0.125, 0.00017361126733299342, 3.254596469197125e-14, 0.0017361176723775945, 0.21716391241515393, 0.00019152618090351235, 0.022049989008316437, 6.651059963974636e-14, 0.6319473522220062, 0.001736111427457888, 1.3797419106041398e-06, 6.346463949446312e-11, 9.772134526743167e-13],
    vec![0.026325757575757582, 0.0, 9.968102073365234e-06, 0.0002196922055305376, 1.9545298183069083e-07, 0.5824861484914939, 6.843908316915283e-06, 0.013178026393970668, 0.1606559041703803, 0.006581179807947901, 0.15789473684210528, 0.052641547049441795, 0.0],
    vec![0.25192307698065114, 0.004807692307869937, 0.25000000000000006, 0.10000404007774716, 1.414027167084302e-05, 1.6964106986574313e-05, 0.007470211549017844, 2.8776073561110325e-09, 0.00012959335494014336, 8.738177544304977e-09, 7.855722379720197e-07, 0.010633484163093614, 0.37500000000000006],
    vec![0.050069444444785415, 0.17500231484209158, 0.0, 0.00834490740740741, 0.0, 0.0, 2.3148149511986067e-05, 0.0004167781399585743, 0.763017795163097, 0.0020833375885076257, 0.00041666879425381276, 0.00020892071922919812, 0.00041668475115740755],
    vec![0.006945891203703704, 3.750857338820301e-08, 0.0, 0.0, 0.0, 0.00695216049382716, 8.487654320987655e-05, 0.5008101851851852, 0.08801774691358025, 1.6075102880658437e-08, 0.02160649755658436, 0.125, 0.2505825885202332],
    vec![0.6930649429821008, 0.0004775149184453323, 2.02271993917775e-05, 0.016673280426043057, 0.0, 0.00047619047619047614, 1.1050554074781304e-11, 0.1666666666666666, 0.11071429222543, 0.0023826535391609535, 0.00833352229780801, 0.0011907092577121813, 0.0],
    vec![0.7625873962851015, 0.000903746832445638, 0.001190476401582028, 0.008933624150939118, 0.01887760553466224, 1.4408541598913807e-05, 0.026913520439349508, 4.2517785723969164e-05, 0.1715136236333705, 2.498336479536565e-08, 4.205964124959991e-11, 9.448224138607519e-06, 0.009013607145663476]
    ];
    let dureet : f64 = 1.0/(ana_file.bpm*60.0);        // durée d'un temp en seconde
    let mut start_temp  : f64 = ana_file.first_instant;
    while start_temp < start_time {start_temp += dureet;}
    let mut end_temp  : f64 = start_temp;
    while end_temp < end_time {end_temp += dureet;}
    let nbt : i32 = ((end_temp - start_temp)/dureet) as i32;        // nombre de temps à jouer
    let list_rhythm : Vec<f64> = markov_rhythm(nbt, 4, &rhythm_prob);
    let nbn : i32 = list_rhythm.len() as i32;      // nombre de notes
    let list_notes : Vec<notes::Note> = markov_notes(nbn, &note_prob);
    let melody : Vec<(notes::Note, f64, f64)> = construct_melody(list_rhythm, list_notes);
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
        }
    }
    return list_notes;
}


fn construct_melody(list_rhythm : Vec<f64>, list_notes : Vec<notes::Note>) -> Vec<(notes::Note, f64, f64)>
{
    let mut melody : Vec<(notes::Note, f64, f64)>;
    let mut instant  : f64 = 0.0;
    for i in 0..list_rhythm.len()
    {
        if list_notes[i].velocity != 0 {melody.push((list_notes[i], instant, instant+list_rhythm[i]));}
        instant += list_rhythm[i];
    }
    return melody;
}


fn scaling(melody : &Vec<(notes::Note, f64, f64)>, anafile : &analysis_file::AnalysisData)
{
    for song in melody {song.0.quantize_to_scale(anafile.scale);}
}