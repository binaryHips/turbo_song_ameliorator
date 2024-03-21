
use fundsp::hacker32::*;
use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;
use pitch_detection::Pitch;

pub fn get_fundamental_note(signal: Vec<f64>, samplerate:f64)-> Pitch<f64>{
    const SIZE: usize = 1024;
    const PADDING: usize = SIZE / 2;
    const POWER_THRESHOLD: f64 = 5.0;
    const CLARITY_THRESHOLD: f64 = 0.7;

    let mut detector = McLeodDetector::new(SIZE, PADDING);
    let pitch = detector
        .get_pitch(&signal, samplerate as usize, POWER_THRESHOLD, CLARITY_THRESHOLD)
        .unwrap();

    return pitch;
}
