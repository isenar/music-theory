mod notes;

use crate::notes::{Key, Note, Octave};

pub fn middle_c_note() -> Note {
    Note::new(Key::C, Octave(4))
}
