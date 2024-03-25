use fundsp::hacker32::*;

///Computes the volume of a slice of audio
///Returns the RMS volume, which is in [0, 1] (I think?)

fn get_slice_volume_RMS(signal_slice: &Vec<f32>) -> f32{

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

        let signal_slice: &Vec<f32> = &signal[(i-window_span)..(i+window_span)].to_vec();

        volumes.push(
            get_slice_volume_RMS(signal_slice)
        );
    

    }

    return volumes;


}

// pub fn get_volumes_RMS_3bands(wav: &Wave32, interval_seconds:f64) -> (Vec<f32>, Vec<f32>, Vec<f32>){


//     const WINDOW_SIZE_SECONDS:f32 = 0.3; //.3s seems usual for RMS metering

//     const low_threshold:usize = 500;
//     const high_threshold:usize = 2000;

//     let samplerate = wav.sample_rate();

//     let window_span:usize = (WINDOW_SIZE_SECONDS * samplerate/2.0) as usize;


//     let mut volumes_low: Vec<f32> = Vec::new();
//     let mut volumes_mid: Vec<f32> = Vec::new();
//     let mut volumes_high: Vec<f32> = Vec::new();


//     let signal: &Vec<f32> = &wav.channel(0);


//     for i in (window_span..wav.len()-window_span).step_by((interval_seconds*samplerate) as usize) {

//         let signal_slice: &Vec<f32> = &signal[(i-window_span)..(i+window_span)].to_vec();

//         volumes.push(
//             get_slice_volume_RMS(signal_slice)
//         );
    

//     }

//     return volumes;

// }