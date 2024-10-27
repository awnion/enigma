use crate::core::alphabet::EnigmaAlphabet;
use crate::core::encoder::Encoder;

pub struct Plugboard<const N: usize = 10> {
    pairs: [(EnigmaAlphabet, EnigmaAlphabet); N],
}

impl<const N: usize> Plugboard<N> {
    /// Create a new Plugboard with the given pairs of letters.
    ///
    /// # Panics
    ///
    /// Panics if any letter appears twice in the pairs.
    pub fn new(pairs: [(EnigmaAlphabet, EnigmaAlphabet); N]) -> Self {
        let mut letter_set = std::collections::HashSet::new();
        for (a, b) in pairs.iter() {
            assert!(!letter_set.contains(a), "{:?} appears twice in pairs", a);
            assert!(!letter_set.contains(b), "{:?} appears twice in pairs", b);
            letter_set.insert(a);
            letter_set.insert(b);
        }
        Self { pairs }
    }
}

impl<const N: usize> Encoder for Plugboard<N> {
    type Letter = EnigmaAlphabet;

    fn encode(&self, input: Self::Letter) -> Self::Letter {
        for (a, b) in self.pairs.iter() {
            if input == *a {
                return *b;
            }
            if input == *b {
                return *a;
            }
        }
        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugboard() {
        use EnigmaAlphabet::*;
        let plugboard = Plugboard::new([(A, B), (C, D)]);
        let encoded = plugboard.encode(A);
        assert_eq!(encoded, B);
    }

    #[test]
    #[should_panic]
    fn test_plugboard_duplicate_pair() {
        use EnigmaAlphabet::*;
        let _ = Plugboard::new([(A, B), (B, C), (C, D)]);
    }
}
