use midly::*;

fn midi_generator(melody : &Vec<(Note, f64, f64)>, pathstring:&str)
{
    let mut mid : Smf = Smf::new(Header::new(SingleTrack,Timecode(Fps, u8)));
    let mut track : Track;
    let list_event : Vec<(Note, f64, bool)> = make_list_event(&melody);     // (Note, timing, is_pressed) ordonned
    for event in list_event
    {
        let fevent : TrackEvent = make_fevent(&event);    // event formated
        track.push(fevent);
    }
    mid.tracks.push(track);
    save_midi(&mid, &pathstring);
}


fn make_list_event(&melody : &Vec<(Note, f64, f64)>) -> Vec<(Note, f64, bool)>
{
    let mut list_event : Vec<(Note, f64, bool)>;
    let mut isong = 0;
    while melody[isong][0].velocity == 0 {isong += 1;}
    list_event.push((melody[isong][0], melody[isong][1], true));
    list_event.push((melody[isong][0], melody[isong][2], false));
    while isong < melody.len()
    {
        let song = melody[isong];
        if song[0].velocity != 0
        {
            let mut i = list_event.len()-1;
            if song[1] >= list_event[i][1]
            {
                list_event.push((song[0], song[1], true));
                list_event.push((song[0], song[2], false));
            }
            else
            {
                while i >= 0 && song[1] < list_event[i][1] {i -= 1;}
                i += 1;
                list_event.insert(i, (song[0], song[1], true));
                while i < list_event.len() && song[2] > list_event[i][1] {i += 1;}
                list_event.insert(i, (song[0], song[2], false));
            }
        }
        isong += 1;
    }
    return list_event;
}


fn make_fevent(event : &(Note, f64, bool)) -> TrackEvent
{
    if event[2] {let midi_message  : MidiMessage = (NoteOn(event[0].to_midi(), event[0].velocity));}
    else {let midi_message  : MidiMessage = (NoteOff(event[0].to_midi(), event[0].velocity));}
    let track_event_kind : TrackEventKind = TrackEventKind::Midi(0, midi_message);  // {0; midi_message};
    
    let track_event = TrackEvent.new(, track_event_kind);
}