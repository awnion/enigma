use std::collections::HashSet;

use super::wiring::Wiring;
use crate::core::alphabet::EnigmaAlphabet;
use crate::core::decoder::Decoder;
use crate::core::encoder::Encoder;

#[derive(Debug, Clone)]
pub struct Rotor {
    wiring: Wiring,
    turnover: HashSet<EnigmaAlphabet>,
    position: EnigmaAlphabet,
}

impl Rotor {
    pub fn new(
        wiring: impl Into<Wiring>,
        trunover: impl IntoIterator<Item = EnigmaAlphabet>,
    ) -> Self {
        let wiring = wiring.into();
        let turnover = trunover.into_iter().collect();
        let position = 0.into();
        Self { wiring, turnover, position }
    }

    pub fn with_position(
        wiring: impl Into<Wiring>,
        trunover: impl IntoIterator<Item = EnigmaAlphabet>,
        position: impl Into<EnigmaAlphabet>,
    ) -> Self {
        let mut rotor = Self::new(wiring, trunover);
        rotor.set(position);
        rotor
    }

    pub fn turn(&mut self) -> bool {
        self.position = self.position + 1;
        self.turnover.contains(&self.position)
    }

    pub fn set(&mut self, position: impl Into<EnigmaAlphabet>) {
        self.position = position.into();
    }
}

impl Encoder for Rotor {
    type Letter = EnigmaAlphabet;

    fn encode<I>(&self, input: I) -> Self::Letter
    where
        I: Into<Self::Letter>,
    {
        let shift = self.position.to_u8();
        self.wiring.ab_wire(input.into() + shift) - shift
    }
}

impl Decoder for Rotor {
    type Letter = EnigmaAlphabet;

    fn decode<I>(&self, input: I) -> Self::Letter
    where
        I: Into<Self::Letter>,
    {
        let shift = self.position.to_u8();
        self.wiring.ba_wire(input.into() - shift) + shift
    }
}

// impl<A, B, C> From<(A, B, C)> for Rotor
// where
//     A: Into<EnigmaAlphabet>,
//     B: IntoIterator<Item = EnigmaAlphabet>,
//     C: Into<Wiring>,
// {
//     fn from(value: (A, B, C)) -> Self {
//         Self::new(value.0, value.1, value.2)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotor() {
        let rotor = Rotor::new("BCDEFGHIJKLMNOPQRSTUVWXYZA", "A".chars().map(Into::into));
        let encoded = rotor.encode('A');
        assert_eq!(encoded, 'B'.into());
    }

    #[test]
    #[should_panic]
    fn test_rotor_invalid() {
        let _ = Rotor::new("ABCDEFGHIJKLMNOPQRSTUVWXYAA", "A".chars().map(Into::into));
    }

    #[test]
    fn test_rotor_from_tuple() {
        let rotor: Rotor = Rotor::new(
            [
                0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25,
            ],
            "A".chars().map(Into::into),
        );
        assert_eq!(rotor.position, b'A'.into());
    }
}
