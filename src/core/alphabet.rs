use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Rem;
use std::ops::Sub;

/// Alphabet for our Enigma machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EnigmaAlphabet {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

macro_rules! impl_enigma_alphabet_ints {
    ($($t:ty)*) => ($(
        impl From<EnigmaAlphabet> for $t {
            #[inline]
            fn from(value: EnigmaAlphabet) -> Self {
                value as $t
            }
        }
    )*)
}

impl_enigma_alphabet_ints! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

impl From<EnigmaAlphabet> for char {
    fn from(value: EnigmaAlphabet) -> Self {
        (value as u32 + b'A' as u32) as u8 as char
    }
}

impl From<&u8> for EnigmaAlphabet {
    fn from(value: &u8) -> Self {
        match value {
            0 => EnigmaAlphabet::A,
            1 => EnigmaAlphabet::B,
            2 => EnigmaAlphabet::C,
            3 => EnigmaAlphabet::D,
            4 => EnigmaAlphabet::E,
            5 => EnigmaAlphabet::F,
            6 => EnigmaAlphabet::G,
            7 => EnigmaAlphabet::H,
            8 => EnigmaAlphabet::I,
            9 => EnigmaAlphabet::J,
            10 => EnigmaAlphabet::K,
            11 => EnigmaAlphabet::L,
            12 => EnigmaAlphabet::M,
            13 => EnigmaAlphabet::N,
            14 => EnigmaAlphabet::O,
            15 => EnigmaAlphabet::P,
            16 => EnigmaAlphabet::Q,
            17 => EnigmaAlphabet::R,
            18 => EnigmaAlphabet::S,
            19 => EnigmaAlphabet::T,
            20 => EnigmaAlphabet::U,
            21 => EnigmaAlphabet::V,
            22 => EnigmaAlphabet::W,
            23 => EnigmaAlphabet::X,
            24 => EnigmaAlphabet::Y,
            25 => EnigmaAlphabet::Z,
            _ => panic!("Invalid letter value: {}", value),
        }
    }
}

impl From<u8> for EnigmaAlphabet {
    fn from(value: u8) -> Self {
        value.into()
    }
}

impl From<&u32> for EnigmaAlphabet {
    fn from(value: &u32) -> Self {
        match value {
            0..=25 => value.into(),
            _ => panic!("Invalid letter value: {}", value),
        }
    }
}

impl From<i32> for EnigmaAlphabet {
    fn from(value: i32) -> Self {
        match value {
            0..=25 => (value as u8).into(),
            _ => panic!("Invalid letter value: {}", value),
        }
    }
}

impl From<char> for EnigmaAlphabet {
    fn from(value: char) -> Self {
        match value {
            'A'..='Z' => EnigmaAlphabet::from(u32::from(value) - b'A' as u32),
            _ => panic!("Invalid letter value: {}", value),
        }
    }
}
