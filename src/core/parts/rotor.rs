use std::collections::HashSet;

use crate::core::alphabet::EnigmaAlphabet;
use crate::core::encoder::Encoder;

pub struct Rotor {
    position: EnigmaAlphabet,
    wiring: [EnigmaAlphabet; 26],
}

impl Rotor {
    pub fn new(position: EnigmaAlphabet, wiring: [EnigmaAlphabet; 26]) -> Self {
        let set: HashSet<EnigmaAlphabet> = wiring.iter().cloned().collect();
        assert!(set.len() == 26, "Wiring must contain 26 unique letters");
        Self { position, wiring }
    }
}

impl Encoder for Rotor {
    type Letter = EnigmaAlphabet;

    fn encode(&self, letter: EnigmaAlphabet) -> EnigmaAlphabet {
        EnigmaAlphabet::from((self.wiring[letter as usize] as u8 + self.position as u8 + 1) % 26)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugboard() {
        use EnigmaAlphabet::*;
        let plugboard = Rotor::new(
            A,
            [B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A],
        );
        let encoded = plugboard.encode(A);
        assert_eq!(encoded, C);
    }

    #[test]
    #[should_panic]
    fn test_rotor_invalid() {
        use EnigmaAlphabet::*;
        let _ = Rotor::new(
            A,
            [A, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, A, A],
        );
    }
}
