use std::vec;

#[allow(dead_code)]
pub enum Notes {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}


impl Notes {
    pub fn freq(&self) -> f32 {
        match self {
            Self::C => 261.63,
            Self::Cs => 277.18,
            Self::D => 293.66,
            Self::Ds => 311.13,
            Self::E => 329.63,
            Self::F => 349.23,
            Self::Fs => 369.99,
            Self::G => 392.00,
            Self::Gs => 415.30,
            Self::A => 440.00,
            Self::As =>466.16,
            Self::B => 493.88,
        }
    }

    pub fn scale() -> Vec<Notes> {
        vec![Self::C  ,
             Self::Cs ,
             Self::D  ,
             Self::Ds ,
             Self::E  ,
             Self::F  ,
             Self::Fs ,
             Self::G  ,
             Self::Gs ,
             Self::A  ,
             Self::As ,
             Self::B  ,
        ]
    }
}