use std::collections::HashSet;
use std::ops::Sub;

use super::wiring::Wiring;
use crate::core::alphabet::EnigmaAlphabet;
use crate::core::decoder::Decoder;
use crate::core::encoder::Encoder;

#[derive(Debug, Clone)]
pub struct Rotor {
    wiring: Wiring,
    ring: u8,
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
        Self { wiring, ring: 0u8, turnover, position }
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
        let is_turnover = self.turnover.contains(&self.position);
        self.position = self.position + 1;
        is_turnover
    }

    pub fn set(&mut self, position: impl Into<EnigmaAlphabet>) {
        self.position = position.into();
    }

    pub fn set_ring(&mut self, ring: u8) {
        self.ring = ring;
    }
}

impl Encoder for Rotor {
    type Letter = EnigmaAlphabet;

    fn encode<I>(&self, input: I) -> Self::Letter
    where
        I: Into<Self::Letter>,
    {
        let shift = self.position.sub(self.ring).to_u8();
        let input = input.into();
        let v = self.wiring.left_to_right_wire(input + shift) - shift;
        // eprintln!(
        //     "encode {} ({:>2}) -> {} -> {}",
        //     self.position.to_char(),
        //     self.position.to_u8(),
        //     input.to_char(),
        //     v.to_char()
        // );
        v
    }
}

impl Decoder for Rotor {
    type Letter = EnigmaAlphabet;

    fn decode<I>(&self, input: I) -> Self::Letter
    where
        I: Into<Self::Letter>,
    {
        let shift = self.position.sub(self.ring).to_u8();
        let input = input.into();
        let v = self.wiring.right_to_left_wire(input + shift) - shift;
        // eprintln!(
        //     "Decode {} ({:>2}) -> {} -> {}",
        //     self.position.to_char(),
        //     self.position.to_u8(),
        //     input.to_char(),
        //     v.to_char()
        // );
        v
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
        let mut rotor = Rotor::new("BCDEFGHIJKLMNOPQRSTUVWXYZA", "A".chars().map(Into::into));
        let encoded = rotor.encode('B');
        assert_eq!(encoded, 'C'.into());

        rotor.turn();

        let encoded = rotor.encode('B');
        assert_eq!(encoded, 'C'.into());

        rotor.turn();

        let decoded = rotor.decode('A');
        assert_eq!(decoded, 'Z'.into());
    }

    #[test]
    fn test_rotor_i() {
        let mut rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "Q".chars().map(Into::into));

        assert_eq!(rotor.decode('B'), 'W'.into());

        rotor.turn();

        assert_eq!(rotor.decode('B'), 'X'.into());
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
        assert_eq!(rotor.position, 'A'.into());
    }
}
