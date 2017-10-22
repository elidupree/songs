extern crate codecophony;
extern crate hound;
extern crate portaudio;
extern crate dsp;
extern crate rand;

use codecophony::*;
use codecophony::phrase::{Phrase, PhraseNote};
use rand::{Rng, SeedableRng, ChaChaRng};
use std::iter::FromIterator;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use std::rc::Rc;


pub const SAMPLE_HZ: f64 = 44100.0;
pub const CHANNELS: usize = 2;
pub type Output = f32;

fn applied<F: FnMut(&mut PhraseNote)> (mut collection: Vec<PhraseNote>, mut callback: F) -> Vec<PhraseNote> {
  for note in collection.iter_mut() {callback (note);}
  collection
}

fn find_tag (collection: &Vec<PhraseNote>, item: & str)->PhraseNote {
  // I don't want to have fallback cases all over the place, but
  // make a meaningless default so that we don't crash
  collection.iter().find (| whatever | whatever.tags.contains (item)).cloned().unwrap_or (PhraseNote::new (0.0, 0.0, 100.0))
}

fn concat (mut first: Vec<PhraseNote>, second: Vec<PhraseNote>)-> Vec<PhraseNote> {
  first.extend (second.into_iter());
  first
}

pub fn current_watcher() {
  let mut percussion_table = HashMap::new();
  let project_path = Path::new("../data_02");
  percussion_table.insert(30,40);
  codecophony::project::watch_phrases (&project_path, &mut |phrases, _changed| {
    
    let mut arising = phrases ["arising"].notes.clone();
    for note in arising.iter_mut() {
      if note.tags.contains ("melody") {
        let start = note.start;
        note.dilate(0.8, start);
        if !note.tags.contains ("first") {
          note.tags.insert (String::from_str ("weakened").unwrap());
        }
      }
    }
    
    let striking = phrases ["striking"].notes.clone();
    let first = concat (
      applied (arising.clone(), | note | {note.tags.remove ("next_phrase");}),
      applied (striking.clone(), |note| note.nudge (find_tag (&arising, "next_phrase").start))
    );
    let second = applied (first.clone(), |note| note.nudge (find_tag (&first, "next_phrase").start));
    
    let first_second = concat(first.clone(), second.clone());
    
    let mut notes: Vec<Box<Renderable<[Output; CHANNELS]> + Send>> = Vec::new();
    for note in first_second.iter() {
      let velocity = if note.tags.contains ("emphasis") {120} else if note.tags.contains ("weakened") {60} else {90};
      if note.tags.contains ("percussion") {
        let pitch = frequency_to_nearest_midi_pitch(note.frequency);
        let instrument = percussion_table.get(&pitch).cloned().unwrap_or(35);
        notes.push(Box::new(MIDIPercussionNote::new (note.start, note.end - note.start, velocity, instrument)));
      }
      else {
        let instrument = if note.tags.contains ("melody") {57} else {43};
        notes.push(Box::new(MIDIPitchedNote::new (note.start, note.end - note.start, frequency_to_nearest_midi_pitch (note.frequency), velocity, instrument)));
      }
    }
    
    codecophony::project::write_phrase (&project_path, "output", &Phrase {notes: first_second.clone()});
    codecophony::project::set_playback_data (&project_path, SAMPLE_HZ, Some(Box::new(notes)));
  });
}


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
  
  /*let beats: f64 = 4.0;
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
    if instrument == 58 || instrument == 71 || instrument == 72 || instrument == 78 || instrument == 79 {continue;} and
    let &beat = generator.choose (& beat_weights).unwrap();
    let &(step, phase) = generator.choose (& step_weights).unwrap();
    let mut time = beat+beats*phase;
    while time < 60.0 {
      notes.push (
        MIDIPercussionNote::new(time/4.0, 1.0, 50, instrument)
      );
      time += step*beats;
    }
  }*/
  
  /*
  let mut generator = rand::chacha::ChaChaRng::from_seed(&[35]);
  let mut freq = 220.0;
  let timeadvance = 0.2;
  let harmonics = vec![
    3.0,5.0,7.0//,9.0,11.0,13.0
  ];
  let mut notes: Vec<_> = (0..1000u32).map(|index| {
    let factor = generator.choose(&harmonics).unwrap();
    if generator.gen() {
      freq *= factor;
    }
    else {
      freq /= factor;
    }
    while freq < 220.0/(1.0f64 + generator.gen::<f64>() * 5.0f64) { freq *= 2.0; }
    while freq > 220.0*(1.0f64 + generator.gen::<f64>() * 5.0f64) { freq /= 2.0; }
    let mut amplitude = timeadvance*0.2*220.0/freq;
    if amplitude > timeadvance*0.5 { amplitude = timeadvance*0.5; } 
    codecophony::SineWave {
      start: index as f64 * timeadvance, duration:1.0,
      frequency: freq, amplitude,
    }
  }).collect();*/
  
  
  
  /*let mut generator = rand::chacha::ChaChaRng::from_seed(&[35]);
  let levels = 5;
  let patterns = vec![
    vec![0,2],
    vec![1,3],
    vec![0,1,2,3],
    vec![0,1],
    vec![2,3],
    vec![2],
  ];
  
  let mut notes: Vec<Box<Renderable<[Output; CHANNELS]> + Send>> = Vec::new();
  for instrument in 35..82 {
    if instrument == 58 || instrument == 71 || instrument == 72 || instrument == 78 || instrument == 79 {continue;}
    
    let mut my_patterns = Vec::new();
    for _ in 0..levels {my_patterns.push (generator.choose (& patterns).unwrap());}
    
    'whoops: for time in 0u32..(1<<(2*levels)) {
      for level in 0..levels {
        if my_patterns [level].iter().find (|a| **a==(time >> (2*level)) & 3).is_none() {
          continue 'whoops;
        }
      }
      notes.push (
        Box::new(MIDIPercussionNote::new(time as f64/4.0, 1.0, 50, instrument))
      );
    }
  }
  
  let mut melody_patterns = Vec::new();
  let harmonics = vec![
    3.0,5.0,7.0,
    1.0/3.0, 1.0/5.0, 1.0/7.0,
  ];
  
  let melody_levels = levels - 1;
  for _ in 0..melody_levels {melody_patterns.push ([
    *generator.choose (& harmonics).unwrap(),
    *generator.choose (& harmonics).unwrap(),
    *generator.choose (& harmonics).unwrap(),
    1.0,
  ]);}
  for time in 0u32..(1<<(2*melody_levels)) {
    for level in 0..melody_levels {
      if (time as usize >> level) & 3 == 0 {
        melody_patterns [level] = [
          *generator.choose (& harmonics).unwrap(),
          *generator.choose (& harmonics).unwrap(),
          *generator.choose (& harmonics).unwrap(),
          1.0,
        ];
      }
    }
    for level in 0..3 {
      let mut frequency = 220.0;
      for level2 in level..melody_levels {
        frequency *= melody_patterns [level2][(time as usize >> level2) & 3];
      }
      while frequency < 100.0*(3-level) as f64/(2.0) { frequency *= 2.0; }
      while frequency > 100.0*(3-level) as f64*(2.0) { frequency /= 2.0; }
      let mut amplitude = 0.1*220.0/frequency;
      if amplitude > 0.25 { amplitude = 0.25; } 
      notes.push (
        Box::new(codecophony::SineWave { start: time as f64, duration: 1.05, frequency, amplitude})
      );
    }
  }*/
  
  
  let mut generator = rand::chacha::ChaChaRng::from_seed(&[35]);
  let notes = assemble_pattern (create_random_pattern ((1u32<<7) as f64, 1, &mut generator), 0.0);
  
  
  let phrases = vec![];// vec![Phrase::from_iter (notes.iter())];
  (Box::new(notes), phrases)
}


