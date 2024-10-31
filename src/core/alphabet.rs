use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Rem;
use std::ops::Sub;
use std::ops::SubAssign;

/// Alphabet for our Enigma machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnigmaAlphabet(u8);

impl EnigmaAlphabet {
    #[inline]
    pub fn to_u8(&self) -> u8 {
        self.0
    }

    #[inline]
    pub fn to_char(&self) -> char {
        (self.to_u8() + b'A') as char
    }
}

macro_rules! into_int_impl {
    ($($t:ty)*) => ($(
        impl From<EnigmaAlphabet> for $t {
            #[inline]
            fn from(value: EnigmaAlphabet) -> Self {
                value.0 as $t
            }
        }
    )*)
}

into_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

impl From<EnigmaAlphabet> for char {
    #[inline]
    fn from(value: EnigmaAlphabet) -> Self {
        value.to_char()
    }
}

impl From<EnigmaAlphabet> for String {
    #[inline]
    fn from(value: EnigmaAlphabet) -> Self {
        value.to_char().to_string()
    }
}

impl From<&u8> for EnigmaAlphabet {
    fn from(value: &u8) -> Self {
        match value {
            0..=25 => EnigmaAlphabet(*value),
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
            'A'..='Z' => (value as u8 - b'A').into(),
            _ => panic!("Invalid letter value: {}", value),
        }
    }
}

impl Add<u8> for EnigmaAlphabet {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        self.0.add(rhs).rem(26).into()
    }
}

impl AddAssign<u8> for EnigmaAlphabet {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs
    }
}

impl Sub<u8> for EnigmaAlphabet {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        (self.0 as i32).sub(rhs as i32).rem(26).into()
    }
}

impl SubAssign<u8> for EnigmaAlphabet {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs
    }
}
