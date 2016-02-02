extern crate codecophony;
extern crate hound;

use codecophony::*;

fn main() {
write_eggs ();
//write_palette ();
}

fn write_eggs () {
  let main_melody = scrawl_MIDI_notes(
                            "transpose 57 velocity 100 instrument 55
        0 step 1 5 7 step 2 7 step 1 5 7 0 0 5 7 7 7 5 7 step 2
        0 step 1 5 7 5 7 5 7 5 7 5 7 5 7 5 7 step 2
          0 step 1 5 7 step 2 7 step 1 5 7 0 0 5 7 7 7 5 7 step 2
          0 step 1 5 7 step 2 7 step 1 5 7 0 0 5 7 7 10 5 7 step 2 finish
");
let chorus_harmony = scrawl_MIDI_notes ("transpose 45 instrument 31
advance 2 0 and 4 and 7 and 12 at 18 0 and 4 and 7 and 12 at 26 4 and 12 and 19 at 34 0 and 4 and 7 and 12 at 50 0 and 4 and 7 and 12 at 58 2 and 7 and 17 and 22 at 64 finish ");
let chorus_beat_part = scrawl_MIDI_notes ("velocity 100 percussion 35 step 2 35 and 38 finish");
let mut chorus_beat = Notes::new ();
for offset in 0..16 {chorus_beat.add (& chorus_beat_part.translated (offset as f64*4.0));}

  let notes = Notes::combining(&[main_melody.clone (), chorus_harmony.clone (), chorus_beat.clone ()])
                .scaled(30.0/170.0);



  let music = notes.render_default(44100);

  let spec = hound::WavSpec {
    channels: 1,
    sample_rate: 44100,
    bits_per_sample: 16,
  };
  let mut writer = hound::WavWriter::create("eggs.wav", spec).unwrap();
  for t in music.samples.iter() {
    writer.write_sample(*t as i16).unwrap();

  }
}

fn write_palette () {
let mut notes = Notes::new ();
for offset in 1..129 {
notes.add (& scrawl_MIDI_notes (& ("instrument ".to_string () + & offset.to_string () + & " 64".to_string ())).translated ((offset + (offset -1)/4) as f64));
}
for offset in 35.. 82 {
notes.add (& scrawl_MIDI_notes (& (" percussion ".to_string () + & offset.to_string ())).translated ((180+ offset + (offset - 35)/4) as f64));
} 

  let music = notes.render_default(44100);

  let spec = hound::WavSpec {
    channels: 1,
    sample_rate: 44100,
    bits_per_sample: 16,
  };
  let mut writer = hound::WavWriter::create("palette.wav", spec).unwrap();
  for t in music.samples.iter() {
    writer.write_sample(*t as i16).unwrap();

  }

}