#[derive (Clone)]
struct Pattern {
  duration: f64,
  offset: f64,
  pattern_type: PatternType,
}

#[derive (Clone)]
enum PatternType {
  Assemblage (Vec<Pattern>),
  Notes (Rc<Fn(f64)->Vec<Box<Renderable<[Output; CHANNELS]> + Send>>>),
}

fn create_random_pattern (duration: f64, duplicates: i32, generator: &mut ChaChaRng)->Pattern {
  if generator.gen_range(0, 3) == 0 {
    Pattern {
      duration,
      offset: 0.0,
      pattern_type: PatternType::Assemblage (vec![create_random_pattern (duration, 2*duplicates, generator), create_random_pattern (duration, 2*duplicates, generator)]),
    }
  }
  else if duration*4.0 > generator.gen() {
    // long patterns must be constructed from sub-patterns
    if generator.gen() || duplicates == 1 {
      //repeating pattern
      let child = create_random_pattern (duration/2.0, duplicates, generator);
      let mut second_child = child.clone();
      second_child.offset += duration/2.0;
      Pattern {
        duration,
        offset: 0.0,
        pattern_type: PatternType::Assemblage (vec![child, second_child]),
      }
    }
    else {
      // offset pattern
      let mut child = create_random_pattern (duration/2.0, (duplicates+1)/2, generator);
      if generator.gen() { child.offset += duration/2.0; }
      child
    }
  }
  else {
    // short patterns are uhhh
    if generator.gen_range(0, 3) != 0 {
      let instrument = generator.gen_range(35, 83);
      Pattern {
        duration,
        offset: 0.0,
        pattern_type: PatternType::Notes (Rc::new(move |time| vec![Box::new(MIDIPercussionNote::new(time as f64, 1.0, 120/duplicates, instrument))])),
      }
    }
    else {
      let frequency: f64 = ((generator.gen::<f64>()*2f64-1f64)+(220f64).ln()).exp();
      let mut amplitude = 0.3*220.0/frequency/(duplicates as f64);
      if amplitude > 1.0/(duplicates as f64) { amplitude = 1.0/(duplicates as f64); } 
      Pattern {
        duration,
        offset: 0.0,
        pattern_type: PatternType::Notes (Rc::new(move |time| vec![Box::new(codecophony::SineWave { start: time, duration, frequency, amplitude})])),
      }
    }
  }
}

fn assemble_pattern (pattern: Pattern, offset: f64)->Vec<Box<Renderable<[Output; CHANNELS]> + Send>> {
  let mut result = Vec::new();
  match pattern.pattern_type {
    PatternType::Assemblage (patterns) => {
      for other_pattern in patterns {
        result.extend (assemble_pattern (other_pattern, offset + pattern.offset));
      }
    },
    PatternType::Notes (notes) => {
      return notes(offset + pattern.offset);
    }
  }
  result
}


