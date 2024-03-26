use fundsp::hacker32::*;
use std::time::{Duration, Instant};

///Computes the volume of a slice of audio
///Returns the RMS volume, which is in [0, 1] (I think?)

fn get_slice_volume_RMS(signal_slice: &[f32]) -> f32{

    let mut sum:f32 = 0.0;


    for sample in signal_slice{
        sum += sample.powf(2.0) as f32;
    }

    return (sum/ (2.0 * signal_slice.len() as f32)).sqrt();
}


pub fn get_volumes_RMS(wav: &Wave32, interval_seconds:f64) -> Vec<f32> {

    const WINDOW_SIZE_SECONDS:f64 = 0.3; //.3s seems usual for RMS metering

    let samplerate = wav.sample_rate();

    let window_span:usize = (WINDOW_SIZE_SECONDS * samplerate/2.0) as usize;


    let mut volumes: Vec<f32> = Vec::new();


    let signal: &Vec<f32> = &wav.channel(0);


    for i in (window_span..wav.len()-window_span).step_by((interval_seconds*samplerate) as usize) {

        let signal_slice: &[f32] = &signal[(i-window_span)..(i+window_span)];

        volumes.push(
            get_slice_volume_RMS(signal_slice)
        );
    

    }

    return volumes;


}

pub fn get_volumes_RMS_3bands(wav: &Wave32, interval_seconds:f64) -> Vec<(f32, f32, f32)>{


    const WINDOW_SIZE_SECONDS:f64 = 0.3; //.3s seems usual for RMS metering

    const low_threshold:f32 = 200.0;
    const high_threshold:f32 = 5000.0;

    let samplerate = wav.sample_rate();

    let window_span:usize = (WINDOW_SIZE_SECONDS * samplerate/2.0) as usize;
    let length = wav.length() as f64 / samplerate;
    let mut wav = wav.clone(); wav.remove_channel(1);

    let start = Instant::now();
    let lp = wav.filter(length, &mut lowpass_hz(low_threshold, 1.0));
    let hp = wav.filter(length, &mut highpass_hz(high_threshold, 1.0));
    let mid = lp.filter(length, &mut highpass_hz(high_threshold, 1.0));

    let duration = start.elapsed();
    println!("Time for all filters : {:?}", duration);


    let lp_signal = lp.channel(0);
    let hp_signal = hp.channel(0);
    let mid_signal = mid.channel(0);

    let mut volumes: Vec<(f32, f32, f32)> = Vec::new();



    for i in (window_span..wav.len()-window_span).step_by((interval_seconds*samplerate) as usize) {

        volumes.push((
            get_slice_volume_RMS(
                &lp_signal[(i-window_span)..(i+window_span)])
            ,
            get_slice_volume_RMS(
                &mid_signal[(i-window_span)..(i+window_span)])
            ,
            get_slice_volume_RMS(
                &hp_signal[(i-window_span)..(i+window_span)])

        ));
    

    }

    return volumes;

}