#[derive(Debug)]
pub enum Key {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl Key {
    pub fn next(&self) -> Self {
        use Key::*;

        match *self {
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
