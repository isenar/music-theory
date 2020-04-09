use crate::note::key::Key;
use crate::note::octave::Octave;

#[derive(Debug)]
pub struct Note {
    pub octave: Octave,
    pub key: Key,
}
