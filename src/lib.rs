extern crate codecophony;
extern crate hound;
extern crate portaudio;
extern crate dsp;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use codecophony::*;
use dsp::sample::ToFrameSliceMut;
use dsp::Frame;
use rand::{Rng, SeedableRng};
use std::str::FromStr;

#[derive (Serialize)]
struct Hack {
  value: String,
}

pub fn set_playback_range (json_string: String) {
  
}

pub fn poll_rendered ()->String {
  serde_json::to_string (& Hack {value: String::from_str ("hello from `songs`").unwrap()}).unwrap()
}
