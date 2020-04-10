mod notes;

use crate::notes::{Key, Note, Octave};

pub fn create_middle_c_note() -> Note {
    Note::new(Key::C, Octave(4))
}
