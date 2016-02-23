extern crate codecophony;
extern crate hound;

use codecophony::*;

fn main() {
write_eggs ();
//write_palette ();
}

fn write_eggs () {
  let mut main_melody = scrawl_MIDI_notes(
                            "transpose 57 velocity 100 instrument 55
        0 step 1 5 7 step 2 7 step 1 5 7 0 0 5 7 7 7 5 7 step 2
        0 step 1 5 7 5 7 5 7 5 7 5 7 5 7 5 7 step 2
          0 step 1 5 7 step 2 7 step 1 5 7 0 0 5 7 7 7 5 7 step 2
          0 step 1 5 7 step 2 7 step 1 5 7 0 0 5 7 7 10 5 7 step 2 finish
");
for note in main_melody.iter_mut () {if note.basics.start % 4.0 == 2.0 {note.renderer.velocity = 127;}}
let chorus_harmony = scrawl_MIDI_notes ("transpose 45 instrument 31 velocity 40
at 2 0 and 4 and 7 and 12
at 18 5 and 9 and 12 and 17
at 34 0 and 4 and 7 and 12 
at 50 2 and 7 and 17 and 22 
at 64 finish ");
let chorus_beat_part = scrawl_MIDI_notes ("velocity 100 percussion 35 step 2 35 and 38 finish");
let mut chorus_beat = Notes::new ();
for offset in 0..16 {chorus_beat.add (& chorus_beat_part.translated (offset as f64*4.0));}
let mut prechorus_beat = Notes::new ();
for offset in 0..8 {prechorus_beat.add (& scrawl_MIDI_notes ("velocity 100 percussion 35 step 4 38 finish"));}

let segment_length = 64.0;

/*

I wrote this verse with a boring rhythm (it should really have dotted notes and stuff) but I'm going to leave the customization for when I actually sing it.


*/
let mut believe_melody = scrawl_MIDI_notes ("transpose 57 velocity 100 instrument 55

5 5 strong 7 2 2 0 2 2 5 7 strong 5 0 0 -2 0 advance 2
transpose 53 advance 1 strong 7 2 2 2 0 2 2 strong 5 0 0 0 0 -2 0 advance 2
transpose 57 -2 0 0 2 2 4 4 5 5 strong 7 0 0 -2 0 advance 3
transpose 53 -2 0 0 2 2 4 4 5 5 strong 7 0 0 -2 0 advance 3");

let standard_chorus_speed = 30.0/170.0;
let standard_chorus_length = segment_length*standard_chorus_speed;
let standard_prechorus_speed = 35.0/170.0;
let standard_prechorus_length = segment_length*standard_prechorus_speed;
let standard_verse_speed = 40.0/170.0;
let standard_verse_length = segment_length*standard_verse_speed;

  let standard_chorus = Notes::combining(&[main_melody.clone (), chorus_harmony.clone (), chorus_beat.clone ()])
                .scaled(standard_chorus_speed);
let standard_prechorus = Notes::combining (& [main_melody.clone (), chorus_harmony.clone (), prechorus_beat.clone ()]).scaled (standard_prechorus_speed);

let mut now = 0.0;
let opening_chorus_start = now;
let mut notes = Notes::new ();
notes.add (& standard_chorus.translated (opening_chorus_start));
now += standard_chorus_length;
let believe_start = now;
notes.add (& believe_melody.translated (believe_start).scaled (standard_verse_speed));
now += standard_verse_length;
notes.add (& standard_prechorus.translated (now));
now += standard_prechorus_length;
notes.add (& standard_chorus.translated (now));

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
