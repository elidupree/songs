extern crate codecophony;
extern crate hound;
extern crate portaudio;
extern crate dsp;
extern crate rand;

use codecophony::*;
use codecophony::phrase::Phrase;
use rand::{Rng, SeedableRng};
use std::iter::FromIterator;
use std::collections::HashMap;


pub const SAMPLE_HZ: f64 = 44100.0;
pub const CHANNELS: usize = 2;
pub type Output = f32;

pub fn current_input_playground (input: & HashMap <String, Phrase>) -> (Box<Renderable<[Output; CHANNELS]> + Send>, Vec<Phrase>) {
  /*let notes = input.get("first_test").unwrap().to_midi_pitched (| note | {
    let instrument = if note.tags.contains ("melody") {61} else {43};
    (90, instrument)
  });*/
  
  let mut notes = Vec::new();
  
  let mut arising: Vec<_> = input.get("arising").unwrap().notes.iter().map(| note | {
    let instrument = if note.tags.contains ("melody") {57} else {43};
    let velocity = if note.tags.contains ("first") {90} else {60};
    let mut duration = note.end - note.start;
    if note.tags.contains ("melody") {duration *= 0.8;};
    (note, MIDIPitchedNote::new (note.start, duration, frequency_to_nearest_midi_pitch (note.frequency), velocity, instrument))
  }).collect();
  for note in arising.iter_mut() {
    note.1.dilate(3.0/4.0, 0.0);
  }
  let striking: Vec<_> = input.get("striking").unwrap().to_midi_pitched (| note | {
    let instrument = if note.tags.contains ("melody") {57} else {43};
    let velocity = if note.tags.contains ("emphasis") {120} else if note.tags.contains ("emphasis") {60} else {90};
    (velocity, instrument)
  }).into_iter().map(|mut note| {
    note.nudge(12.0/4.0);
    note
  }).collect();
  notes.extend(arising.iter().map(|n| n.1.clone()));
  notes.extend(striking.iter().cloned());
  notes.extend(arising.iter().map(|n| n.1.clone()).map(|mut note| {
    note.nudge(29.0/4.0);
    note
  }));
  notes.extend(arising.iter().cloned().filter_map(|mut note| {
    if note.0.tags.contains ("bass") {
      note.1.nudge(29.0/4.0);
      note.1.transpose(7);
      Some(note.1)
    } else {None}
  }));
  notes.extend(striking.iter().cloned().map(|mut note| {
    note.nudge(29.0/4.0);
    note
  }));
  let phrases = vec![Phrase::from_iter (notes.iter())];
  (Box::new(notes), phrases)
}

pub fn current_playground() -> (Box<Renderable<[Output; CHANNELS]> + Send>, Vec<Phrase>) {
  
  /*let note = codecophony::SineWave {
    start:0.0, duration:1.0,
    frequency: 265.0, amplitude: 0.25,
  };*/
  
  /*let mut notes: Vec<_> = (0..100u32).map(|index| codecophony::SineWave {
    start: index as f64 * 0.3, duration:1.0,
    frequency: 220.0, amplitude: 0.1,
  }).collect();
  
  
  codecophony::interval_optimizer::optimize_notes (&mut notes,
    codecophony::interval_optimizer::OptimizeNotesParameters {max_change_ratio: 2.0, .. Default::default()},
    |(_note, frequency), neighbors| {
      let mut result = 0.0;
      for &(_, neighbor_frequency) in neighbors.iter() {
        let interval = codecophony::interval_optimizer::closest_reference_interval (frequency/neighbor_frequency);
        let error = ((interval.frequency()/frequency)-1.0).powi(2);
        let limit_score = if interval.odd_limit == 1 {
          if ((frequency/neighbor_frequency)-1.0).abs() < 0.5 {
            // unison bad!
            -13.0
          }
          else {
            // octave ok
            -5.0
          }
        }
        else {
          //(interval.odd_limit as f64).ln()
          -interval.odd_limit as f64
        };
        result += limit_score - error;
      }
      result
    }
  );
  
  for note in notes.iter_mut() {note.amplitude *= 220.0/note.frequency;}*/
  
  /*let notes: Vec<_> = (0..100u32).map(|index|
    MIDIPitchedNote::new(index as f64 * 0.3, 1.0, 1+index as i32, 90, 3)
  ).collect();
  
  let notes: Vec<_> = (0..1000u32).map(|index|
    MIDIPercussionNote::new((index as f64 + 1.0).ln(), 1.0, 90, 35)
  ).collect();*/
  
  let beats: f64 = 4.0;
  use std::iter;
  let beat_weights: Vec<f64> =
    iter::repeat(0.0).take(8)
    .chain(iter::repeat(2.0).take(4))
    .chain(iter::repeat(1.0).take(2))
    .chain(iter::repeat(3.0).take(2))
    .chain(iter::repeat(0.5).take(1))
    .chain(iter::repeat(1.5).take(1))
    .chain(iter::repeat(2.5).take(1))
    .chain(iter::repeat(3.5).take(1))
    .collect();
  let step_weights: Vec<(f64, f64)> =
    iter::repeat((1.0,0.0)).take(1)
    .chain(iter::repeat((2.0,0.0)).take(1))
    .chain(iter::repeat((2.0,1.0)).take(1))
    .chain(iter::repeat((4.0,0.0)).take(1))
    .chain(iter::repeat((4.0,1.0)).take(1))
    .chain(iter::repeat((4.0,2.0)).take(1))
    .chain(iter::repeat((4.0,3.0)).take(1))
    .collect();
  
  let mut generator = rand::chacha::ChaChaRng::from_seed(&[35]);
  
  let mut notes = Vec::new();
  for instrument in 35..82 {
    if instrument == 58 || instrument == 71 || instrument == 72 || instrument == 78 || instrument == 79 {continue;}
    let &beat = generator.choose (& beat_weights).unwrap();
    let &(step, phase) = generator.choose (& step_weights).unwrap();
    let mut time = beat+beats*phase;
    while time < 60.0 {
      notes.push (
        MIDIPercussionNote::new(time/4.0, 1.0, 50, instrument)
      );
      time += step*beats;
    }
  }
  
  let phrases = vec![Phrase::from_iter (notes.iter())];
  (Box::new(notes), phrases)
}


