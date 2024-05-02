use midly::*;
use crate::utils::{*, notes::*};

fn midi_generator(melody : &Vec<(Note, f64, f64)>, pathstring:&str)
{
    let mut mid : Smf = Smf::new(Header::new(Format::SingleTrack, Timing::Timecode(Fps::Fps24, 80)));
    let mut track : Track;
    let list_event : Vec<(Note, f64, bool)> = make_list_event(&melody);     // (Note, timing, is_pressed) ordonned
    let mut time_passed : f64 = 0.0;
    track.push(make_fevent(&list_event[0], list_event[0].1));
    for i in 1..(list_event.len())
    {
        time_passed = list_event[i].1 - list_event[i-1].1;
        let fevent : TrackEvent = make_fevent(&list_event[i], time_passed);    // event formated
        track.push(fevent);
    }
    mid.tracks.push(track);
    file_import::save_midi(&mid, &pathstring);
}


fn make_list_event(&melody : &Vec<(Note, f64, f64)>) -> Vec<(Note, f64, bool)>
{
    let mut list_event : Vec<(Note, f64, bool)>;
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


fn make_fevent(event : &(Note, f64, bool), time_passed : f64) -> TrackEvent
{
    let mut midi_message : MidiMessage;
    if event.2 {midi_message = MidiMessage::NoteOn{key : event.0.to_midi(), vel : event.0.velocity};}
    else {midi_message = MidiMessage::NoteOff{key : event.0.to_midi(), vel : event.0.velocity};}
    let track_event_kind : TrackEventKind = TrackEventKind::Midi{channel : num::u4::new(0 as u8), message : midi_message};
    let tick_per_sec = 1920; // FPS*subframe (80)
    let track_event = TrackEvent{delta : num::u28::new((time_passed*(tick_per_sec as f64)) as u32), kind : track_event_kind};
    return track_event;
}