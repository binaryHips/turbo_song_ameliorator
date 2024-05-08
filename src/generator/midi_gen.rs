use midly::*;
use crate::utils::{*, notes::*};

/// Génère un fichier midi (.mid) reproduisant la mélodie en entré
/// Le fichier est créé dans le dossier assets/generation
pub fn midi_generator(melody : &Vec<(Note, f64, f64)>, analysis_data : &analysis_file::AnalysisData, pathstring : &str)
{
    let mut mid : Smf = Smf::new(Header::new(Format::SingleTrack, Timing::Metrical(num::u15::new((1920*60/(analysis_data.bpm as i64)) as u16))));      // Timecode(Fps::Fps24, 80), 24*80 = 1920
    let mut track : Track = vec![];
    track.push(TrackEvent{delta : num::u28::new(0 as u32), kind : TrackEventKind::Meta(MetaMessage::TrackName(&[0 as u8]))});
    track.push(TrackEvent{delta : num::u28::new(0 as u32), kind : TrackEventKind::Meta(MetaMessage::Tempo(num::u24::new((1000000*60/(analysis_data.bpm as i64)) as u32)))});
    track.push(TrackEvent{delta : num::u28::new(0 as u32), kind : TrackEventKind::Meta(MetaMessage::TimeSignature(4 as u8, 2 as u8, (1920*(analysis_data.bpm as i32)/60) as u8, 8 as u8))});
    let list_event : Vec<(Note, f64, bool)> = make_list_event(melody);     // (Note, timing, is_pressed) ordonned
    track.push(make_fevent(&list_event[0], list_event[0].1));
    for i in 1..(list_event.len())
    {
        let time_passed : f64 = list_event[i].1 - list_event[i-1].1;
        let fevent : TrackEvent = make_fevent(&list_event[i], time_passed);    // event formated
        track.push(fevent);
    }
    track.push(TrackEvent{delta : num::u28::new(0 as u32), kind : TrackEventKind::Meta(MetaMessage::EndOfTrack)});
    mid.tracks.push(track);
    file_import::save_midi(&mid, &pathstring);
}


/// Convertit la mélodie (suite de (note, instant début, instant fin) en suite d'évenement (note, instant, pressé ou relaché).
/// Retourne la suite d'évenement sous forme de Veccteur de triplet (Note, f64, bool).
fn make_list_event(melody : &Vec<(Note, f64, f64)>) -> Vec<(Note, f64, bool)>
{
    let mut list_event : Vec<(Note, f64, bool)> = vec![];
    let mut isong = 0;
    while melody[isong].0.velocity == 0 {isong += 1;}
    list_event.push((melody[isong].0, melody[isong].1, true));
    list_event.push((melody[isong].0, melody[isong].2, false));
    while isong < melody.len()
    {
        let song = melody[isong];
        if song.0.velocity != 0
        {
            let mut i = list_event.len()-1;
            if song.1 >= list_event[i].1
            {
                list_event.push((song.0, song.1, true));
                list_event.push((song.0, song.2, false));
            }
            else
            {
                while i >= 0 && song.1 < list_event[i].1 {i -= 1;}
                i += 1;
                list_event.insert(i, (song.0, song.1, true));
                while i < list_event.len() && song.2 > list_event[i].1 {i += 1;}
                list_event.insert(i, (song.0, song.2, false));
            }
        }
        isong += 1;
    }
    return list_event;
}


/// Convertit un évenement en TrackEvent, qui est le type formaté par Rust pour les évenement midi
/// Retourne le TrackEvent correspondant à l'évenement
fn make_fevent(event : &(Note, f64, bool), time_passed : f64) -> TrackEvent
{

    let midi_message : MidiMessage;
    if event.2 {midi_message = MidiMessage::NoteOn{key : event.0.to_midi(), vel : event.0.velocity};}
    else {midi_message = MidiMessage::NoteOff{key : event.0.to_midi(), vel : event.0.velocity};}
    let track_event_kind : TrackEventKind = TrackEventKind::Midi{channel : num::u4::new(0 as u8), message : midi_message};
    let tick_per_sec = 1920; // FPS*subframe (80)
    let track_event = TrackEvent{delta : num::u28::new((time_passed*(tick_per_sec as f64)) as u32), kind : track_event_kind};
    return track_event;
}