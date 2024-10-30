use std::collections::HashSet;
use std::ops::Rem;
use std::slice::Iter;

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
        let position = EnigmaAlphabet::A;
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
        // self.position = EnigmaAlphabet::from((self.position as u8 + 1) % 26);
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
        let shift = self.position as u8;
        let shifted_input = (input.into() as u8 + shift).rem(26);
        (self.wiring.ab_wire(shifted_input) as u8 + 26 - shift).rem(26).into()
    }
}

impl Decoder for Rotor {
    type Letter = EnigmaAlphabet;

    fn decode<I>(&self, input: I) -> Self::Letter
    where
        I: Into<Self::Letter>,
    {
        let shift = self.position as u8;
        let shifted_input = (input.into() as u8 + shift).rem(26);
        (self.wiring.ba_wire(shifted_input) as u8 + 26 - shift).rem(26).into()
    }
}

impl<A, B, C> From<(A, B, C)> for Rotor
where
    A: Into<EnigmaAlphabet>,
    B: IntoIterator<Item = EnigmaAlphabet>,
    C: Into<Wiring>,
{
    fn from(value: (A, B, C)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::engine::Enigma;

    #[test]
    fn test_rotor() {
        use EnigmaAlphabet::*;
        let rotor = Rotor::new(
            A,
            [A],
            [B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A],
        );
        let encoded = rotor.encode(A);
        assert_eq!(encoded, B);
    }

    #[test]
    #[should_panic]
    fn test_rotor_invalid() {
        use EnigmaAlphabet::*;
        let _ = Rotor::new(
            A,
            [A],
            [A, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, A, A],
        );
    }

    #[test]
    fn test_rotor_from_tuple() {
        use EnigmaAlphabet::*;
        let rotor: Rotor = Rotor::new(
            'A',
            [A],
            [
                0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25,
            ],
        );
        assert_eq!(rotor.position, A);
    }
}
