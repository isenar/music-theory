use crate::notes::key::Key;
use crate::notes::octave::Octave;

#[derive(Debug, Eq, PartialEq)]
pub struct Note {
    key: Key,
    octave: Octave,
}

impl Note {
    pub fn new(key: impl Into<Key>, octave: Octave) -> Self {
        Self {
            key: key.into(),
            octave,
        }
    }

    pub fn next(&self) -> Self {
        let next_key = self.key.next();
        let octave = match next_key {
            Key::C => self.octave + Octave(1),
            _ => self.octave,
        };

        Self {
            key: next_key,
            octave,
        }
    }

    pub fn key(&self) -> Key {
        self.key
    }

    pub fn octave(&self) -> Octave {
        self.octave
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_octave_when_going_from_b_to_c() {
        let b4_note = Note::new(Key::B, Octave(4));
        let expected = Note::new(Key::C, Octave(5));

        assert_eq!(expected, b4_note.next());
    }
}
