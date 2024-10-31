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

    fn encode<I>(&self, input: I) -> Self::Letter
    where
        I: Into<Self::Letter>,
    {
        let input = input.into();
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
        let plugboard = Plugboard::new([(0u8.into(), 1u8.into()), (2.into(), 3.into())]);

        let encoded = plugboard.encode(0);
        assert_eq!(encoded, 1.into());
    }

    #[test]
    #[should_panic]
    fn test_plugboard_duplicate_pair() {
        let _ =
            Plugboard::new([(0u8.into(), 1u8.into()), (1.into(), 2.into()), (2.into(), 3.into())]);
    }
}
