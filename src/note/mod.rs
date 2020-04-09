mod key;
#[allow(clippy::module_inception)]
mod note;
mod octave;

pub use note::Note;
pub use octave::Octave;
pub use key::Key;