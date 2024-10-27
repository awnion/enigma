use std::collections::HashSet;

use crate::core::alphabet::EnigmaAlphabet;
use crate::core::encoder::Encoder;

pub struct Reflector {
    wiring: [EnigmaAlphabet; 26],
}

impl Reflector {
    pub fn new(wiring: [EnigmaAlphabet; 26]) -> Self {
        let set: HashSet<EnigmaAlphabet> = wiring.iter().cloned().collect();
        assert!(set.len() == 26, "Wiring must contain 26 unique letters");
        Self { wiring }
    }
}

impl Encoder for Reflector {
    type Letter = EnigmaAlphabet;

    fn encode(&self, letter: EnigmaAlphabet) -> EnigmaAlphabet {
        EnigmaAlphabet::from(self.wiring[letter as usize] as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflector() {
        use EnigmaAlphabet::*;
        let plugboard = Reflector::new([
            B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A,
        ]);
        let encoded = plugboard.encode(A);
        assert_eq!(encoded, B);
    }

    #[test]
    #[should_panic]
    fn test_reflector_invalid() {
        use EnigmaAlphabet::*;
        let _ = Reflector::new([
            B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, A, A,
        ]);
    }
}
