extern crate codecophony;
extern crate hound;
extern crate portaudio;
extern crate dsp;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

use codecophony::*;
use dsp::sample::ToFrameSliceMut;
use dsp::Frame;
use rand::{Rng, SeedableRng};
use std::str::FromStr;
use std::sync::Mutex;

mod sandbox;

use sandbox::{SAMPLE_HZ, CHANNELS, Output};

lazy_static! {
  static ref GUI: Mutex<Option<codecophony::rendering_gui::RenderingGui>> = Mutex::new(None);
}

pub fn push_gui_input (json_string: String) {
  let mut guard = GUI.lock().unwrap();
  let input = serde_json::from_str (& json_string).unwrap();
  if let Some(gui) = guard.as_mut() {
    gui.apply_gui_input (&input);
  }
  match input {
    _=>(),
  }
}

pub fn poll_updates ()->String {
  let mut guard = GUI.lock().unwrap();
  let mut new_phrases = None;
  if guard.is_none() {
    let gui = codecophony::rendering_gui::RenderingGui::new(SAMPLE_HZ);
    let (notes, phrases) = sandbox::current_playground();
    new_phrases = Some (phrases);
    gui.set_playback_range (((SAMPLE_HZ*notes.start()) as FrameTime, (SAMPLE_HZ*notes.end()) as FrameTime + 1));
    gui.set_playback_data (Some(notes));
    *guard = Some(gui);
    
  }
  let gui = guard.as_ref().unwrap();
  let mut updates = gui.gui_updates();
  if let Some(phrases) = new_phrases {
    updates.push (codecophony::rendering_gui::GuiUpdate::ReplacePhrases (phrases)) ;
  }
  
  serde_json::to_string (& updates).unwrap()
}
