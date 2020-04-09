mod note;

use crate::note::{Note, Octave, Key};

pub fn create_note() -> Note {
    Note {
        octave: Octave(1),
        key: Key::C,
    }
}

