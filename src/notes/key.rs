use strum_macros::{Display, EnumString};

#[derive(Display, Debug, EnumString, Copy, Clone, Eq, PartialEq)]
pub enum Key {
    C,
    #[strum(to_string = "C#")]
    CSharp,
    D,
    #[strum(to_string = "D#")]
    DSharp,
    E,
    F,
    #[strum(to_string = "F#")]
    FSharp,
    G,
    #[strum(to_string = "G#")]
    GSharp,
    A,
    #[strum(to_string = "A#")]
    ASharp,
    B,
}

impl Key {
    pub fn next(self) -> Self {
        use Key::*;

        match self {
            C => CSharp,
            CSharp => D,
            D => DSharp,
            DSharp => E,
            E => F,
            F => FSharp,
            FSharp => G,
            G => GSharp,
            GSharp => A,
            A => ASharp,
            ASharp => B,
            B => C,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Key::C => Key::CSharp ; "C to C# (white and black keys)")]
    #[test_case(Key::E => Key::F      ; "E to F (white keys)")]
    #[test_case(Key::B => Key::C      ; "'edge case' with wrapping the key value")]
    fn next_key(key: Key) -> Key {
        key.next()
    }
}
